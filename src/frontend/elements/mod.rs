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
    Element, column, row, button, scrollable, text, container,
    widget::{ Row }
  },
  Length, Svg, alignment, Alignment, Color
};

pub fn view_datasets<'a>(
  datasets: &Vec<Dataset>,
  selected: usize
) -> Element<'a, Message> {

  let dataset_list = 
    datasets
      .iter()
      .enumerate()
      .fold(column().spacing(2).padding(10), |col, (i, ds)| {

        let btn = button(
          row().spacing(2).padding(2)
            .align_items(alignment::Horizontal::Center.into())
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
            
              column().padding(0).spacing(0).push(
                row().padding(2).spacing(2).push(
                  text("Title: ").size(12u16).width(Length::FillPortion(1))
                ).push(
                  text(format!("{}", ds.title)).size(12u16).width(Length::FillPortion(1))
                )).push(row().padding(2).spacing(2).push(
                  text("Operator: ").size(12u16).width(Length::FillPortion(1))
                ).push(
                  text(format!("{}", ds.operator)).size(12u16).width(Length::FillPortion(1))
                )).push(row().padding(2).spacing(2).push(
                  text("Contact: ").size(12u16).width(Length::FillPortion(1))
                ).push(
                  text(format!("{}", ds.contact)).size(12u16).width(Length::FillPortion(1))
                )).push(row().padding(2).spacing(2).push(
                  text("Institution: ").size(12u16).width(Length::FillPortion(1))
                ).push(
                  text(format!("{}", ds.institution)).size(12u16).width(Length::FillPortion(1))
                )).push(row().padding(2).spacing(2).push(
                  text("Instrument: ").size(12u16).width(Length::FillPortion(1))
                ).push(
                  text(format!("{}", ds.instrument)).size(12u16).width(Length::FillPortion(1))
                )).push(row().padding(2).spacing(2).push(
                  text("Date: ").size(12u16).width(Length::FillPortion(1))
                ).push(
                  text(format!("{:?}", ds.date.format("%d.%m.%Y - %H:%M:%S").to_string())).size(12u16).width(Length::FillPortion(1))
                )
              )
              
            } else {
              column().padding(0).spacing(0)
            }
          );
        
        col.push(ds_col)
      
      });
  
  // peak list
  let peak_list = if selected < datasets.len() {
    
    if datasets[selected].peaks.len() > 0 {
    
      datasets[selected].peaks.iter()
        .fold(
          column().spacing(2).padding(10)
            .push(row().padding(2).spacing(5)
              .push(text("m/z").size(14u16).width(Length::FillPortion(1)))
              .push(text("int").size(14u16).width(Length::FillPortion(1)))), 
          |col, (x, y)| {
            col.push(
              row().padding(2).spacing(5)
                .push(text(format!("{:.3}", x)).size(12u16)
                  .width(Length::FillPortion(1))
                  .horizontal_alignment(alignment::Horizontal::Right)
                )
                .push(text(format!("{:.0}", y)).size(12u16)
                  .width(Length::FillPortion(1))
                  .horizontal_alignment(alignment::Horizontal::Right)
                )
            )
        })
    } else {
      column()
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .push(
          text("No peaks selected").size(14u16)
            .color(Color {r: 0.2, g: 0.2, b: 0.2, a: 0.8})
        )
      
    }
    
  } else {
    column().padding(20).spacing(0)
      .align_items(Alignment::Center)
      .push(
        text("No Dataset Selected").size(14u16)
          .color(Color {r: 0.2, g: 0.2, b: 0.2, a: 0.8})
      )
  };
  
  
  row().padding(0).spacing(0)
    .push(container(scrollable(dataset_list)).width(Length::FillPortion(3)))
    .push(container(scrollable(peak_list   )).width(Length::FillPortion(2)))
    .into()
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


