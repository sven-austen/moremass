
use crate::Message;

use crate::frontend::elements::popups::ForPopup;

use iced::{
  alignment::Horizontal,
  
  pure::{
    Element, column, text, text_input, 
  }
};

#[derive(Debug, Clone)]
pub enum FindFileMsg {
  PathInput(String),
}

#[derive(Default, Debug, Clone)]
pub struct State {
  file_path: String,
}

impl State {

  pub fn view<'a>(&self) -> Element<'a, Message> {

    column().padding(20).spacing(20)
      .align_items(Horizontal::Center.into())
      .push(text("Enter File Path:"))
      .push(
        text_input(
          "File Path",
          &self.file_path,
          |s| { Message::ForPopup(ForPopup::ForFindFile(FindFileMsg::PathInput(s)))},
        ).on_submit(Message::LoadFromPath(self.file_path.clone()))
      ).into()
  }
  
  pub fn update(&mut self, msg: FindFileMsg) {
    match msg {
      FindFileMsg::PathInput(s) => {
        self.file_path = s;
      }
    }
  }

}
