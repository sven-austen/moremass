pub mod parser;
use chrono::{DateTime, Utc, TimeZone};

#[derive(Clone)]
pub struct Dataset {
  pub title:       String,
  pub operator:    String,
  pub contact:     String,
  pub institution: String,
  pub instrument:  String,
  pub date:        DateTime<Utc>,
  pub path:        String, 
  
  pub points: Vec<(f64, f64)>,
  pub peaks:  Vec<(f64, f64)>,
  pub y_min:  f64,
  pub y_max:  f64,
  pub x_min:  f64,
  pub x_max:  f64,
  
  pub visible: bool,
}

impl Default for Dataset {
  fn default() -> Self { Dataset {
    title:       "".to_string(),
    operator:    "".to_string(),
    contact:     "".to_string(),
    institution: "".to_string(),
    instrument:  "".to_string(),
    date:        Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
    path:        "".to_string(),
    
    points: vec![],
    peaks:  vec![],
    y_min:  500.0,
    y_max:  0.0,
    x_min:  500.0,
    x_max:  0.0,
    
    visible: true,
  }}
}

impl Dataset {
  pub fn find_peaks(&mut self) {
    
    // TODO this; right now i just spit out some junk data
      
    self.peaks = vec![
      (0.0, 0.0),
      (610.0, 100.0),
      (650.0, 300.0),
      (680.0, 160.0),
      (740.0, 400.0)
    ];
    
  }
}

#[derive(Default)]
pub struct Data {
  pub sets: Vec<Dataset>,
  pub x_min_g: f64,
  pub x_max_g: f64,
  pub y_min_g: f64,
  pub y_max_g: f64,
  pub curr_ds: usize,
}

impl Data {
  pub fn push(&mut self, ds: Option<Dataset>) {
    if let Some(d) = ds {
      if self.sets.len() > 0 {
        self.x_min_g = self.x_min_g.min(d.x_min);
        self.x_max_g = self.x_max_g.max(d.x_max);
        self.y_min_g = self.y_min_g.min(d.y_min);
        self.y_max_g = self.y_max_g.max(d.y_max);
      } else {
        self.x_min_g = d.x_min;
        self.x_max_g = d.x_max;
        self.y_min_g = d.y_min;
        self.y_max_g = d.y_max;
      }
      self.sets.push(d);
      self.curr_ds = self.sets.len() - 1;
    }
  }
}
