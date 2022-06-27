
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
            )
            .on_submit(self.parse_inputs())
            .width(Length::FillPortion(1))
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
            )
            .on_submit(self.parse_inputs())
            .width(Length::FillPortion(1))
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
            )
            .on_submit(self.parse_inputs())
            .width(Length::FillPortion(1))
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
    
    let ratio   = if let Ok(v) = f64::from_str(&self.sn_ratio) {
      v
    } else {
      0.0
    };
    let abs_int = if let Ok(v) = f64::from_str(&self.abs_intensity) {
      v
    } else {
      0.0
    };
    let rel_int = if let Ok(v) = f64::from_str(&self.rel_intensity) {
      v
    } else {
      0.0
    };
    
    Message::FindPeaks(ratio, abs_int, rel_int)
    
  }

}
