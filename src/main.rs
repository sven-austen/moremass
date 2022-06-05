
use iced::{
  button, Length, Settings, Sandbox, Element, Column, Text, Row, Button, TextInput, text_input,
  Alignment, alignment::Horizontal,
};

pub fn main() -> iced::Result {
  MoreMass::run(Settings {
    antialiasing: true,
    ..Settings::default()
  })
}

#[derive(Default)]
struct MoreMass {
//  datasets:     Vec<backend::Dataset>,
  data:         backend::Data,
  controls:     Controls,
  file_path:    String,
  
  canvas_state: spectrum::State,
  popup:        Option<WhichPopup>,
}

#[derive(Default)]
struct Controls {
  btn_load:  button::State,
  btn_clear: button::State,
  txt_file:  text_input::State,
}

#[derive(Debug, Clone)]
pub enum Message {
  Popup( WhichPopup ),
  ChangeFilePath( String ),
  LoadFile,
  Clear,
  Noop
}

#[derive(Debug, Clone, Copy)]
pub enum WhichPopup {
  FindFile,
}

impl Sandbox for MoreMass {
  type Message = Message;
  
  fn new() -> Self {
    MoreMass::default()
  }

  fn title(&self) -> String {
    String::from("MoreMass")
  }
  
  fn update(&mut self, message: Message) {
    match message {
      Message::Popup(which) => {
        self.popup = Some(which);
      }
      Message::ChangeFilePath(s) => {
        self.file_path = s;
      }
      Message::LoadFile => {
/*        if let Some(d) = crate::backend::parser::parse_mzxml_badly(&self.file_path) {
          self.data.sets.push(d);
          self.data.x_min_g = self.data.x_min_g.min(d.x_min);
          self.data.x_max_g = self.data.x_max_g.max(d.x_max);
          self.data.y_min_g = self.data.y_min_g.min(d.y_min);
          self.data.y_max_g = self.data.y_max_g.max(d.y_max);
        }*/
        self.data.push(crate::backend::parser::parse_mzxml_badly(&self.file_path));
        self.file_path = "".to_string();
        self.popup = None;
        self.canvas_state._req_redraw();
      }
      Message::Clear => {
        self.data = crate::backend::Data::default();
        self.canvas_state._req_redraw();
      }
      Message::Noop => { }
    }
  }
  
  fn view(&mut self) -> Element<Message> {
  
    let left = Column::new().padding(20).spacing(20)
      .width(Length::FillPortion(1))
      .align_items(Horizontal::Center.into())
      .push(
        Button::new(
          &mut self.controls.btn_load, Text::new("Load file")
        ).on_press(Message::Popup(WhichPopup::FindFile))
      )
      .push(
        Button::new(
          &mut self.controls.btn_clear, Text::new("Clear")
        ).on_press(Message::Clear)
      );
    
    let center = Column::new().push(
      self.canvas_state.view(&self.data).map(|_| {Message::Noop})
    ).width(Length::FillPortion(5));
    
    let right: Element<Message> = 
      if let Some(which) = self.popup {
        match which {
        
          WhichPopup::FindFile => {
          
            let input = TextInput::new(
              &mut self.controls.txt_file,
              "File Path",
              &self.file_path,
              |s| { Message::ChangeFilePath(s) },
            ).on_submit(Message::LoadFile);
            
            Column::new().padding(20).spacing(20)
              .align_items(Horizontal::Center.into())
              .push(Text::new("Enter File Path:"))
              .push::<Element<Message>>(input.into())
              .width(Length::FillPortion(4))
              .into()
              
          }
        }
      } else {
        Column::new()
          .width(Length::FillPortion(1))
          .align_items(Alignment::Center)
          .push(Text::new("No popup"))
          .into()
      };


    Column::new().padding(20).spacing(10)
      .align_items(Alignment::Center)
      .push( Text::new("MoreMass").width(Length::Shrink).size(50) )
      .push(
        Row::new()
          .push(left)
          .push(center)
          .push(right)
      )
      .into()
  }
}

// --------------------------------------------------------
mod spectrum {
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
    
    pub fn _req_redraw(&mut self) {
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
}

mod ui_elements {
/*  use iced::{
    Element, 
    pure::{ button::Button }
  };
  use crate::Message;
  
  pub fn popup_button<'a>(text: &str, which: WhichPopup) -> Element<'a, Message> {
    
    Button::new(text).on_press(Message::Popup(kid)).into();

  }*/
}

// --------------------------------------------------------
mod backend {

  #[derive(Clone)]
  pub struct Dataset {
    pub author: String,
    pub peaks:  Vec<(f64, f64)>,
    pub y_min:  f64,
    pub y_max:  f64,
    pub x_min:  f64,
    pub x_max:  f64,
  }
  
  pub struct Data {
    pub sets: Vec<Dataset>,
    pub x_min_g: f64,
    pub x_max_g: f64,
    pub y_min_g: f64,
    pub y_max_g: f64,
  }
  
  impl Default for Data {
    fn default() -> Self { Data {
      x_min_g: f64::MAX,
      x_max_g: 0f64,
      y_min_g: f64::MAX,
      y_max_g: 0f64,
      sets:    vec![]
    }}
  }
  
  impl Data {
    pub fn push(&mut self, ds: Option<Dataset>) {
      if let Some(d) = ds {
        self.x_min_g = self.x_min_g.min(d.x_min);
        self.x_max_g = self.x_max_g.max(d.x_max);
        self.y_min_g = self.y_min_g.min(d.y_min);
        self.y_max_g = self.y_max_g.max(d.y_max);
        self.sets.push(d);
      }
    }
  }

  pub mod parser {
    use crate::backend::Dataset;
    use std::fs::File;
    use std::path::Path;
    use std::io::{self, BufRead, Cursor, Read};
    use base64::decode_config;
    
  //  use byteorder::{BigEndian, ReadBytesExt};

  //  const CNT_HEAD:   &str = "          peaksCount=\"";
    const PEAKS_HEAD: &str = "             contentType=\"m/z-int\">";
    const CONFIG: base64::Config = base64::Config::new(base64::CharacterSet::Standard, true);

    pub fn parse_mzxml_badly( s: &String ) -> Option<Dataset> {

      let path = Path::new(s);
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
          
          let mut peaks: Vec<(f64, f64)> = Vec::with_capacity(bytes.len() / 16);
          let mut buf:   [u8; 8]    = [0u8; 8];
          let mut cursor            = Cursor::new(&bytes);
          let mut x: f64;
          let mut y: f64;
          
          for _i in 0 .. (bytes.len() / 16) {
            match cursor.read(&mut buf) {
              Ok(n) => {
                if n != 8 {
                  break;
                }
                
                x = f64::from_be_bytes(buf);
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
                
                y = f64::from_be_bytes(buf);
              },
              Err(why) => {
                println!("Error occured while parsing data: {:?}", why);
                return None
              }
            }
            peaks.push((x, y));
          }

          return Some(Dataset {
            author: "MariuuuUUUUus".to_string(),
            y_min:  peaks.iter().map(|&(_, y)| {y}).fold(0.0f64,   |acc, v| if acc < v {acc} else {v}),
            y_max:  peaks.iter().map(|&(_, y)| {y}).fold(0.0f64,   |acc, v| if acc > v {acc} else {v}),
            x_min:  peaks.iter().map(|&(x, _)| {x}).fold(f64::MAX, |acc, v| if acc < v {acc} else {v}),
            x_max:  peaks.iter().map(|&(x, _)| {x}).fold(f64::MAX, |acc, v| if acc > v {acc} else {v}),
            peaks:  peaks,
          });
        }
      }
      
      return None;
    }
  }
}
