use raqote::*;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, value_name ="width", help = "Width of the resulting wallpaper")]
    width: Option<i32>,

    #[arg(long, value_name ="height", help = "Height of the resulting wallpaper")]
    height: Option<i32>
}

fn main() {
    let cli = Args::parse();

    let width = cli.width.unwrap_or(1920);
    let height = cli.height.unwrap_or(1080);

    let mut dt = DrawTarget::new(width, height);

    let mut pb = PathBuilder::new();
    pb.rect(0., 0., dt.width() as f32, dt.height() as f32);
    let path = pb.finish();

    let gradient = Source::new_linear_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.0,
                    color: (
                        Color::new(0xff, 0xcc, 0x00, 0x00)
                        ),
                },
                GradientStop {
                    position: 1.0,
                    color: (
                        Color::new(0xff, 0xcc, 0xcc, 0xff)
                        ),
                }
            ]
        }, Point::new(0., 0.),
        Point::new(dt.width() as f32, dt.height() as f32), Spread::Pad);
    dt.fill(&path, &gradient, &DrawOptions::new());


    dt.write_png("example.png").unwrap();
}