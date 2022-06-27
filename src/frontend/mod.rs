
pub mod elements;
pub mod plot;

use iced::Svg;

pub fn get_icon(s: &str) -> Svg {
  Svg::from_path(format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), s))
}
