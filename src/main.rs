use nannou::prelude::*;
use nannou::color::*;
use std::str;
use std::collections::HashMap;


fn main() {
    nannou::sketch(view).size(420, 200).run()
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
    let s: &str = "fa42c98";
    // get the characters from the hash
    let char_vec: Vec<char> = s.chars().collect();
    let mut x=-180.0;
    for c in char_vec {
        // convert the character into a str: https://www.reddit.com/r/rust/comments/eanwkm/how_to_create_a_str_from_char/
        let i=c.to_string();
        let j=i.as_str();

        draw.rect()
            .color(hsl((char_hue[j] as f32) / 360.0,1.0,0.5))
            .w_h(60.0,200.0)
            .x_y(x,0.0);
        x=x+60.0;
    }

    draw.to_frame(app, &frame).unwrap();
}

