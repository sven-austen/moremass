
use crate::backend::MSPoint;

pub fn sort_in_place(arr: &mut Vec<f64>) {
  arr.sort_by(|a, b| {a.partial_cmp(b).unwrap()});
}

pub fn get_maxima(pts: &Vec<MSPoint>) -> Vec<usize> {
  let mut rising   = false;
  let mut last_int = 0.0;
  let mut maxima   = vec![];
  
  for (i, pt) in pts.iter().enumerate() {
    if last_int > pt.int {
      if rising {
        maxima.push(i-1);
      }
      rising = false;
    } else {
      rising = true;
    }
    last_int = pt.int;
  }
  
  maxima
}

pub fn update_snrs(
  pts:         &mut Vec<MSPoint>, 
  maxima:      &Vec<usize>,
  noise_level: f64,
  noise_width: f64
) {
  print!("lvl: {:?}, width: {:?}", noise_level, noise_width);

  if noise_width > 0.0 {
  
    for i in maxima {
      pts[*i].snr = (pts[*i].int - noise_level) / noise_width;
    }
  
  }
}
