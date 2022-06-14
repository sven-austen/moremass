/*
 * Canvas Implementation that displays a collection of Datasets.
 * 
 */

use iced::{

  pure::{
    widget::canvas::event::{self, Event},
    widget::canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke},
    
    column, Element,
  },
  
  mouse, Length, Point, Rectangle, Color, Size, Vector,
  Alignment
};

use crate::backend::{ Data, Dataset };
use crate::{ Message };

const COLORS: [Color; 3] = [
  Color { r: 0.169, g: 0.302, b: 0.455, a: 1.0 }, // blue
  Color { r: 0.824, g: 0.329, b: 0.4  , a: 1.0 }, // red
  Color { r: 0.651, g: 0.255, b: 0.523, a: 1.0 }, // purple
];

const RULER_GIRTH: f32 = 10.0;
const PAD:         f32 = 20.0;
const SCALE_X:     f32 =  5.0;

#[derive(Default)]
pub struct State {
//  selection: Option<Selection>,
  cache: canvas::Cache,
  pub x0: f32,
  pub y0: f32,
  pub l_click: Option<Point>,
  pub r_click: Option<Point>,
}

#[derive(Debug, Clone)]
pub enum CanvasMsg {
  LClick(Point),
  RClick(Point),
  MouseUp,
  MoveTo(f32, f32, f32, f32),
  Noop,
}

impl State {
  pub fn view<'a>(
    &'a self,
    data: &'a Data
  ) -> Element<'a, CanvasMsg> {

    column().spacing(10).align_items(Alignment::Center)
      .push(
        Canvas::new(Spectrum {
          state: self,
          data,
        })
        .height(Length::Fill)
        .width( Length::Units(f64::max(0.0, data.x_max_g - data.x_min_g) as u16) )
      )
      .into()
  }
  
  pub fn req_redraw(&mut self) {
    self.cache.clear()
  }
}

pub struct Spectrum<'a> {
  state: &'a State,
  data: &'a Data
}

impl<'a> canvas::Program<CanvasMsg> for Spectrum<'a> {
  type State = State;

  fn update(
    &self,
    state: &mut State,
    event: Event,
    bounds: Rectangle,
    cursor: Cursor
  ) -> (event::Status, Option<CanvasMsg>) {

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
            CanvasMsg::LClick(cursor_position)
          }
          
          mouse::Event::CursorMoved {position: Point {x: x1, y: y1}} => {
            CanvasMsg::MoveTo(x1, y1, SCALE_X, 1.0)
          }
          
          mouse::Event::ButtonReleased(mouse::Button::Left) => {
            CanvasMsg::MouseUp
          }
          
          _ => CanvasMsg::Noop
        };
        
        (event::Status::Captured, Some(message))
      }
      _ => (event::Status::Ignored, None)
    }
  }
  
  fn draw(
    &self, 
    state: &State, 
    bounds: Rectangle, 
    _cursor: Cursor
  ) -> Vec<Geometry> {
  
    let scale_y = (bounds.height - RULER_GIRTH - 2.0*PAD) / self.data.y_max_g as f32;
    
    let adj_point = |(x, y)| {
      Point {
        x: (x /*- state.x0*/ as f64) as f32 *SCALE_X + PAD + RULER_GIRTH,
        y: (self.data.y_max_g - y as f64) as f32 *scale_y + PAD
      }
    };
    
    vec![
      self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
      
        // 
        frame.stroke(&Path::rectangle(Point::ORIGIN, frame.size()), Stroke::default().with_width(2.0));
      
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
        frame.translate(Vector { x: -self.state.x0*SCALE_X, y: 0.0 });
        
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
        
        
      
      })
    ]
  }
}
