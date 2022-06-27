
pub mod elements;
pub mod plot;

use crate::{ Message, MoreMass };
use crate::frontend::elements::{
  header::header, ribbon, view_datasets, 
};

use iced::{
  Svg, Length,
  Alignment, 
  pure::{ 
    row, Element, column, container
  },
};


pub fn get_icon(s: &str) -> Svg {
  Svg::from_path(format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), s))
}

pub fn view<'a>(mm: &'a MoreMass) -> Element<'a, Message> {

  let center = column().push(
    mm.plot.view(&mm.data)
  ).width(Length::FillPortion(5));
  
  
  let right: Element<Message> = mm.popup.view();
  
  column().padding(20)
    .align_items(Alignment::Center)
    .push(header())
    .push(ribbon())
    .push(
      row()
        .push( 
          container(view_datasets(&mm.data.sets, mm.data.curr_ds))
            .width(Length::FillPortion(2))
        )
        .push(center)
        .push(right)
    )
    .into()

}
