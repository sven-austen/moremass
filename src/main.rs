mod backend;
mod frontend;

use crate::backend::{
  Dataset,
  parser::parse_mzxml_badly
};

use crate::frontend::spectrum_view;

use iced::{
  Settings, Length, Point, Rectangle,
  Alignment, alignment::Horizontal,
  pure::{ self, button, pick_list, row, scrollable, text, Sandbox, Element, column, text_input }
};


pub fn main() -> iced::Result {
  MoreMass::run(Settings {
    antialiasing: true,
    ..Settings::default()
  })
}

// Application State ------------------
#[derive(Default)]
struct MoreMass {
  data:         backend::Data,
  controls:     Controls,
  file_path:    String,
  
  canvas_view:  Rectangle,
  
  canvas_state: spectrum_view::State,
  popup:        Option<WhichPopup>,
}

#[derive(Default)]
struct Controls {
//  btn_load:  button::State,
//  btn_clear: button::State,
  
//  scr_datasets: scrollable::State,
  
//  txt_file:  text_input::State,
}

// Message Types ----------------------
#[derive(Debug, Clone)]
pub enum Message {
  Popup( WhichPopup ),
  ChangeFilePath( String ),
  FileOp( WhichFileOp ),
  CanvasMessage( spectrum_view::CanvasMsg ),
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
        self.data.push(parse_mzxml_badly(&self.file_path));
        self.file_path = "".to_string();
        self.popup = None;
        self.canvas_state.x0 = self.data.x_min_g as f32;
        self.canvas_state.req_redraw();
//        self.reset_canvas_view();
      }
      Message::FileOp(which) => {
      
        match which {
          WhichFileOp::New => {
            self.data.push(Some(Dataset::default()));
  //          self.reset_canvas_view();
          }
          WhichFileOp::Open => {
            self.popup = Some(WhichPopup::FindFile);
    //        self.reset_canvas_view();
          }
          WhichFileOp::CloseAll => {
            self.data = crate::backend::Data::default();
            self.canvas_state.req_redraw();
      //      self.reset_canvas_view();
          }
        }
        
      }
      
      Message::CanvasMessage(msg) => {
        match msg {
          spectrum_view::CanvasMsg::LClick(pt) => {
            self.canvas_state.l_click = Some(pt);
          }
          
          spectrum_view::CanvasMsg::RClick(pt) => {
            self.canvas_state.r_click = Some(pt);
          }
          
          spectrum_view::CanvasMsg::MouseUp => {
            self.canvas_state.l_click = None;
            self.canvas_state.r_click = None;
          }
          
          spectrum_view::CanvasMsg::MoveTo(x1, y1, sx, sy) => {
            
            if let Some(Point {x, y}) = self.canvas_state.l_click {
              self.canvas_state.x0 += (x - x1) / sx;
            
              self.canvas_state.l_click = Some(
                Point { x: x1, y: y1 }
              );
            }
            
            if let Some(Point {x, y}) = self.canvas_state.r_click {
              
              self.canvas_state.r_click = Some(
                Point { x: x1, y: y1 }
              );
            }
            
            self.canvas_state.req_redraw();
          }
          
          spectrum_view::CanvasMsg::Noop => { }
        }
      }
      
      Message::Clear => {
        self.data = crate::backend::Data::default();
        self.canvas_state.req_redraw();
//        self.reset_canvas_view();
      }
      Message::Noop => { }
    }
  }
  
  fn view(&self) -> Element<Message> {

    
    let center = column().push(
      self.canvas_state.view(&self.data).map(Message::CanvasMessage)
    )
//    .push(text())
    .width(Length::FillPortion(5));
    
    
    let right: Element<Message> = 
      if let Some(which) = self.popup {
        match which {
        
          WhichPopup::FindFile => {
          
            column().padding(20).spacing(20)
              .align_items(Horizontal::Center.into())
              .push(text("Enter File Path:"))
              .push(
                text_input(
                  "File Path",
                  &self.file_path,
                  |s| { Message::ChangeFilePath(s) },
                ).on_submit(Message::LoadFile)
              )
              .width(Length::FillPortion(4))
              .into()
              
          }
        }
      } else {
        column()
          .width(Length::FillPortion(1))
          .align_items(Alignment::Center)
          .push(text("No popup"))
          .into()
      };

      column().padding(20)
        .align_items(Alignment::Center)
        .push(header())
        .push(ribbon())
        .push(
          row()
            .push(loaded_datasets())
            .push(center)
            .push(right)
        )
        .into()

/*    Column::new().padding(20).spacing(10)
      .align_items(Alignment::Center)
      .push( Text::new("MoreMass").width(Length::Shrink).size(50) )
      .push(
        Row::new()
          .push(left)
          .push(center)
          .push(right)
      )
      .into()*/
  }
}
/*
impl MoreMass {

  reset_canvas_view(&mut self) {
    
    self.canvas_view.x      = self.data.x_min_g.min(self.data.x_max_g);
    self.canvas_view.y      = 0.0;
    self.canvas_view.width  = 5.0; // *  pt.x
    self.canvas_view.height = 1.0; // * (y_max_g - pt.y)
    
  }

}*/

fn header() -> pure::Element<'static, Message> {

  row().padding(0).spacing(5)
    .width(Length::Fill).height(Length::Units(20))
    .push(
      pick_list(
        &WhichFileOp::ALL[..],
        None,
        Message::FileOp
      ).placeholder("File")
    )
    .into()
}

fn ribbon() -> pure::Element<'static, Message> {
  row().padding(10).spacing(10)
    .width(Length::Fill).height(Length::Units(40))
    .push(
      text("I am a ribbon :)")
    )
    .into()
}

fn loaded_datasets() -> pure::Element<'static, Message> {
  let s = scrollable(text("i am scrollable"));
  
  /*for ds in &self.data.sets {
    s.push(Text::new(ds.author.clone()));
  }*/
  s.into()
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WhichFileOp {
  New,
  Open,
  CloseAll,
}

impl WhichFileOp {
  const ALL: [WhichFileOp; 3] = [
    WhichFileOp::New,
    WhichFileOp::Open,
    WhichFileOp::CloseAll,
  ];
}

impl std::fmt::Display for WhichFileOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WhichFileOp::New => "New",
                WhichFileOp::Open => "Open",
                WhichFileOp::CloseAll => "Close All",
            }
        )
    }
}

