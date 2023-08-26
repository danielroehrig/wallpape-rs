use clap::{command, CommandFactory, Parser, Subcommand};
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
    #[command(disable_help_flag = true)]
    Render {
        /// File path to rendered image
        dest: String,

        #[arg(
            short,
            long,
            value_name = "width",
            default_value_t = 1920,
            help = "Width of the resulting wallpaper"
        )]
        width: i32,

        #[arg(
            short,
            long,
            value_name = "height",
            default_value_t = 1080,
            help = "Height of the resulting wallpaper"
        )]
        height: i32,

        #[arg(
            long,
            value_name = "palette",
            help = "Colorscheme to use",
            default_value_t = String::from("cyberpunk"),
        )]
        palette: String,

        #[arg(
            long,
            value_name = "effect",
            help = "Effect you want to use",
            default_value_t = String::from("voronoi"),
        )]
        effect: String,
    },
    /// List options
    #[command(disable_help_flag = true, about = "List possible options")]
    List {
        /// Effects or Palettes
        list: String,
    },
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
            let palette = match get_palette(palette.as_str()) {
                Some(x) => x,
                None => {
                    eprintln!("Unknown palette \"{}\"", palette);
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
        None => {
            let mut cmd = Args::command();
            let _ = cmd.print_help();
            exit(0);
        }
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
