/*
 * Canvas Implementation that displays a collection of Datasets.
 * 
 */

use iced::{

  pure::{
    widget::canvas::event::{self, Event},
    widget::canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke, Text,},
    
    column, Element,
  },
  
  mouse, keyboard, Length, Point, Rectangle, Color, Size,
  Alignment
};

use crate::Message;
use crate::backend::{ Data };

const COLORS: [Color; 3] = [
  Color { r: 0.169, g: 0.302, b: 0.455, a: 1.0 }, // blue
  Color { r: 0.824, g: 0.329, b: 0.4  , a: 1.0 }, // red
  Color { r: 0.651, g: 0.255, b: 0.523, a: 1.0 }, // purple
];

const RULER_GIRTH: f32 = 10.0;


pub struct State {
  cache: canvas::Cache,
  pub x0: f32,
  pub y0: f32,
  pub sx: f32,
  pub sy: f32,
  pub l_click: Option<Point>,
  pub r_click: Option<Point>,
  pub selection: (f32, f32), // only valid if rClick is not None
  pub modifiers: keyboard::Modifiers,
}

impl Default for State {
  fn default() -> Self {
    State {
      sx: 5.0,
      sy: 1.0,
      
      x0: 0.0,
      y0: 0.0,
      l_click: None,
      r_click: None,
      selection: (0.0, 0.0),
      cache: canvas::Cache::default(),
      modifiers: keyboard::Modifiers::default(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum PlotMsg {
  LClick(Point),
  RClick(Point),
  MouseUp(mouse::Button),
  MoveTo(f32, f32),
  Scaled(f32, (f32, f32)),
  ModifiersChanged(keyboard::Modifiers),
}

impl State {
  pub fn view<'a>(
    &'a self,
    data: &'a Data
  ) -> Element<'a, Message> {

    column().padding(20).spacing(10).align_items(Alignment::Center)
      .push(
        Canvas::new(Plot {
          state: self,
          data,
        })
        .height(Length::Fill)
        .width(Length::Fill)
      )
      .into()
  }
  
  pub fn update(&mut self, msg: PlotMsg) {
  
    match msg {
      PlotMsg::LClick(pt) => {
        self.l_click = Some(pt);
      }
      
      PlotMsg::RClick(pt) => {
        self.r_click = Some(pt);
        let Point {x, y: _} = pt;
        self.selection = (x, x);
      }
      
      // TODO make this smarter
      PlotMsg::MouseUp(btn) => {
        match btn {
          mouse::Button::Left => {
            self.l_click = None;
          }
          mouse::Button::Right => {
            self.r_click = None;
          }
          _ => {}
          
        }
      }
      
      PlotMsg::MoveTo(x1, y1) => {
        
        if let Some(Point {x, y}) = self.l_click {
        
          self.x0 += x - x1;
          self.y0 += y - y1;
            
          self.l_click = Some(
            Point { x: x1, y: y1 }
          );
        }
        
        if let Some(_) = self.r_click {
          
          let (from, _) = self.selection;
          self.selection = ( from, x1 );
          self.r_click = Some(
            Point { x: x1, y: y1 }
          );
        }
        
        self.req_redraw();
      }
      
      PlotMsg::Scaled(delta, (dx, _dy)) => {
        
        if self.modifiers.shift() {
          self.sy *= delta;
          //self.y0 += dy;
          //self.y0 *= delta;
        } else {
          self.sx *= delta;
          self.x0 += dx;
          self.x0 *= delta;
        }
        
        self.req_redraw();
        
      }
      
      PlotMsg::ModifiersChanged(mods) => {
        self.modifiers = mods;
      }
    }
  }
  
  pub fn rethink_bounds(&mut self, data: &Data) {
    if data.sets.len() == 1 {
      self.sx = 20.0;
      self.sy = 0.9;
      self.x0 = self.sx * data.x_min_g as f32;
      self.y0 = 0.0;
    }
  }
  
  pub fn req_redraw(&mut self) {
    self.cache.clear()
  }
}

pub struct Plot<'a> {
  state: &'a State,
  data: &'a Data
}

pub enum Interaction {
  None,
}

impl Default for Interaction {
  fn default() -> Self {
    Interaction::None
  }
}

impl<'a> canvas::Program<Message> for Plot<'a> {
  type State = Interaction;

  fn update(
    &self,
    _interaction: &mut Interaction,
    event: Event,
    bounds: Rectangle,
    cursor: Cursor
  ) -> (event::Status, Option<Message>) {
  
    // for mouseup, we dont care where the cursor is
    if let Event::Mouse(mouse::Event::ButtonReleased(btn)) = event {
      match btn {
        mouse::Button::Right => {
        
          if let Some(peak) = self.max_pt_in_highlight(&bounds) {
            return (
              event::Status::Captured, 
              Some(Message::AddPeak(peak))
            );
          } else {
            return (
              event::Status::Ignored, 
              Some(Message::ForPlot(PlotMsg::MouseUp(mouse::Button::Right)))
            );
          }
        
        }
        _ => { return (event::Status::Ignored, Some(Message::ForPlot(PlotMsg::MouseUp(btn)))); }
      }
    
    }

    let cursor_position =
      if let Some(position) = cursor.position_in(&bounds) {
        position
      } else {
        return (event::Status::Ignored, None);
      };

    match event {
      Event::Mouse(mouse_event) => {
        let message = match mouse_event {
        
          mouse::Event::ButtonPressed(btn) => {
            match btn {
              mouse::Button::Left => {
                Some(PlotMsg::LClick(cursor_position))
              }
              mouse::Button::Right => {
                Some(PlotMsg::RClick(cursor_position))
              }
              _ => None
            }
          }
          
          mouse::Event::CursorMoved {position: _} => {
            let Point {x: x1, y: y1} = cursor_position;
            Some(PlotMsg::MoveTo(x1, y1))
          }
          
          mouse::Event::WheelScrolled{ delta } => match delta {
            mouse::ScrollDelta::Lines  { y, .. } |
            mouse::ScrollDelta::Pixels { y, .. } => {
            
              let factor = 1.0 + y / 30.0;
              let translation = 
                if let Some(cursor_to_origin) = 
                  cursor.position_from(
                    Point {
                      x: bounds.x + RULER_GIRTH, 
                      y: bounds.y
                    }
                  ) {
                    
                    (
                      cursor_to_origin.x * (factor - 1.0),
                      (self.state.y0) 
                        * (factor - 1.0)//cursor_to_origin.y * (factor - 1.0)
                    )
                } else {
                  (0.0, 0.0)
                };
              
              Some(PlotMsg::Scaled( factor, translation ))
              
            }
          }
          
          _ => None
        };
        
        (event::Status::Captured, message.map(Message::ForPlot))
      } // </mouse events>
      
      Event::Keyboard(key_event) => {
        let message = match key_event {
          keyboard::Event::ModifiersChanged(modfs) => {
            Some(PlotMsg::ModifiersChanged(modfs))
          }
          _ => None
        };
        
        (event::Status::Captured, message.map(Message::ForPlot))
      }
    }
  }
  
  fn draw(
    &self, 
    _interaction: &Interaction, 
    big_bounds: Rectangle, 
    _cursor: Cursor
  ) -> Vec<Geometry> {
  
    let bounds = Rectangle {
      x: big_bounds.x + 20.0,
      y: big_bounds.y + 20.0,
      width:  big_bounds.width  - 40.0,
      height: big_bounds.height - 40.0
    };
    
    vec![
    
      self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
      
        // 
        frame.stroke(&Path::rectangle(Point::ORIGIN, frame.size()), Stroke::default().with_width(2.0));
        
        // baseline
        if self.state.y0 > RULER_GIRTH {
          let baseline = Path::new(|p| {
            p.move_to(Point { x: RULER_GIRTH,  y: bounds.height - self.state.y0 - RULER_GIRTH });
            p.line_to(Point { x: bounds.width, y: bounds.height - self.state.y0 - RULER_GIRTH });
          });
          frame.stroke(
            &baseline, 
            Stroke::default()
              .with_width(0.5)
              .with_color(Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0})
          );
        }

        // RULERS ------------------------------------

        // vertical
        frame.fill_rectangle( 
          Point { x: 0.0, y: 0.0 }, 
          Size  { width: RULER_GIRTH, height: bounds.height - RULER_GIRTH }, 
          Color { r: 1.0, g: 1.0, b: 1.0, a: 0.8 } 
        );

        // horizontal
        frame.fill_rectangle(
          Point { x: RULER_GIRTH, y: bounds.height - RULER_GIRTH },
          Size  { width: bounds.width - RULER_GIRTH, height: RULER_GIRTH },
          Color { r: 0.0, g: 1.0, b: 1.0, a: 0.8 }
        );
        
        // selection
        if let Some(_) = self.state.r_click {
          let (from, to) = self.state.selection;
          
          frame.fill_rectangle(
            Point { x: from, y: 0.0 },
            Size  { width: to - from, height: bounds.height - RULER_GIRTH },
            Color { r: 0.0, g: 0.0, b: 1.0, a: 0.5 }
          );
        }
      
        // PEAKS -------------------------------------
//        frame.translate(Vector { x: -self.state.x0, y: 0.0 });
        
        let mut i = 0;
        for ds in self.data.sets.iter().filter(|ds| {ds.visible}) {
          let points = &ds.points;
          let peaks  = &ds.peaks;
          
          if points.len() < 2 { continue; }
          
          let lower: f64 = ((self.state.x0 + RULER_GIRTH)/self.state.sx).into();
          let upper: f64 = ((self.state.x0 + RULER_GIRTH + bounds.width)/self.state.sx).into();
          
          // draw points 
          let curve = Path::new(|p| {
            let mut j = 0;
            while j < points.len()-1 && fst(points[j]) < lower {
              j += 1;
            }
            
            p.move_to(self.to_coords(points[j], &bounds));
            j += 1;
            
            while j < points.len() && fst(points[j]) < upper {
              p.line_to(self.to_coords(points[j], &bounds));
              j += 1;
            }
          });
          frame.stroke(&curve, Stroke::default().with_width(1.0).with_color(COLORS[i%3]));
          i += 1;
          
          // draw peaks
          for (xpk, ypk) in peaks {
            
            let Point {x: x_, y: y_} = self.to_coords((*xpk, *ypk), &bounds);
            
            let path = Path::new(|p| {
              p.move_to(Point {x: x_, y: bounds.height - RULER_GIRTH - self.state.y0});
              p.line_to(Point {x: x_, y: y_});
            });
            frame.stroke(
              &path, 
              Stroke::default()
                .with_width(1.0)
                .with_color(Color {r: 1.0, g: 0.0, b: 0.0, a: 1.0}));
            
            let txt = Text {
              content: format!("{:.1}", *ypk),
              position: Point {x: x_, y: y_ - 10.0},
              ..Text::default()
            };
            frame.fill_text(txt);
          }
        }
        
      })
    ]
  }
}

impl Plot<'_> {
  fn max_pt_in_highlight(&self, bounds: &Rectangle) -> Option<(f64, f64)> {
    
    if self.data.sets.len() > self.data.curr_ds {
      
      let (lower_c, upper_c) = self.state.selection;
      let (lower,   upper  ) = 
        ( fst(self.to_values(Point {x: lower_c, y: 0.0 }, &bounds))
        , fst(self.to_values(Point {x: upper_c, y: 0.0 }, &bounds)));
      let valid_pts: Vec<&(f64, f64)> = 
        self.data.sets[self.data.curr_ds].points.iter()
          .filter( |(x, _)| {x > &lower && x < &upper} )
          .collect();
      
      if valid_pts.len() == 0 {
        return None;
      }
      
      Some(valid_pts[1..].iter()
        .fold(*valid_pts[0], |(acc_x, acc_y), (x, y)| {
          if y > &acc_y {
            (*x, *y)
          } else {
            (acc_x, acc_y)
          }
        })
      )
      
    } else {
      None
    }
    
  }

  fn to_values(&self, Point {x, y}: Point, bounds: &Rectangle) -> (f64, f64) {
  
    
    let fac =
      (bounds.height - RULER_GIRTH) / self.data.y_max_g as f32;
    
    (
       ((x + self.state.x0) / self.state.sx) as f64,
      (((y + self.state.y0) / fac + self.data.y_max_g as f32) / self.state.sy) as f64
    )
    
  }

  fn to_coords(&self, (x, y): (f64, f64), bounds: &Rectangle) -> Point {
    
    let fac =
      (bounds.height - RULER_GIRTH) / self.data.y_max_g as f32;

    Point {
      x: x as f32 * self.state.sx - self.state.x0,
      y: (self.data.y_max_g as f32 - y as f32 * self.state.sy) * fac - self.state.y0
    }
    
  }
}

fn fst((val, _): (f64, f64)) -> f64 {
  val
}
