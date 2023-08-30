use rand::seq::SliceRandom;
use raqote::{
    Color, DrawOptions, DrawTarget, Gradient, GradientStop, PathBuilder, Point, Source, Spread,
};
use std::process::exit;

pub fn render(palette: &Vec<Color>, dt: &mut DrawTarget) {
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
