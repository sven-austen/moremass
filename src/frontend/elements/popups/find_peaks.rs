
use crate::Message;
use crate::frontend::elements::popups::ForPopup;

use std::str::FromStr;

use iced::{
  Length, 
  
  pure::{
    Element, column, text, row, text_input, button, 
  }
};

#[derive(Debug, Clone)]
pub enum FindPeaksMsg {
  SNRatio(String),
  AbsIntensity(String),
  RelIntensity(String),
}

#[derive(Default, Debug, Clone)]
pub struct State {
  sn_ratio: String,
  abs_intensity: String,
  rel_intensity: String,
}

impl State {

  pub fn view<'a>(&self) -> Element<'a, Message> {
  
    column().padding(20).spacing(20)
      .push(text("Find Peaks").size(20u16))
      .push(
        row()
          .push(text("S/N-Ratio").width(Length::FillPortion(1)))
          .push(
            text_input(
              "",
              &self.sn_ratio,
              |s| {
                Message::ForPopup(
                  ForPopup::ForFindPeaks(
                    FindPeaksMsg::SNRatio(s))) }
            ).width(Length::FillPortion(1))
          )
      ).push(
        row()
          .push(text("Abs. Intensity").width(Length::FillPortion(1)))
          .push(
            text_input(
              "",
              &self.abs_intensity,
              |s| {
                Message::ForPopup(
                  ForPopup::ForFindPeaks(
                    FindPeaksMsg::AbsIntensity(s))) },
            ).width(Length::FillPortion(1))
          )
      ).push(
        row()
          .push(text("Rel. Intensity").width(Length::FillPortion(1)))
          .push(
            text_input(
              "",
              &self.rel_intensity,
              |s| {
                Message::ForPopup(
                  ForPopup::ForFindPeaks(
                    FindPeaksMsg::RelIntensity(s)))}
            ).width(Length::FillPortion(1))
          )
      ).push(
        button(text("Go")).on_press(self.parse_inputs())
      ).into()
    
  }
  
  pub fn update(&mut self, msg: FindPeaksMsg) {
    match msg {
      FindPeaksMsg::SNRatio(s) => {
        self.sn_ratio = s;
      }
      FindPeaksMsg::AbsIntensity(s) => {
        self.abs_intensity = s;
      }
      FindPeaksMsg::RelIntensity(s) => {
        self.rel_intensity = s;
      }
    }
  }
  
  pub fn parse_inputs(&self) -> Message {
    
    let res_ratio = f64::from_str(&self.sn_ratio);
    let res_abs   = f64::from_str(&self.abs_intensity);
    let res_rel   = f64::from_str(&self.rel_intensity);
    
    if let Ok(ratio) = res_ratio {
      if let Ok(abs_int) = res_abs {
        if let Ok(rel_int) = res_rel {
          return Message::FindPeaks(ratio, abs_int, rel_int);
        }
      }
    }
    Message::Noop
    
  }

}
