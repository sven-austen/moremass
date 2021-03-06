mod backend;
mod frontend;

use crate::backend::{
  parser::parse_mzxml_badly
};

use crate::frontend::{
  plot, elements::{
    popups::{ self, WhichPopup },
    header::{ WhichFileOp, WhichProcessingOp },
  }
};
use iced::{
  Settings,
  pure::{
    Element, Sandbox, 
  }
};


pub fn main() -> iced::Result {
  MoreMass::run(Settings {
    antialiasing: true,
    ..Settings::default()
  })
}

// Application State ------------------
#[derive(Default)]
pub struct MoreMass {
  data:  backend::Data,
  
  plot:  plot::State,
  popup: WhichPopup,
}

// Message Types ----------------------
#[derive(Debug, Clone)]
pub enum Message {
  Popup( WhichPopup ),
  ClosePopup,
  
  FileOp( WhichFileOp ),
  ProcessingOp( WhichProcessingOp ),
  
  ForPlot ( plot::PlotMsg ),
  ForPopup( popups::ForPopup ),
  
  ToggleVisibility( usize ),
  SelectDataset( usize ),
  AddPeak( usize ),
  RemovePeaks( f64, f64 ),
  FindPeaks( f64, f64, f64 ),
  LoadFromPath( String ),
  Clear,
  
  Noop
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
        self.popup = which;
      }
      Message::ClosePopup => {
        self.popup = WhichPopup::NoPopup;
      }
      
      Message::LoadFromPath(s) => {
        self.data.push(parse_mzxml_badly(&s));
        self.popup = WhichPopup::NoPopup;
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
//            self.data.push(Some(Dataset::default()));
          }
          WhichFileOp::Open => {
            self.popup = popups::new_find_file();
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
              self.popup = popups::new_find_peaks();
            }
          }
        }
      
      }
      
      Message::ForPlot(msg) => {
        self.plot.update(msg);
      }
      
      Message::ForPopup(msg) => {
        self.popup.update(msg);
      }
      
      Message::AddPeak(i) => {
        // assumes that curr_ds is valid bc that was checked for in Plot
        self.data.sets[self.data.curr_ds].pushpeak(i);
        self.plot.r_click = None;
        self.plot.req_redraw();
      }
      
      Message::RemovePeaks(lower, upper) => {
        self.data.sets[self.data.curr_ds].removepeaks(lower, upper);
        self.plot.r_click = None;
        self.plot.req_redraw();
      }
      
      Message::FindPeaks(ratio, abs_int, rel_int) => {
        if self.data.sets.len() > self.data.curr_ds {
          self.data.sets[self.data.curr_ds].find_peaks(ratio, abs_int, rel_int, true);
          self.plot.req_redraw();
        }
        self.popup = WhichPopup::NoPopup;
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

    frontend::view(self)

  }
}

