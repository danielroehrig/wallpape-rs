use clap::{Parser, Subcommand};
use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand::{Rng, RngCore};
use raqote::*;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::process::exit;
use voronator::delaunator::Point as Vpoint;
use voronator::VoronoiDiagram;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    dest: String,

    #[arg(long, value_name = "width", help = "Width of the resulting wallpaper")]
    width: Option<i32>,

    #[arg(
        long,
        value_name = "height",
        help = "Height of the resulting wallpaper"
    )]
    height: Option<i32>,

    #[arg(long, value_name = "palette", help = "Colorscheme to use")]
    palette: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List options
    List { list: String },
}

fn main() {
    let palettes = build_palettes();

    let cli = Args::parse();

    if let Some(Commands::List { list }) = cli.command {
        match list.as_str() {
            "palettes" => {
                print_palettes(&palettes);
            }
            _ => {
                eprintln!("Unknown list option \"{}\"", list);
                exit(1);
            }
        };
        exit(0);
    }

    let width = cli.width.unwrap_or(1920);
    let height = cli.height.unwrap_or(1080);
    let color_scheme = cli.palette.unwrap_or(String::from("cyberpunk"));
    let dest = cli.dest;

    let palette = match palettes.get(color_scheme.as_str()) {
        Some(x) => x,
        None => {
            eprintln!("Unknown palette \"{}\"", color_scheme);
            exit(1);
        }
    };

    let mut dt = DrawTarget::new(width, height);

    draw_gradient(palette, &mut dt);
    draw_little_boxes(palette, &mut dt);
    draw_voronoi(palette, &mut dt);

    match dt.write_png(dest.clone()) {
        Ok(_) => {
            println!("Image written to {}", dest);
            exit(0);
        }
        Err(e) => {
            eprintln!("Could not write to file \"{}\"! Error: {}", dest, e);
            exit(1);
        }
    };
}

fn print_palettes(palettes: &HashMap<&str, Vec<Color>>) {
    println!("Available colorschemes:");
    for scheme in palettes.keys() {
        println!("{}", scheme);
    }
}

fn build_palettes() -> HashMap<&'static str, Vec<Color>> {
    HashMap::from([
        (
            "cyberpunk",
            vec![
                Color::new(0xff, 247, 37, 133),
                Color::new(0xff, 114, 9, 183),
                Color::new(0xff, 58, 12, 163),
                Color::new(0xff, 67, 97, 238),
                Color::new(0xff, 76, 201, 240),
            ],
        ),
        (
            "pastel",
            vec![
                Color::new(0xff, 205, 180, 219),
                Color::new(0xff, 255, 200, 221),
                Color::new(0xff, 255, 175, 204),
                Color::new(0xff, 189, 224, 254),
                Color::new(0xff, 162, 210, 255),
            ],
        ),
        (
            "gundam",
            vec![
                Color::new(0xff, 43, 45, 66),
                Color::new(0xff, 141, 153, 174),
                Color::new(0xff, 237, 242, 244),
                Color::new(0xff, 239, 35, 60),
                Color::new(0xff, 217, 4, 41),
            ],
        ),
    ])
}

fn draw_gradient(palette: &Vec<Color>, dt: &mut DrawTarget) {
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

fn draw_little_boxes(palette: &Vec<Color>, dt: &mut DrawTarget) {
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

fn draw_voronoi(palette: &Vec<Color>, dt: &mut DrawTarget) {
    let rng = &mut rand::thread_rng();
    let range1 = Uniform::new(0., dt.width() as f64);
    let range2 = Uniform::new(0., dt.height() as f64);
    let mut points: Vec<(f64, f64)> = (0..100)
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
