use std::cmp;

use rand::Rng;
use tcod::colors::*;
use tcod::console::*;

mod config;
use config::*;


struct Tcod {
    root: Root,
    screen: Offscreen,
}

// all map is only tiles
struct Tile {
    collision_enabled: bool,
    is_visible: bool,
}

impl Tile{
    fn empty() -> Self {
        // we can not collide and see the empty tile
        Tile {
            collision_enabled: false,
            is_visible: false,
        }
    }

    fn wall() -> Self {
        // we can collide and see the wall
        Tile {
            collision_enabled: true,
            is_visible: true,
        }
    }
}

// map is 2-dimension list of tiles
type Map = Vec<Vec<Tile>>;


fn handle_keys(tcod: &mut Tcod) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        // Key {
        //     code: Enter,
        //     alt: true,
        //     ..
        // } => {
        //     // Alt+Enter: toggle fullscreen
        //     let fullscreen = tcod.root.is_fullscreen();
        //     tcod.root.set_fullscreen(!fullscreen);
        // }
        Key { code: Escape, .. } => return true, // exit game

        // // movement keys
        // Key { code: Up, .. } => player.move_by(0, -1, game),
        // Key { code: Down, .. } => player.move_by(0, 1, game),
        // Key { code: Left, .. } => player.move_by(-1, 0, game),
        // Key { code: Right, .. } => player.move_by(1, 0, game),

        _ => {}
    }
    false
}
    

fn main() {
    tcod::system::set_fps(config::LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title("Roguelike Game Prototype")
        .init();


    let screen = Offscreen::new(config::MAP_WIDTH, config::MAP_HEIGHT);

    let mut tcod = Tcod {root, screen};

    while !tcod.root.window_closed() {

        let exit = handle_keys(&mut tcod);

        if exit {
            break;
        }
    }

}

