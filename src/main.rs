use raqote::*;
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::process::exit;
use rand::seq::SliceRandom;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, value_name = "width", help = "Width of the resulting wallpaper")]
    width: Option<i32>,

    #[arg(long, value_name = "height", help = "Height of the resulting wallpaper")]
    height: Option<i32>,

    #[arg(long, value_name = "palette", help = "Colorscheme to use")]
    palette: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List options
    List {
        list: String
    }
}

fn main() {
    let palettes = build_palettes();

    let cli = Args::parse();

    match cli.command {
        Some(Commands::List { list }) => {
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
        None => {}
    }

    let width = cli.width.unwrap_or(1920);
    let height = cli.height.unwrap_or(1080);
    let color_scheme = cli.palette.unwrap_or(String::from("cyberpunk"));

    let palette = match palettes.get(color_scheme.as_str()) {
        Some(x) => x,
        None => {
            eprintln!("Unknown palette \"{}\"", color_scheme);
            exit(1);
        }
    };

    let mut dt = DrawTarget::new(width, height);

    draw_gradient(palette, &mut dt);


    dt.write_png("example.png").unwrap();
}

fn print_palettes(palettes: &HashMap<&str, Vec<Color>>) {
    println!("Available colorschemes:");
    for scheme in palettes.keys() {
        println!("{}", scheme);
    }
}

fn build_palettes() -> HashMap<&'static str, Vec<Color>> {
    HashMap::from(
        [
            ("cyberpunk",
             vec![
                 Color::new(0xff, 247, 37, 133),
                 Color::new(0xff, 114, 9, 183),
                 Color::new(0xff, 58, 12, 163),
                 Color::new(0xff, 67, 97, 238),
                 Color::new(0xff, 76, 201, 240),
             ]),
            ("pastel",
             vec![
                 Color::new(0xff, 205, 180, 219),
                 Color::new(0xff, 255, 200, 221),
                 Color::new(0xff, 255, 175, 204),
                 Color::new(0xff, 189, 224, 254),
                 Color::new(0xff, 162, 210, 255),
             ]),
            ("gundam",
             vec![
                 Color::new(0xff, 43, 45, 66),
                 Color::new(0xff, 141, 153, 174),
                 Color::new(0xff, 237, 242, 244),
                 Color::new(0xff, 239, 35, 60),
                 Color::new(0xff, 217, 4, 41),
             ])
        ]
    )
}

fn draw_gradient(palette: &Vec<Color>, dt: &mut DrawTarget) {
    let colors: Vec<&Color> = palette.choose_multiple(&mut rand::thread_rng(), 2).collect();

    let mut pb = PathBuilder::new();
    pb.rect(0., 0., dt.width() as f32, dt.height() as f32);
    let path = pb.finish();

    let gradient = Source::new_linear_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.0,
                    color: **(
                        colors.get(0).unwrap()
                    ),
                },
                GradientStop {
                    position: 1.0,
                    color: **(
                        colors.get(1).unwrap()
                    ),
                },
            ]
        }, Point::new(0., 0.),
        Point::new(dt.width() as f32, dt.height() as f32), Spread::Pad);
    dt.fill(&path, &gradient, &DrawOptions::new());
}