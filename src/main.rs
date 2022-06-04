
use iced::{
  button, Button, Alignment, Length, Settings, Point, Sandbox, Element, Column, Text, 
};

pub fn main() -> iced::Result {
  MoreMass::run(Settings {
    antialiasing: true,
    ..Settings::default()
  })
}

#[derive(Default)]
struct MoreMass {
  datasets:     Vec<backend::Dataset>,
  button_state: button::State,
  canvas_state: spectrum::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  Clear,
  Noop
}

impl Sandbox for MoreMass {
  type Message = Message;
  
  fn new() -> Self {
    MoreMass::default()
  }
    // backend::parser::parse_mzxml_badly("/home/sven/Documents/projects/mMass/data/20220516/Input_caColumn_1.mzXML".to_string())
  
  fn title(&self) -> String {
    String::from("MoreMass")
  }
  
  fn update(&mut self, message: Message) {
    match message {
      Message::Clear => {
        // TODO
      }
      Message::Noop => { }
    }
  }
  
  fn view(&mut self) -> Element<Message> {
    Column::new()
      .padding(20)
      .spacing(20)
      .align_items(Alignment::Center)
      .push( Text::new(self.title()).width(Length::Shrink).size(50) )
      .push(self.canvas_state.view(&self.datasets).map(|_| {Message::Noop}))
      .push( 
        Button::new(&mut self.button_state, Text::new("Clear"))
          .padding(8)
          .on_press(Message::Clear)
      )
      .into()
  }
}

// --------------------------------------------------------
mod spectrum {
  use iced::{
    canvas::event::{self, Event},
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke},
    /*mouse, */Element, Length, Point, Rectangle
  };
  
  use crate::backend::Dataset;

  #[derive(Default)]
  pub struct State {
//    selection: Option<Selection>,
    cache: canvas::Cache,
  }
  
  impl State {
    pub fn view<'a>(
      &'a mut self,
      datasets: &'a Vec<Dataset>
    ) -> Element<'a, ()> {
      Canvas::new(Thing {
        state: self,
        datasets,
      })
      .width(Length::Fill)
      .height(Length::Fill)
      .into()
    }
    
    pub fn _req_redraw(&mut self) {
      self.cache.clear()
    }
  }
  
  pub struct Thing<'a> {
    state: &'a mut State,
    datasets: &'a Vec<Dataset>
  }
  
  impl<'a> canvas::Program<()> for Thing<'a> {
    fn update(
      &mut self,
      event: Event,
      bounds: Rectangle,
      cursor: Cursor
    ) -> (event::Status, Option<()>) {
      
      (event::Status::Ignored, None) // make this functional
    }
    
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
      let global_max_y = self.datasets.iter()
        .fold(1f32, |acc, ds| {
          let &Dataset { y_max, .. } = &ds;
          if acc > *y_max { acc } else { *y_max }
        });
      let global_min_x = self.datasets.iter().fold(10000f32, |acc, ds| {
        let &Dataset {x_min, ..} = &ds;
        if acc < *x_min {acc} else {*x_min}
      });
      let scale = (bounds.height) / global_max_y;
      let adj_point = |Point { x, y }| {
        Point { x: (x-global_min_x)*0.3, y: (global_max_y-6.0*y)/*y*/*scale }
      };
      
      let content =
        self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
          let peaklists = self.datasets.iter().map(|ds| {
            let Dataset { peaks, .. } = ds;
            peaks
          });
          
          let curve = Path::new(|p| {
            for peaklist in peaklists {
              
              if peaklist.len() < 2 { continue; }
              
              p.move_to(adj_point(peaklist[0]));
              for peak in &peaklist[1..] {
                p.line_to(adj_point(*peak));
              }
            }
          });

          frame.stroke(&curve, Stroke::default().with_width(0.5));
          
          frame.stroke(&Path::rectangle(Point::ORIGIN, frame.size()), Stroke::default());
        });
      
      vec![content]
    }
  }
}

// --------------------------------------------------------
mod backend {
  use iced::Point;
  
  #[derive(Clone)]
  pub struct Dataset {
    pub author: String,
    pub peaks:  Vec<Point>,
    pub y_min:  f32,
    pub y_max:  f32,
    pub x_min:  f32,
    pub x_max:  f32,
  }

  mod parser {
    use crate::backend::Dataset;
    use std::fs::File;
    use std::path::Path;
    use std::io::{self, BufRead, Cursor, Read};
    use base64::decode_config;
    
    use iced::Point;
    
  //  use byteorder::{BigEndian, ReadBytesExt};

  //  const CNT_HEAD:   &str = "          peaksCount=\"";
    const PEAKS_HEAD: &str = "             contentType=\"m/z-int\">";
    const CONFIG: base64::Config = base64::Config::new(base64::CharacterSet::Standard, true);

    pub fn parse_mzxml_badly( s: String ) -> Option<Dataset> {

      let path = Path::new(&s);
      let display = path.display();
      
      // open file
      let file = match File::open(&path) {
        Err(why) => {
          println!("Could not open {}: {}", display, why);
          return None;
        }
        Ok(file) => file
      };
      
      // iterate over lines
      for maybe_line in io::BufReader::new(file).lines() {
      
        let line = match maybe_line {
          Err(why) => {
            println!("Could not read line from data file: {:?}", why);
            return None;
          }
          Ok(l) => l
        };
        
        // read peak list
        if line.len() >= PEAKS_HEAD.len() && 
           line[..PEAKS_HEAD.len()] == *PEAKS_HEAD {
            
          let bytes = match decode_config(&line[PEAKS_HEAD.len() .. line.len()-8], CONFIG) {
            Err(why) => {
              println!("Could not decode peak list: {:?}", why);
              return None;
            }
            Ok(bs) => bs
          };
          
          let mut peaks: Vec<Point> = Vec::with_capacity(bytes.len() / 16);
          let mut buf:   [u8; 8]    = [0u8; 8];
          let mut cursor            = Cursor::new(&bytes);
          let mut x: f32;
          let mut y: f32;
          
          for _i in 0 .. (bytes.len() / 16) {
            match cursor.read(&mut buf) {
              Ok(n) => {
                if n != 8 {
                  break;
                }
                
                x = f64::from_be_bytes(buf) as f32;
              },
              Err(why) => {
                println!("Error occured while parsing data: {:?}", why);
                return None
              }
            }
            
            // read again for second value
            match cursor.read(&mut buf) {
              Ok(n) => {
                if n != 8 {
                  break;
                }
                
                y = f64::from_be_bytes(buf) as f32;
              },
              Err(why) => {
                println!("Error occured while parsing data: {:?}", why);
                return None
              }
            }
            peaks.push(Point {
              x: x,
              y: y,
            });
          }

          return Some(Dataset {
            author: "MariuuuUUUUus".to_string(),
            y_min:  peaks.iter().map(|&Point{y, ..}| {y}).fold(0.0f32,   |acc, v| if acc < v {acc} else {v}),
            y_max:  peaks.iter().map(|&Point{y, ..}| {y}).fold(0.0f32,   |acc, v| if acc > v {acc} else {v}),
            x_min:  peaks.iter().map(|&Point{x, ..}| {x}).fold(f32::MAX, |acc, v| if acc < v {acc} else {v}),
            x_max:  peaks.iter().map(|&Point{x, ..}| {x}).fold(f32::MAX, |acc, v| if acc > v {acc} else {v}),
            peaks:  peaks,
          });
        }
      }
      
      return None;
    }
  }
}
