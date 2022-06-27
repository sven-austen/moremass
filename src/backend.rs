pub mod parser;
mod calcs;
use chrono::{DateTime, Utc};

#[derive(Clone, Default, Debug)]
pub struct MSPoint {
  pub mz:  f64,
  pub int: f64,
  pub snr: f64, // Signal/Noise Ratio; only neq 0 for points that are local maxima
}

#[derive(Clone)]
pub struct Dataset {
  pub metadata: Metadata,
  
  pub points:  Vec<MSPoint>,
  pub peaks:   Vec<usize>,
  pub maxima:  Vec<usize>,
  pub mz_min:  f64,
  pub mz_max:  f64,
  pub int_min: f64,
  pub int_max: f64,
  
  pub noise_level: f64,
  pub noise_width: f64,
  
  pub visible: bool,
}

#[derive(Clone)]
pub struct Metadata {
  pub title:       String,
  pub operator:    String,
  pub contact:     String,
  pub institution: String,
  pub instrument:  String,
  pub date:        DateTime<Utc>,
  pub path:        String, 
}

impl Dataset {
  pub fn neww( md: Metadata, mut pts: Vec<MSPoint> ) -> Self {
  
    let mz_min = pts[0].mz;
    let mz_max = pts[pts.len() - 1].mz;
  
    let mut buf: Vec<f64> =
      pts.clone().iter()
        .map(|MSPoint {int, ..}| {*int})
        .collect();
    calcs::sort_in_place(&mut buf);
    
    let int_min     = buf[0];
    let int_max     = buf[ buf.len() - 1 ];
    let noise_level = buf[ buf.len() / 2 ];
    
    for i in 0..buf.len() {
      buf[i] = f64::abs(buf[i] - noise_level);
    }
    calcs::sort_in_place(&mut buf);
    let noise_width = buf[ buf.len() / 2 ] * 2.0;
    
    let maxima = calcs::get_maxima(&pts);
    calcs::update_snrs(&mut pts, &maxima, noise_level, noise_width);
  
    Dataset {
      metadata: md,
      points: pts,
      peaks:  vec![],
      maxima: maxima,
      
      mz_min:  mz_min,
      mz_max:  mz_max,
      int_min: int_min,
      int_max: int_max,
      
      noise_level: noise_level,
      noise_width: noise_width,
      
      visible: true,
    }
  
  }

  pub fn pushpeak(&mut self, i: usize) {
    if self.points.len() > i && !self.peaks.contains(&i) {
      self.peaks.push(i);
    }
  }
  
  pub fn removepeaks(&mut self, lower: f64, upper: f64) {
  
    self.peaks = self.peaks.clone().into_iter().filter(
      |&i| {
        self.points[i].mz < lower ||
        self.points[i].mz > upper
      }
    ).collect();

  }

  pub fn find_peaks(
    &mut self, 
    ratio:   f64, 
    abs_int: f64, 
    rel_int: f64,
    overwrite: bool
  ) {
    if overwrite {
      self.peaks = vec![];
    }
    
    let threshold = f64::max(self.int_max * rel_int, abs_int);
    
    for i in self.maxima.clone() {
      if self.points[i].int > threshold && 
         self.points[i].snr > ratio {
      
        self.pushpeak(i);
      }
    }
    
  }
  
}

#[derive(Default)]
pub struct Data {
  pub sets: Vec<Dataset>,
  pub mz_min:  f64,
  pub mz_max:  f64,
  pub int_min: f64,
  pub int_max: f64,
  pub curr_ds: usize,
}

impl Data {
  pub fn push(&mut self, ds: Option<Dataset>) {
    if let Some(d) = ds {
      if self.sets.len() > 0 {
        self.mz_min  = f64::min(self.mz_min,  d.mz_min);
        self.mz_max  = f64::max(self.mz_max,  d.mz_max);
        self.int_min = f64::min(self.int_min, d.int_min);
        self.int_max = f64::max(self.int_max, d.int_max);
      } else {
        self.mz_min  = d.mz_min;
        self.mz_max  = d.mz_max;
        self.int_min = d.int_min;
        self.int_max = d.int_max;
      }
      self.sets.push(d);
      self.curr_ds = self.sets.len() - 1;
    }
  }
}
