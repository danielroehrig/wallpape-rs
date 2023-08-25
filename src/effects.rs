use clap::builder::Str;
use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand::{Rng, RngCore};
use raqote::{
    Color, DrawOptions, DrawTarget, Gradient, GradientStop, PathBuilder, Point, SolidSource,
    Source, Spread, StrokeStyle,
};
use std::cmp::{max, min};
use std::process::exit;
use voronator::delaunator::Point as Vpoint;
use voronator::VoronoiDiagram;
pub enum Fx {
    LittleBoxes,
    Gradient,
    Voronoi,
}

impl Fx {
    pub const VALUES: [Self; 3] = [Self::LittleBoxes, Self::Gradient, Self::Voronoi];
    pub fn to_s(&self) -> String {
        match self {
            Fx::LittleBoxes => String::from("little_boxes"),
            Fx::Gradient => String::from("gradient"),
            Fx::Voronoi => String::from("voronoi"),
        }
    }
}

pub fn draw_little_boxes(palette: &Vec<Color>, dt: &mut DrawTarget) {
    let rng_color: Color;
    let rng = &mut rand::thread_rng();
    match palette.choose(rng) {
        Some(c) => rng_color = *c,
        None => {
            eprintln!("Palette seems to be empty");
            exit(1);
        }
    }
    let size = dt.width() / 100;
    for col in 0..100 {
        for row in 0..=(dt.height() / size) {
            let deviation = match rng.next_u32() % 30 {
                x if x > 15 => -(x as i32 / 2),
                x if x < 15 => (x as i32) / 2,
                _ => 0,
            };

            let r = min(max(Color::r(rng_color) as i32 + deviation, 0), 255) as u8;
            let g = min(max(Color::g(rng_color) as i32 + deviation, 0), 255) as u8;
            let b = min(max(Color::b(rng_color) as i32 + deviation, 0), 255) as u8;
            let solid = Color::new(Color::a(rng_color), r, g, b);

            let color = Source::Solid(SolidSource::from(solid));
            let mut pb = PathBuilder::new();
            pb.rect(
                (col * size) as f32,
                (row * size) as f32,
                size as f32,
                size as f32,
            );
            let path = pb.finish();
            dt.fill(&path, &color, &DrawOptions::new());
        }
    }
}

pub fn draw_gradient(palette: &Vec<Color>, dt: &mut DrawTarget) {
    let colors: Vec<&Color> = palette
        .choose_multiple(&mut rand::thread_rng(), 2)
        .collect();

    if colors.len() < 2 {
        eprintln!("Too few colors for gradient effect");
        exit(1);
    }

    let mut pb = PathBuilder::new();
    pb.rect(0., 0., dt.width() as f32, dt.height() as f32);
    let path = pb.finish();

    let gradient = Source::new_linear_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.0,
                    color: **(colors.get(0).expect("Missing Color")),
                },
                GradientStop {
                    position: 1.0,
                    color: **(colors.get(1).expect("Missing Color")),
                },
            ],
        },
        Point::new(0., 0.),
        Point::new(dt.width() as f32, dt.height() as f32),
        Spread::Pad,
    );
    dt.fill(&path, &gradient, &DrawOptions::new());
}

pub fn draw_voronoi(palette: &Vec<Color>, dt: &mut DrawTarget) {
    let rng = &mut rand::thread_rng();
    let range1 = Uniform::new(0., dt.width() as f64);
    let range2 = Uniform::new(0., dt.height() as f64);
    let points: Vec<(f64, f64)> = (0..100)
        .map(|_| (rng.sample(&range1), rng.sample(&range2)))
        .collect();
    let diagram = VoronoiDiagram::<Vpoint>::from_tuple(
        &(0., 0.),
        &(dt.width() as f64, dt.height() as f64),
        &points,
    )
    .unwrap();
    let cells = diagram.cells();
    for c in cells {
        let rng_color: Color;
        match palette.choose(rng) {
            Some(c) => rng_color = *c,
            None => {
                eprintln!("Palette seems to be empty");
                exit(1);
            }
        }
        let mut pb = PathBuilder::new();
        let fp = c.points().first().unwrap();
        pb.move_to(fp.x as f32, fp.y as f32);
        for p in c.points() {
            pb.line_to(p.x as f32, p.y as f32)
        }
        pb.line_to(fp.x as f32, fp.y as f32);
        let path = pb.finish();
        dt.stroke(
            &path,
            &Source::Solid(SolidSource::from(Color::new(0xff, 0x00, 0x00, 0x00))),
            &StrokeStyle::default(),
            &DrawOptions::new(),
        );
        dt.fill(
            &path,
            &Source::Solid(SolidSource::from(rng_color)),
            &DrawOptions::new(),
        );
    }
}
