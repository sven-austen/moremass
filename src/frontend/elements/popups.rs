
mod find_file;
mod find_peaks;

use crate::{
  Message,
  frontend::get_icon,
};

use iced::{
  Length,
  pure::{
    Element, column, row, button, 
  }
};

#[derive(Debug, Clone)]
pub enum WhichPopup {
  NoPopup,
  FindFile(find_file::State),
  FindPeaks(find_peaks::State),
}

impl Default for WhichPopup {
  fn default() -> Self {
    WhichPopup::NoPopup
  }
}

#[derive(Debug, Clone)]
pub enum ForPopup {
  ForFindFile(find_file::FindFileMsg),
  ForFindPeaks(find_peaks::FindPeaksMsg),
}

impl WhichPopup {

  pub fn view(&self) -> Element<Message> {
  
    let body: Element<Message> = match self {
    
      WhichPopup::FindFile(state) => {
        state.view()
      }
      
      WhichPopup::FindPeaks(state) => {
        state.view()
      }
      
      _ => { return column().width(Length::Units(0)).into(); }
      
    };
    
    let controls: Element<Message> = 
      row().padding(2).spacing(5)
        .push(
          button(get_icon("outline-cross-small.svg"))
            .padding(0)
            .width(Length::Units(15))
            .height(Length::Units(15))
            .on_press(Message::ClosePopup)
        ).into();
    
    column().padding(0).spacing(0)
      .width(Length::FillPortion(3))
      .push(controls)
      .push(body)
      .into()
  }
  
  pub fn update(&mut self, msg: ForPopup) {
    
    match self {
    
      WhichPopup::FindFile(state) => {
        match msg {
          ForPopup::ForFindFile(m) => {state.update(m);}
          _ => {}
        }
      }
      
      WhichPopup::FindPeaks(state) => {
        match msg {
          ForPopup::ForFindPeaks(m) => {state.update(m);}
          _ => {}
        }
      }
      
      WhichPopup::NoPopup => { }
    
    }
    
  }

}

pub fn new_find_file() -> WhichPopup {
  WhichPopup::FindFile(find_file::State::default())
}

pub fn new_find_peaks() -> WhichPopup {
  WhichPopup::FindPeaks(find_peaks::State::default())
}

