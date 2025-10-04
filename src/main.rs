use nannou::prelude::*;
use nannou::color::*;

use std::collections::HashMap;


fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let char_hue: HashMap<&str, i32> = HashMap::from([
    ("0", 0),
    ("1", 22),
    ("2", 44),
    ("3", 66),
    ("4", 88),
    ("5", 110),
    ("6", 132),
    ("7", 154),
    ("8", 176),
    ("9", 198),
    ("a", 220),
    ("b", 242),
    ("c", 264),
    ("d", 286),
    ("e", 308),
    ("f", 330),
]);
    draw.background().color(PURPLE);
    let s: &str = "ff42ctv";
    let char_vec: Vec<char> = s.chars().collect();
    for c in char_vec {
        let i:&str=&str::from(c);
        //println!("{}", c);
        //let s = format!(“{}”, c);
        draw.ellipse().color(hsl((char_hue[&c] as f32) / 360.0,1.0,0.5));
    }

    draw.to_frame(app, &frame).unwrap();
}

