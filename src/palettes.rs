use raqote::Color;
use std::collections::HashMap;

lazy_static! {
    static ref PALETTES: HashMap<&'static str, Vec<Color>> = {
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
    };
}

pub fn get_palette(name: &str) -> Option<&Vec<Color>> {
    PALETTES.get(name)
}

pub fn get_palette_names() -> Vec<&'static str> {
    PALETTES.clone().into_keys().collect()
}
