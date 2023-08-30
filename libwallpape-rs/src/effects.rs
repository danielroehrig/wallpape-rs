mod gradient;
mod little_boxes;
mod voronoi;

use raqote::{Color, DrawTarget};
use std::str::FromStr;

pub enum Fx {
    LittleBoxes,
    Gradient,
    Voronoi,
}

impl Fx {
    pub const VALUES: [Self; 3] = [Self::LittleBoxes, Self::Gradient, Self::Voronoi];
}

impl ToString for Fx {
    fn to_string(&self) -> String {
        match self {
            Fx::LittleBoxes => String::from("little_boxes"),
            Fx::Gradient => String::from("gradient"),
            Fx::Voronoi => String::from("voronoi"),
        }
    }
}

impl FromStr for Fx {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "little_boxes" => Ok(Fx::LittleBoxes),
            "gradient" => Ok(Fx::Gradient),
            "voronoi" => Ok(Fx::Voronoi),
            _ => Err("Good not be converted to FX".to_string()),
        }
    }
}

pub fn run_fx(fx: Fx, palette: &Vec<Color>, dt: &mut DrawTarget) {
    match fx {
        Fx::LittleBoxes => little_boxes::render(palette, dt),
        Fx::Gradient => gradient::render(palette, dt),
        Fx::Voronoi => voronoi::render(palette, dt),
    }
}
