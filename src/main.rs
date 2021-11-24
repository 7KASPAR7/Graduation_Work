use std::cmp;

use rand::Rng;
use tcod::colors::*;
use tcod::console::*;

mod config;
use config::*;


struct Tcod {
    root: Root,
    con: Offscreen,
}



fn main() {
    tcod::system::set_fps(config::LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title("Roguelike Game Prototype")
        .init();


    let con = Offscreen::new(config::MAP_WIDTH, config::MAP_HEIGHT);

    let mut tcod = Tcod { root, con };

    while !tcod.root.window_closed() {
        // clear the screen of the previous frame
        tcod.con.clear();
    }

}

