use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand::Rng;
use raqote::{Color, DrawOptions, DrawTarget, PathBuilder, SolidSource, Source, StrokeStyle};
use std::process::exit;
use voronator::delaunator::Point as Vpoint;
use voronator::VoronoiDiagram;

pub fn render(palette: &Vec<Color>, dt: &mut DrawTarget) {
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
