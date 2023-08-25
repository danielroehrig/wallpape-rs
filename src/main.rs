use clap::{Parser, Subcommand};
use raqote::*;
use std::process::exit;
use wallpape_rs::effects::*;
use wallpape_rs::palettes::*;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run effect
    Render {
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

        #[arg(long, value_name = "effect", help = "Effect you want to use")]
        effect: Option<String>,
    },
    /// List options
    List { list: String },
}

fn main() {
    let cli = Args::parse();

    match cli.command {
        Some(Commands::List { list }) => {
            match list.as_str() {
                "palettes" => {
                    print_palettes();
                }
                "effects" => {
                    print_effects();
                }
                _ => {
                    eprintln!("Unknown list option \"{}\"", list);
                    exit(1);
                }
            };
            exit(0);
        }
        Some(Commands::Render {
            dest,
            height,
            width,
            palette,
            effect,
        }) => {
            let width = width.unwrap_or(1920);
            let height = height.unwrap_or(1080);
            let color_scheme = palette.unwrap_or(String::from("cyberpunk"));

            let palette = match get_palette(color_scheme.as_str()) {
                Some(x) => x,
                None => {
                    eprintln!("Unknown palette \"{}\"", color_scheme);
                    exit(1);
                }
            };

            let mut dt = DrawTarget::new(width, height);

            // draw_gradient(palette, &mut dt);
            // draw_little_boxes(palette, &mut dt);
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
        None => exit(1),
    }
}

fn print_palettes() {
    println!("Available colorschemes:");
    for scheme in get_palette_names() {
        println!("{}", scheme);
    }
}

fn print_effects() {
    println!("Available effects:");
    for x in Fx::VALUES {
        println!("{}", x.to_s())
    }
}
