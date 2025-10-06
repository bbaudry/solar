use nannou::prelude::*;
use nannou::color::*;
use std::str;
use std::collections::HashMap;
use std::env;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_ntimes(1))
        //.update(update)
        .view(view)
        //.simple_window(view)
        .run();
}

struct Model {
    commit_hash: String,
    w:u32,
    h:u32,
    stripe_w:u32
}

fn model(app: &App) -> Model {
    let args: Vec<String> = env::args().collect();
    let h=&args[1];
    let stripe_w:u32=99;
    let canvas_w:u32=u32::try_from(h.len()).unwrap()*stripe_w;
    let canvas_h:u32=canvas_w as f32/1.6; // height and width have a golden ratio
    let stripe_h:f32=stripe_h as u32;
    dbg!(h);
    app.new_window()
    .size(canvas_w,canvas_h)
    .build()
    .unwrap();
    Model{
        commit_hash:String::from(h),
        w:canvas_w,
        h:canvas_h,
        stripe_w:stripe_w,
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
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
    let s: &str = model.commit_hash.as_str();
    // get the characters from the hash
    let char_vec: Vec<char> = s.chars().collect();
    let mut x=- (model.w as f32/2.0) + model.stripe_w as f32 / 2.0;
    for c in char_vec {
        // convert the character into a str: https://www.reddit.com/r/rust/comments/eanwkm/how_to_create_a_str_from_char/
        let i=c.to_string();
        let j=i.as_str();

        draw.rect()
            .color(hsl((char_hue[j] as f32) / 360.0,1.0,0.5))
            .w_h(model.stripe_w as f32,model.h as f32)
            .x_y(x,0.0);
        x=x+model.stripe_w as f32;
    }

    draw.to_frame(app, &frame).unwrap();
}

