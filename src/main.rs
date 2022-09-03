#![windows_subsystem = "windows"]

mod load_text;
use std::fs::File;

use load_text::get_text_from_file;

use rand::{seq::SliceRandom, thread_rng};
use raylib::prelude::*;

/* Variables */
const WINDOW_SIZE: (i32, i32) = (600, 300);
const FONT_SIZE: f32 = 26.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_SIZE.0, WINDOW_SIZE.1)
        .title("")
        .build();

    /* Get text-files */
    let files: Vec<File> = vec![
        File::open("assets/wordlist/first.txt").expect("Couldn't open file"),
        File::open("assets/wordlist/second.txt").expect("Couldn't open file"),
        File::open("assets/wordlist/third.txt").expect("Couldn't open file"),
    ];

    let mut text = get_text_from_file(files);

    /* Generate compliment */
    let mut compliment: String = String::from("[ press space to recieve words of wisdom ]");
    let mut rng = thread_rng();

    /* Image stuff */
    let icon: Image = Image::load_image("assets/images/icon.png").unwrap();
    let bg_img = Image::load_image("assets/images/bg.png").unwrap();
    let bg_tex = rl.load_texture_from_image(&thread, &bg_img).unwrap();

    /* Set stuff */
    rl.set_window_icon(&icon);
    rl.set_target_fps(60);
    let fle: FontLoadEx = FontLoadEx::Default(255);
    let font_1 = rl
        .load_font_ex(&thread, "assets/comicz.ttf", FONT_SIZE as i32, fle)
        .unwrap();

    /* Game loop */
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_texture(&bg_tex, 0, 0, Color::WHITE);

        let txt_l = measure_text_ex(&font_1, &compliment, FONT_SIZE, 0.0);
        let txt_1_pos: Vector2 = Vector2 {
            x: (WINDOW_SIZE.0 as f32) / 2.0 - (txt_l.x / 2.0),
            y: 120.0,
        };

        d.draw_text_ex(
            &font_1,
            &compliment,
            &txt_1_pos,
            FONT_SIZE,
            0.0,
            Color::LIGHTGRAY,
        );

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            text[0].shuffle(&mut rng);
            text[1].shuffle(&mut rng);
            text[2].shuffle(&mut rng);
            compliment = format!("{} {} {}", text[0][0], text[1][0], text[2][0]);
        }
    }
}
