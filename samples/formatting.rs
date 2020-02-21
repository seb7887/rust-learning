use std::fmt::{self, Formatter, Display};

struct City {
  name: &'static str,
  lat: f32,
  lon: f32,
}

impl Display for City {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
    let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

    write!(f, "{}: {:.3}°{} {:3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
  }
}

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "RGB ({r}, {g}, {b}) 0x{r:X}{g:X}{b:X}", r = self.red, g = self.green, b = self.blue)
  }
}

fn main() {
  for city in [
    City { name: "Dublin", lat: 53.34778, lon: -6.259772 },
    City { name: "Oslo", lat: 56.95, lon: 10.75 },
  ].iter() {
    println!("{}", *city);
  }

  for color in [
    Color { red: 128, green: 255, blue: 90 },
  ].iter() {
    println!("{}", *color);
  }
}