/*
 * Canvas Implementation that displays a collection of Datasets.
 * 
 */

use iced::{
  canvas::event::{self, Event},
  canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke},
  mouse, Element, Length, Point, Rectangle, Color, Size,
  Column, Alignment
};

use crate::backend::{ Data, Dataset };

const COLORS: [Color; 3] = [
  Color { r: 0.169, g: 0.302, b: 0.455, a: 1.0 }, // blue
  Color { r: 0.824, g: 0.329, b: 0.4  , a: 1.0 }, // red
  Color { r: 0.651, g: 0.255, b: 0.523, a: 1.0 }, // purple
];

const RULER_GIRTH: f32 = 10.0;
const PAD:         f32 = 20.0;
const SCALE_X:     f32 =  5.0;//0.3;

#[derive(Default)]
pub struct State {
//  selection: Option<Selection>,
  cache: canvas::Cache,
  pub x0: f32,
  pub y0: f32,
  pub l_click: Option<Point>,
  pub r_click: Option<Point>,
}

impl State {
  pub fn view<'a>(
    &'a mut self,
    data: &'a Data
  ) -> Element<'a, ()> {

    Column::new().spacing(10).align_items(Alignment::Center)
      .push::<Element<'a, ()>>(
        Canvas::new(Spectrum {
          state: self,
          data,
        })
        .height(Length::Fill)
        .width( Length::Units(f64::max(0.0, data.x_max_g - data.x_min_g) as u16) )
        .into()
      )
      .into()
  }
  
  pub fn req_redraw(&mut self) {
    self.cache.clear()
  }
}

pub struct Spectrum<'a> {
  state: &'a mut State,
  data: &'a Data
}

impl<'a> canvas::Program<()> for Spectrum<'a> {
  fn update(
    &mut self,
    event: Event,
    bounds: Rectangle,
    cursor: Cursor
  ) -> (event::Status, Option<()>) {

    let cursor_position =
      if let Some(position) = cursor.position_in(&bounds) {
        position
      } else {
        return (event::Status::Ignored, None);
      };

    match event {
      Event::Mouse(mouse_event) => {
        let message = match mouse_event {
          mouse::Event::ButtonPressed(mouse::Button::Left) => {
            self.state.l_click = Some(cursor_position);
            None
          }
          
          mouse::Event::CursorMoved {..} => {
            if let Some(lcl) = self.state.l_click {
              self.state.x0 = self.state.x0 + (lcl.x - cursor_position.x) / SCALE_X;
              self.state.l_click = Some(cursor_position);
              self.state.req_redraw();
            }
            
            None
          }
          
          mouse::Event::ButtonReleased(mouse::Button::Left) => {
            self.state.l_click = None;
            
            None
          }
          
          _ => None
        };
        
        (event::Status::Captured, message)
      }
      _ => (event::Status::Ignored, None)
    }
  }
  
  fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
  
    let scale_y = (bounds.height - RULER_GIRTH - 2.0*PAD) / self.data.y_max_g as f32;
    
    let adj_point = |(x, y)| {
      Point {
        x: (x - self.state.x0 as f64) as f32 *SCALE_X + RULER_GIRTH + PAD,
        y: (self.data.y_max_g - y + self.state.y0 as f64) as f32 *scale_y + PAD
      }
    };
    
    vec![
      self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
      
        // RULERS ------------------------------------

        // vertical
        frame.fill_rectangle( 
          Point { x: PAD, y: PAD }, 
          Size  { width: RULER_GIRTH, height: bounds.height - RULER_GIRTH - 2.0*PAD }, 
          Color { r: 1.0, g: 0.0, b: 0.0, a: 0.5 } 
        );

        // horizontal
        frame.fill_rectangle(
          Point { x: PAD + RULER_GIRTH, y: bounds.height - PAD - RULER_GIRTH },
          Size  { width: bounds.width - RULER_GIRTH - PAD, height: RULER_GIRTH },
          Color { r: 0.0, g: 1.0, b: 0.0, a: 0.5 }
        );
      
        // PEAKS -------------------------------------
        let peaklists: Vec<&Vec<(f64, f64)>> = self.data.sets.iter().map(|ds| {
          let Dataset { peaks, .. } = ds; peaks
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
      
      })
    ]
  }
}
