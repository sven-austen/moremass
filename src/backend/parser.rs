
use crate::backend::Dataset;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, Cursor, Read};
use base64::decode_config;

//  use byteorder::{BigEndian, ReadBytesExt};

/*const OPERATOR_HEAD:    &str = "";
const CONTACT_HEAD:     &str = "";
const INSTITUTION_HEAD: &str = "";
const INSTRUMENT_HEAD:  &str = "";
const DATE_HEAD:        &str = "";
const PATH_HEAD:        &str = "    <parentFile fileName=\"";*/
const POINTS_HEAD:       &str = "             contentType=\"m/z-int\">";
const CONFIG: base64::Config = base64::Config::new(base64::CharacterSet::Standard, true);

pub fn parse_mzxml_badly( s: &String ) -> Option<Dataset> {

  let path = Path::new(s);
  let display = path.display();
  
  // open file
  let file = match File::open(&path) {
    Err(why) => {
      println!("Could not open {}: {}", display, why);
      return None;
    }
    Ok(file) => file
  };
  
  // iterate over lines
  for maybe_line in io::BufReader::new(file).lines() {
  
    let line = match maybe_line {
      Err(why) => {
        println!("Could not read line from data file: {:?}", why);
        return None;
      }
      Ok(l) => l
    };
    
    // read peak list
    if line.len() >= POINTS_HEAD.len() && 
       line[..POINTS_HEAD.len()] == *POINTS_HEAD {
        
      let bytes = match decode_config(&line[POINTS_HEAD.len() .. line.len()-8], CONFIG) {
        Err(why) => {
          println!("Could not decode peak list: {:?}", why);
          return None;
        }
        Ok(bs) => bs
      };
      
      let mut points: Vec<(f64, f64)> = Vec::with_capacity(bytes.len() / 16);
      let mut buf:   [u8; 8]    = [0u8; 8];
      let mut cursor            = Cursor::new(&bytes);
      let mut x: f64;
      let mut y: f64;
      
      for _i in 0 .. (bytes.len() / 16) {
        match cursor.read(&mut buf) {
          Ok(n) => {
            if n != 8 {
              break;
            }
            
            x = f64::from_be_bytes(buf);
          },
          Err(why) => {
            println!("Error occured while parsing data: {:?}", why);
            return None
          }
        }
        
        // read again for second value
        match cursor.read(&mut buf) {
          Ok(n) => {
            if n != 8 {
              break;
            }
            
            y = f64::from_be_bytes(buf);
          },
          Err(why) => {
            println!("Error occured while parsing data: {:?}", why);
            return None
          }
        }
        points.push((x, y));
      }

      return Some(Dataset {
        y_min:  points.iter().map(|&(_, y)| {y}).fold(0.0f64,   |acc, v| if acc < v {acc} else {v}),
        y_max:  points.iter().map(|&(_, y)| {y}).fold(0.0f64,   |acc, v| if acc > v {acc} else {v}),
        x_min:  points.iter().map(|&(x, _)| {x}).fold(f64::MAX, |acc, v| if acc < v {acc} else {v}),
        x_max:  points.iter().map(|&(x, _)| {x}).fold(f64::MAX, |acc, v| if acc > v {acc} else {v}),
        points: points,
        title: "Test Title".to_string(),
        ..Dataset::default()
      });
    }
  }
  
  return None;
}
