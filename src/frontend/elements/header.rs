
use crate::{
  Message,
};

use iced::{
  pure::{
    row, pick_list, 
    widget::{ Row }
  },
  Length, alignment::Horizontal
};

pub fn header<'a>() -> Row<'a, Message> {

  row().padding(0).spacing(5)
    .width(Length::Fill).height(Length::Units(20))
    .align_items(Horizontal::Center.into())
    .push(
      pick_list(
        &WhichFileOp::ALL[..],
        None,
        Message::FileOp
      ).placeholder("File")
    )
    .push(
      pick_list(
        &WhichProcessingOp::ALL[..],
        None,
        Message::ProcessingOp
      ).placeholder("Processing")
    )
    .into()
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


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WhichProcessingOp {
  FindPeaks,
}

impl WhichProcessingOp {
  const ALL: [WhichProcessingOp; 1] = [
    WhichProcessingOp::FindPeaks,
  ];
}

impl std::fmt::Display for WhichProcessingOp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        WhichProcessingOp::FindPeaks => "Find Peaks",
      }
    )
  }
}

