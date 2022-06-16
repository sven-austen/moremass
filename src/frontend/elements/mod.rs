/*
 * A collection of UI elements
 */

pub mod header;

use crate::{
  Message,
  backend::{ Dataset },
};

use iced::{
  pure::{
    column, row, button, scrollable, text, container,
    widget::{ Scrollable, Row }
  },
  Length, Svg, alignment::Horizontal
};

pub fn view_datasets<'a>(
  datasets: &Vec<Dataset>,
  selected: usize
) -> Scrollable<'a, Message> {

  let dataset_list = 
    datasets
      .iter()
      .enumerate()
      .fold(column().spacing(2).padding(10), |col, (i, ds)| {

        let btn = button(
          row().spacing(2).padding(2)
            .align_items(Horizontal::Center.into())
            .push(
              container (if selected == i {
                get_icon("outline-chevron-down-small-round.svg")
              } else {
                get_icon("outline-chevron-right-small-round.svg")
              })
              .width(Length::Units(20))
            )
            .push(
              button( if ds.visible {
                get_icon("outline-eye-open.svg")
              } else {
                get_icon("outline-eye-closed.svg")
              })
              .on_press(Message::ToggleVisibility(i))
              .width(Length::Units(20))
            )
            .push(
              text(ds.title.clone())
                .size(14u16)
                .width(Length::Fill)
            )
        )
        .on_press(Message::SelectDataset(i));
        
        let ds_col = column().padding(0).spacing(0)
          .push(btn)
          .push(
            if selected == i {
            
              if ds.peaks.len() == 0 {
                column().padding(0).spacing(0).push(
                  text("No peaks registered").size(12u16)
                )
              } else {
                ds.peaks.iter().enumerate()
                  .fold(column().padding(0).spacing(0), |pk_col, (_j, (x, y))| {
                    pk_col.push(
                      row().padding(0).spacing(0)
                        .push(text(format!("x: {:.3}", x)).size(12u16).width(Length::FillPortion(1)))
                        .push(text(format!("y: {:.3}", y)).size(12u16).width(Length::FillPortion(1)))
                    )
                  })
              }
            } else {
              column().padding(0).spacing(0)
            }
          );
        
        col.push(ds_col)
      
      });
  
  scrollable(dataset_list)
  
}

pub fn ribbon<'a>() -> Row<'a, Message> {
  row().padding(10).spacing(10)
    .width(Length::Fill).height(Length::Units(60))
    .push(
      button( get_icon("outline-sun.svg") )
        .on_press(Message::LoadFromPath("/home/sven/Documents/projects/mMass/data/20220516/Input_caColumn_1.mzXML".to_string()))
    )
    .push(
      button( get_icon("outline-star-2.svg") )
        .on_press(Message::LoadFromPath("/home/sven/Documents/projects/mMass/data/20220516/Input_caColumn_I2.mzXML".to_string()))
    )
    .push(
      button( get_icon("outline-search.svg") )
        .on_press(Message::Noop)
    )
    .push(
      button( get_icon("outline-share-1.svg") )
        .on_press(Message::Noop)
    )
    .push(
      button( get_icon("outline-share-2.svg") )
        .on_press(Message::Noop)
    )
    .push(
      button( get_icon("outline-shield-check.svg") )
        .on_press(Message::Noop)
    )
    .push(
      button( get_icon("outline-shirt.svg") )
        .on_press(Message::Noop)
    )
    .push(
      button( get_icon("outline-sound-2.svg") )
        .on_press(Message::Noop)
    )
    .into()
}

fn get_icon(s: &str) -> Svg {
  Svg::from_path(format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), s))
}


