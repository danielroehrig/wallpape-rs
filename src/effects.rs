use rand::seq::SliceRandom;
use rand::RngCore;
use raqote::{Color, DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};
use std::cmp::{max, min};
use std::process::exit;

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
