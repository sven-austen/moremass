mod backend;
mod frontend;

use crate::backend::{
  Dataset,
  parser::parse_mzxml_badly
};

use crate::frontend::plot;
//use crate::frontend::plot;
use crate::frontend::elements::*;
use crate::frontend::elements::header::*;

use iced::{
  Settings, Length,
  Alignment, alignment::Horizontal,
  pure::{ 
    row, text, Sandbox, Element, column, text_input,
  },
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
  
  plot: plot::State,
  popup: Option<WhichPopup>,
}

#[derive(Default)]
struct Controls {
  file_path: String,
}

// Message Types ----------------------
#[derive(Debug, Clone)]
pub enum Message {
  Popup( WhichPopup ),
  ChangeFilePath( String ),
  ToggleVisibility( usize ),
  SelectDataset( usize ),
  
  FileOp( WhichFileOp ),
  ProcessingOp( WhichProcessingOp ),
  
  ForPlot( plot::PlotMsg ),
  AddPeak( (f64, f64) ),
  LoadFile,
  LoadFromPath( String ),
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
        self.controls.file_path = s;
      }
      Message::LoadFile => {
        self.data.push(parse_mzxml_badly(&self.controls.file_path));
        self.controls.file_path = "".to_string();
        self.popup = None;
        self.plot.rethink_bounds(&self.data);
        self.plot.req_redraw();
      }
      
      Message::LoadFromPath(s) => {
        self.data.push(parse_mzxml_badly(&s));
        self.controls.file_path = "".to_string();
        self.popup = None;
        self.plot.rethink_bounds(&self.data);
        self.plot.req_redraw();
      }
      
      Message::ToggleVisibility(index) => {
        self.data.sets[index].visible = !self.data.sets[index].visible;
        self.plot.req_redraw();
      }
      
      Message::SelectDataset(index) => {
        if index < self.data.sets.len() {
          self.data.curr_ds = index;
          if !self.data.sets[index].visible {
            self.data.sets[index].visible = true;
            self.plot.req_redraw();
          }
        }
      }
      
      Message::FileOp(which) => {
      
        match which {
          WhichFileOp::New => {
            self.data.push(Some(Dataset::default()));
          }
          WhichFileOp::Open => {
            self.popup = Some(WhichPopup::FindFile);
          }
          WhichFileOp::CloseAll => {
            self.data = crate::backend::Data::default();
            self.plot.req_redraw();
          }
        }
        
      }
      
      Message::ProcessingOp(which) => {
      
        match which {
          WhichProcessingOp::FindPeaks => {
            if self.data.curr_ds < self.data.sets.len() {
              self.data.sets[self.data.curr_ds].find_peaks(); // TODO implement popup for entering parameters
              self.plot.req_redraw();
            }
          }
        }
      
      }
      
      Message::ForPlot(msg) => {
        self.plot.update(msg);
      }
      
      Message::AddPeak(pk) => {
        // assumes that curr_ds is valid bc that was checked for in Plot
        self.data.sets[self.data.curr_ds].peaks.push(pk);
        self.plot.r_click = None;
        self.plot.req_redraw();
      }
      
      Message::Clear => {
        self.data = crate::backend::Data::default();
        self.plot.req_redraw();
//        self.reset_canvas_view();
      }
      Message::Noop => { }
    }
  }
  
  fn view(&self) -> Element<Message> {

    
    let center = column().push(
      self.plot.view(&self.data)
//      self.plot.view(&self.data).map(Message::ForPlot)
    )
    .width(Length::FillPortion(6));
    
    
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
                  &self.controls.file_path,
                  Message::ChangeFilePath,
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
            .push( 
              row().push(view_datasets(&self.data.sets, self.data.curr_ds))
                .width(Length::FillPortion(2))
            )
//            .push(peaks)
            .push(center)
            .push(right)
        )
        .into()

  }
}

