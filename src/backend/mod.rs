pub mod parser;

#[derive(Clone)]
pub struct Dataset {
  pub author: String,
  pub peaks:  Vec<(f64, f64)>,
  pub y_min:  f64,
  pub y_max:  f64,
  pub x_min:  f64,
  pub x_max:  f64,
}

pub struct Data {
  pub sets: Vec<Dataset>,
  pub x_min_g: f64,
  pub x_max_g: f64,
  pub y_min_g: f64,
  pub y_max_g: f64,
}

impl Default for Data {
  fn default() -> Self { Data {
    x_min_g: f64::MAX,
    x_max_g: 0f64,
    y_min_g: f64::MAX,
    y_max_g: 0f64,
    sets:    vec![]
  }}
}

impl Data {
  pub fn push(&mut self, ds: Option<Dataset>) {
    if let Some(d) = ds {
      self.x_min_g = self.x_min_g.min(d.x_min);
      self.x_max_g = self.x_max_g.max(d.x_max);
      self.y_min_g = self.y_min_g.min(d.y_min);
      self.y_max_g = self.y_max_g.max(d.y_max);
      self.sets.push(d);
    }
  }
}
