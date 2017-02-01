use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    latitude: f32,
    longitude: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.latitude >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.longitude >= 0.0 { 'W' } else { 'W' };

        write!(f, "{}: {:3}°{} {:3}°{}", self.name, self.latitude.abs(), lat_c, self.longitude.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main(){
    for city in [
            City { name: "Dublin", latitude: 53.347778, longitude: -6.259722 },
            City { name: "Oslo", latitude: 59.95, longitude: 10.75 },
            City { name: "Vancouver", latitude: 49.25, longitude: -123.1 },
        ].iter() {
            println!("{}", *city);
        }
        for color in [
            Color { red: 128, green: 255, blue: 90 },
            Color { red: 0, green: 3, blue: 254 },
            Color { red: 0, green: 0, blue: 0 },
        ].iter() {
            // Switch this to use {} once you've added an implementation
            // for fmt::Display
            println!("{:?}", *color)
        }
}
