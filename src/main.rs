use std::cmp;

use rand::Rng;
use tcod::colors::*;
use tcod::console::*;

use tcod::map::{Map as FovMap};

mod config;
mod structures;

fn spawn_units(room: structures::Rect, map: &structures::Map, units: &mut Vec<structures::Unit>) {
    let monster_num = rand::thread_rng().gen_range(0..config::MAX_ROOM_MONSTERS + 1);
    
    for _ in 0..monster_num {
        let x = rand::thread_rng().gen_range(room.x1..room.x2+1);
        let y = rand::thread_rng().gen_range(room.y1..room.y2+1);
        if !is_blocked(x, y, map, units) {
            let mut monster;
            if rand::random::<f32>() < 0.75 {
                monster = structures::Unit::new(x, y, 'O', DESATURATED_PURPLE, "Orc", true);
            } 
            else {
                monster = structures::Unit::new(x, y, 'T', DESATURATED_ORANGE, "Troll", true);
            }
            monster.alive = true;
            units.push(monster);
        }
    }

}

fn is_blocked(x: i32, y: i32, map: &structures::Map, units: &[structures::Unit]) -> bool {
    if map[x as usize][y as usize].collision_enabled {
        return true;
    }
    units.iter().any(|unit| unit.blocks && unit.loc() == (x,y))
}

fn create_room(room: structures::Rect, map: &mut structures::Map) {
    for x in (room.x1 + 1).. room.x2 {
        for y in (room.y1 + 1).. room.y2 {
            map[x as usize][y as usize] = structures::Tile::empty();
        }
    }
}

fn create_hor_tunnel(x1: i32, x2: i32, y: i32, map: &mut structures::Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = structures::Tile::empty();
    }
}

fn create_ver_tunnel(x: i32, y1: i32, y2: i32, map: &mut structures::Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = structures::Tile::empty();
    }
}

fn generate_map(units: &mut Vec<structures::Unit>) -> structures::Map {

    let mut map = vec![vec![structures::Tile::wall(); config::MAP_HEIGHT as usize]; config::MAP_WIDTH as usize];
    
    let mut rooms = vec![];

    for _ in 0..config::MAX_ROOMS {

        let width = rand::thread_rng().gen_range(config::ROOM_MIN_SIZE..config::ROOM_MAX_SIZE + 1);
        let height = rand::thread_rng().gen_range(config::ROOM_MIN_SIZE..config::ROOM_MAX_SIZE + 1);

        let x = rand::thread_rng().gen_range(0..config::MAP_WIDTH - width);
        let y = rand::thread_rng().gen_range(0..config::MAP_HEIGHT - height);

        let new_room = structures::Rect::new(x, y, width, height);

        let failed = rooms.iter().any(|other_room| new_room.is_intersected_with(other_room));

        if !failed {

            create_room(new_room, &mut map);

            spawn_units(new_room, &map, units);

            // center coordinates of the new room
            let (new_x, new_y) = new_room.center();
            println!("new {}, {}", new_x, new_y);

            if rooms.is_empty() {
                units[config::PLAYER].set_loc(new_x, new_y);
            }  
            else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                println!("prev {}, {}", prev_x, prev_y);

                if rand::random() {
                    create_hor_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_ver_tunnel(new_x, prev_y, new_y,  &mut map);
                } else {
                    create_ver_tunnel(prev_x, prev_y, new_y, &mut map);
                    create_hor_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }
    }

    map
}

fn render(tcod: &mut structures::Tcod, game: &mut structures::Game, units: &[structures::Unit], fov_recompute: bool) {

    if fov_recompute {
        // recompute FOV if needed (the player moved or something)
        let player = &units[config::PLAYER];
        tcod.fov.compute_fov(player.x, player.y, config::FOV_RADIUS, config::FOV_LIGHT_WALLS, config::FOV_ALG);
    }

    for y in 0..config::MAP_HEIGHT {
        for x in 0..config::MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].is_visible;
            let color = match (visible, wall) {
                // outside of field of view:
                (false, true) => config::COLOR_DARK_WALL,
                (false, false) => config::COLOR_DARK_GROUND,
                // inside fov:
                (true, true) => config::COLOR_LIGHT_WALL,
                (true, false) => config::COLOR_LIGHT_GROUND,
            };

            let explored = &mut game.map[x as usize][y as usize].is_explored;
            if visible {
                // since it's visible, explore it
                *explored = true;
            }
            if *explored {
                // show explored tiles only (any visible tile is explored already)
                tcod.screen
                    .set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

   

    for unit in units {
        if tcod.fov.is_in_fov(unit.x, unit.y){
            unit.draw(&mut tcod.screen);
        }
    }

    blit(
        &tcod.screen,
        (0, 0),
        (config::MAP_WIDTH, config::MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn move_by(id: usize, dx: i32, dy: i32, map: &structures::Map, units: &mut [structures::Unit]) {
    let (x, y) = units[id].loc();
    if !is_blocked(x + dx, y + dy, map, units) {
        units[id].set_loc(x + dx, y + dy);
    }
}

fn handle_keys(tcod: &mut structures::Tcod, game: &structures::Game, units: &mut [structures::Unit]) -> structures::PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use structures::PlayerAction::*;

    let key = tcod.root.wait_for_keypress(true);
    let player_alive = units[config::PLAYER].alive;

    match (key, key.text(), player_alive) {

        (Key { code: Escape, .. }, _, _) => Exit, // exit game
        
        // Write in the {} some logic for player and npc moving
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(0, -1, game, units);
            TookTurn
        },
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(0, 1, game, units);
            TookTurn
        },
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(-1, 0, game, units);
            TookTurn
        },
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(1, 0, game, units);
            TookTurn
        },

        _ => DidnotTakeTurn,
    }
    
}
    

fn player_move_or_attack(dx: i32, dy: i32, game: &structures::Game, units: &mut [structures::Unit]) {
    // the coordinates the player is moving to/attacking
    let x = units[config::PLAYER].x + dx;
    let y = units[config::PLAYER].y + dy;

    // try to find an attackable object there
    let target_id = units.iter().position(|unit| unit.loc() == (x, y));

    // attack if target found, move otherwise
    match target_id {
        Some(target_id) => {
            println!(
                "You tried to attack {}!",
                units[target_id].name
            );
        }
        None => {
            move_by(config::PLAYER, dx, dy, &game.map, units);
        }
    }
}


fn main() {
    tcod::system::set_fps(config::LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title("Roguelike Game Prototype")
        .init();


    //let screen = Offscreen::new(config::MAP_WIDTH, config::MAP_HEIGHT);
        
    let mut tcod = structures::Tcod {
        root,
        screen: Offscreen::new(config::MAP_WIDTH, config::MAP_HEIGHT),  
        fov: FovMap::new(config::MAP_WIDTH, config::MAP_HEIGHT),  
    };

    let mut player = structures::Unit::new(5, 5, '@', BLUE, "Player", true);

    player.alive = true;

    let mut units = vec![player]; // don't forget to change the number of units in generate_map() and handle_keys() definitions

    let mut game = structures::Game {
        map: generate_map(&mut units),
    };
    for y in 0..config::MAP_HEIGHT {
        for x in 0..config::MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].is_visible,
                !game.map[x as usize][y as usize].collision_enabled,
            );
        }
    }

    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {
        
        tcod.screen.clear();

        let fov_recompute = previous_player_position != (units[config::PLAYER].x, units[config::PLAYER].y);
        render(&mut tcod, &mut game, &units, fov_recompute);
        tcod.root.flush();

        previous_player_position = units[config::PLAYER].loc();
        let player_action = handle_keys(&mut tcod, &game, &mut units);
        if player_action == structures::PlayerAction::Exit {
            break;
        }
        if units[config::PLAYER].alive && player_action != structures::PlayerAction::DidnotTakeTurn {
            for unit in &units {
                // only if unitt is not player
                if (unit as *const _) != (&units[config::PLAYER] as *const _) {
                    println!("it is the {}'s turn!", unit.name);
                }
            }
        }
    }
}

