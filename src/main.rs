mod backend;
mod frontend;

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

// Application State ------------------
#[derive(Default)]
struct MoreMass {
  data:         backend::Data,
  controls:     Controls,
  file_path:    String,
  
  canvas_state: frontend::spectrum_view::State,
  popup:        Option<WhichPopup>,
}

#[derive(Default)]
struct Controls {
  btn_load:  button::State,
  btn_clear: button::State,
  
  txt_file:  text_input::State,
}

// Message Types ----------------------
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
        self.data.push(crate::backend::parser::parse_mzxml_badly(&self.file_path));
        self.file_path = "".to_string();
        self.popup = None;
        self.canvas_state.x0 = self.data.x_min_g as f32;
        self.canvas_state.req_redraw();
      }
      Message::Clear => {
        self.data = crate::backend::Data::default();
        self.canvas_state.req_redraw();
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

