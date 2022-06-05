/*
 * Canvas Implementation that displays a collection of Datasets.
 * 
 */

use iced::{
  canvas::event::{self, Event},
  canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke},
  /*mouse, */Element, Length, Point, Rectangle, Color,
};

use crate::backend::{ Data, Dataset };

const COLORS: [Color; 3] = [
  Color { r: 0.169, g: 0.302, b: 0.455, a: 1.0 }, // blau
  Color { r: 0.824, g: 0.329, b: 0.4  , a: 1.0 }, // rot
  Color { r: 0.651, g: 0.255, b: 0.523, a: 1.0 }, // lila
];

#[derive(Default)]
pub struct State {
//    selection: Option<Selection>,
  cache: canvas::Cache,
}

impl State {
  pub fn view<'a>(
    &'a mut self,
    data: &'a Data
  ) -> Element<'a, ()> {
    Canvas::new(Thing {
      state: self,
      data,
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }
  
  pub fn req_redraw(&mut self) {
    self.cache.clear()
  }
}

pub struct Thing<'a> {
  state: &'a mut State,
  data: &'a Data
}

impl<'a> canvas::Program<()> for Thing<'a> {
  fn update(
    &mut self,
    _event: Event,
    _bounds: Rectangle,
    _cursor: Cursor
  ) -> (event::Status, Option<()>) {

    (event::Status::Ignored, None) // make this functional
  }
  
  fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
  
    let scale = (bounds.height as f64) / self.data.y_max_g;
    let adj_point = |(x, y)| {
      Point { 
        x: ((x - self.data.x_min_g)*0.3) as f32, 
        y: ((self.data.y_max_g - 6.0*y)/*y*/*scale) as f32 
      }
    };
    
    let content =
      self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
        let peaklists: Vec<&Vec<(f64, f64)>> = self.data.sets.iter().map(|ds| {
          let Dataset { peaks, .. } = ds;
          peaks
        }).collect();
        
        for i in 0..peaklists.len() {
          let peaklist = peaklists[i];
          if peaklist.len() < 2 { continue; }
          
          let curve = Path::new(|p| {
            p.move_to(adj_point(peaklist[0]));
            for peak in &peaklist[1..] {
              p.line_to(adj_point(*peak));
            }
          });
          frame.stroke(&curve, Stroke::default().with_width(0.5).with_color(COLORS[i%3]));
        }
        
        frame.stroke(&Path::rectangle(Point::ORIGIN, frame.size()), Stroke::default().with_width(2.0));
      });
    
    vec![content]
  }
}
