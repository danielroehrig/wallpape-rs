use raqote::*;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, value_name = "width", help = "Width of the resulting wallpaper")]
    width: Option<i32>,

    #[arg(long, value_name = "height", help = "Height of the resulting wallpaper")]
    height: Option<i32>,

    #[arg(long, value_name = "palette", help = "Colorscheme to use")]
    palette: Option<String>,
}

fn main() {
    let palettes = HashMap::from(
        [
            ("cyberpunk",
             vec![
                 Color::new(0xff, 0, 255, 159),
                 Color::new(0xff, 0, 184, 255),
             ])
        ]
    );


    let cli = Args::parse();

    let width = cli.width.unwrap_or(1920);
    let height = cli.height.unwrap_or(1080);
    let colorsheme = cli.palette.unwrap_or(String::from("cyberpunk"));

    let pallete  = palettes.get(colorsheme.as_str()).unwrap();

    let mut dt = DrawTarget::new(width, height);

    let mut pb = PathBuilder::new();
    pb.rect(0., 0., dt.width() as f32, dt.height() as f32);
    let path = pb.finish();

    let gradient = Source::new_linear_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.0,
                    color: *(
                        pallete.get(0).unwrap()
                    ),
                },
                GradientStop {
                    position: 1.0,
                    color: *(
                        pallete.get(1).unwrap()
                    ),
                },
            ]
        }, Point::new(0., 0.),
        Point::new(dt.width() as f32, dt.height() as f32), Spread::Pad);
    dt.fill(&path, &gradient, &DrawOptions::new());


    dt.write_png("example.png").unwrap();
}