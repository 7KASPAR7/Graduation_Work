use std::cmp;

use rand::Rng;
use tcod::colors::*;
use tcod::console::*;

use tcod::map::{Map as FovMap};

mod config;
mod structures;

fn spawn_objects(room: structures::Rect, map: &structures::Map, objects: &mut Vec<structures::Object>) {
    let monster_num = rand::thread_rng().gen_range(0..config::MAX_ROOM_MONSTERS + 1);
    
    for _ in 0..monster_num {
        let x = rand::thread_rng().gen_range(room.x1..room.x2+1);
        let y = rand::thread_rng().gen_range(room.y1..room.y2+1);
        if !is_blocked(x, y, map, objects) {
            let mut monster;
            if rand::random::<f32>() < 0.75 {
                monster = structures::Object::new(x, y, 'O', DESATURATED_PURPLE, "Orc", true);
                monster.attackable = Some(structures::Attackable{max_hp: 50, hp: 50, armor: 5, damage: 12, on_death: structures::DeathCallback::Monster})
            } 
            else {
                monster = structures::Object::new(x, y, 'T', DESATURATED_ORANGE, "Troll", true);
                monster.attackable = Some(structures::Attackable{max_hp: 30, hp: 30, armor: 2, damage: 15, on_death: structures::DeathCallback::Monster})
            }
            monster.alive = true;
            monster.ai = Some(structures::Ai::Basic);
            objects.push(monster);
        }
    }

    let num_items = rand::thread_rng().gen_range(0..config::MAX_ROOM_ITEMS + 1);

    for _ in 0..num_items {
        // choose random spot for this item
        let x = rand::thread_rng().gen_range(room.x1 + 1..room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1..room.y2);

    // only place it if the tile is not blocked
    if !is_blocked(x, y, map, objects) {
        // create a healing potion
        let mut item = structures::Object::new(x, y, '!',  VIOLET, "healing potion", false);
        item.item = Some(structures::Item::Heal);
        objects.push(item);
    }
}

}

/// add to the player's inventory and remove from the map
fn pick_item_up(object_id: usize, game: &mut structures::Game, objects: &mut Vec<structures::Object>) {
    if game.inventory.len() >= 26 {
        game.messages.add(
            format!(
                "Your inventory is full, cannot pick up {}.",
                objects[object_id].name
            ),
            RED,
        );
    } else {
        let item = objects.swap_remove(object_id);
        game.messages
            .add(format!("You picked up a {}!", item.name), GREEN);
        game.inventory.push(item);
    }
}

fn is_blocked(x: i32, y: i32, map: &structures::Map, objects: &[structures::Object]) -> bool {
    if map[x as usize][y as usize].collision_enabled {
        return true;
    }
    objects.iter().any(|object| object.blocks && object.loc() == (x,y))
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

fn menu<T: AsRef<str>>(header: &str, options: &[T], width: i32, root: &mut Root) -> Option<usize> {
    assert!(
        options.len() <= 26,
        "Cannot have a menu with more than 26 options."
    );

    // calculate total height for the header (after auto-wrap) and one line per option
    let header_height = root.get_height_rect(0, 0, width, config::SCREEN_HEIGHT, header);
    let height = options.len() as i32 + header_height;

    // create an off-screen console that represents the menu's window
    let mut window = Offscreen::new(width, height);

    // print the header, with auto-wrap
    window.set_default_foreground(WHITE);
    window.print_rect_ex(
        0,
        0,
        width,
        height,
        BackgroundFlag::None,
        TextAlignment::Left,
        header,
    );

    // print all the options
    for (index, option_text) in options.iter().enumerate() {
        let menu_letter = (b'a' + index as u8) as char;
        let text = format!("({}) {}", menu_letter, option_text.as_ref());
        window.print_ex(
            0,
            header_height + index as i32,
            BackgroundFlag::None,
            TextAlignment::Left,
            text,
        );
    }

    // blit the contents of "window" to the root console
    let x = config::SCREEN_WIDTH / 2 - width / 2;
    let y = config::SCREEN_HEIGHT / 2 - height / 2;
    blit(&window, (0, 0), (width, height), root, (x, y), 1.0, 0.7);

    // present the root console to the player and wait for a key-press
    root.flush();
    let key = root.wait_for_keypress(true);

    // convert the ASCII code to an index; if it corresponds to an option, return it
    if key.printable.is_alphabetic() {
        let index = key.printable.to_ascii_lowercase() as usize - 'a' as usize;
        if index < options.len() {
            Some(index)
        } else {
            None
        }
    } else {
        None
    }
}

fn inventory_menu(inventory: &[structures::Object], header: &str, root: &mut Root) -> Option<usize> {
    let options = if inventory.len() == 0 {
        vec!["Inventory is empty.".into()]
    } else {
        inventory.iter().map(|item| item.name.clone()).collect()
    };

    let inventory_index = menu(header, &options, config::INVENTORY_WIDTH, root);

    if inventory.len() > 0 {
        inventory_index
    } else {
        None
    }
}


fn cast_heal(_inventory_id: usize, _tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object]) -> structures::UseResult {
    if let Some(attackable) = objects[config::PLAYER].attackable {
        if attackable.hp == attackable.max_hp {
            game.messages.add("You don't need a heal potion.", RED);
            return structures::UseResult::Cancelled;
        }

        game.messages.add(format!("You was healed by {}!", config::HEAL_AMOUNT.to_string()), LIGHT_YELLOW);
        objects[config::PLAYER].heal(config::HEAL_AMOUNT);
        return structures::UseResult::UsedUp;
    }
    structures::UseResult::Cancelled
}


fn use_item(inventory_id: usize, tcod: &mut structures::Tcod, game: &mut  structures::Game, objects: &mut [ structures::Object]) {
    use structures::Item::*;
    if let Some(item) = game.inventory[inventory_id].item {
        let on_use = match item {
            Heal => cast_heal,
        };
        match on_use(inventory_id, tcod, game, objects) {
            structures::UseResult::UsedUp => {
                game.inventory.remove(inventory_id);
            }
            structures::UseResult::Cancelled => {
                game.messages.add("Cancelled", WHITE);
            }
        }
    } else {
        game.messages.add(format!("The {} can't be used.", game.inventory[inventory_id].name), WHITE);
    }
}


fn generate_map(objects: &mut Vec<structures::Object>) -> structures::Map {

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

            spawn_objects(new_room, &map, objects);

            let (new_x, new_y) = new_room.center();
            // println!("new {}, {}", new_x, new_y);

            if rooms.is_empty() {
                objects[config::PLAYER].set_loc(new_x, new_y);
            }  
            else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                // println!("prev {}, {}", prev_x, prev_y);

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

fn render(tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &[structures::Object], fov_recompute: bool) {

    if fov_recompute {
        let player = &objects[config::PLAYER];
        tcod.fov.compute_fov(player.x, player.y, config::FOV_RADIUS, config::FOV_LIGHT_WALLS, config::FOV_ALG);
    }

    for y in 0..config::MAP_HEIGHT {
        for x in 0..config::MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].is_visible;
            let color = match (visible, wall) {
                (false, true) => config::COLOR_DARK_WALL,
                (false, false) => config::COLOR_DARK_GROUND,

                (true, true) => config::COLOR_LIGHT_WALL,
                (true, false) => config::COLOR_LIGHT_GROUND,
            };

            let explored = &mut game.map[x as usize][y as usize].is_explored;
            if visible {
                *explored = true;
            }
            if *explored {
                tcod.screen
                    .set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }
    let mut to_draw: Vec<_> = objects.iter().collect();
    to_draw.sort_by(|o1, o2| { o1.blocks.cmp(&o2.blocks) });
    for object in &to_draw {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.screen);
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

    
    tcod.root.set_default_foreground(WHITE);
    if let Some(attackable) = objects[config::PLAYER].attackable {
        tcod.root.print_ex(
            1,
            config::SCREEN_HEIGHT - 2,
            BackgroundFlag::None,
            TextAlignment::Left,
            format!("HP: {}/{} ", attackable.hp, attackable.max_hp),
        );
    }
    
    tcod.panel.set_default_background(BLACK);
    tcod.panel.clear();

    let mut y = config::MESSAGES_HEIGHT as i32;
    for &(ref msg, color) in game.messages.iter().rev() {
        let msg_height = tcod.panel.get_height_rect(config::MESSAGES_X, y, config::MESSAGES_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        tcod.panel.set_default_foreground(color);
        tcod.panel.print_rect(config::MESSAGES_X, y, config::MESSAGES_WIDTH, 0, msg);
    }

    let hp = objects[config::PLAYER].attackable.map_or(0, |f| f.hp);
    let max_hp = objects[config::PLAYER].attackable.map_or(0, |f| f.max_hp);
    render_bar(&mut tcod.panel, 1, 1, config::BAR_WIDTH, "HP", hp, max_hp, LIGHT_RED, DARKER_RED);

    blit(
        &tcod.panel,
        (0, 0),
        (config::SCREEN_WIDTH, config::PANEL_HEIGHT),
        &mut tcod.root,
        (0, config::PANEL_Y),
        1.0,
        1.0,
    );

}

fn move_by(id: usize, dx: i32, dy: i32, map: &structures::Map, objects: &mut [structures::Object]) {
    let (x, y) = objects[id].loc();
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_loc(x + dx, y + dy);
    }
}

fn handle_keys(tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &mut Vec<structures::Object>) -> structures::PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use structures::PlayerAction::*;

    let key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[config::PLAYER].alive;

    match (key, key.text(), player_alive) {

        (Key { code: Escape, .. }, _, _) => Exit,
        
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(0, -1, game, objects);
            TookTurn
        },
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(0, 1, game, objects);
            TookTurn
        },
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(-1, 0, game, objects);
            TookTurn
        },
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(1, 0, game, objects);
            TookTurn
        },
        
        (Key { code: Number1, .. }, _, true) => {
            //println!("Tried to pick up");
            let item_id = objects
                .iter()
                .position(|object| object.loc() == objects[config::PLAYER].loc() && object.item.is_some());
            if let Some(item_id) = item_id {
                pick_item_up(item_id, game, objects);
            }
            DidnotTakeTurn
        },

        (Key { code: Number2, .. }, _, true) => {
            let inventory_index = inventory_menu(
                &game.inventory,
                "Press the key to use an item or any other to cancel.\n",
                &mut tcod.root,
            );
            if let Some(inventory_index) = inventory_index {
                use_item(inventory_index, tcod, game, objects);
            }
            DidnotTakeTurn
        }

        _ => DidnotTakeTurn,
    }
    
}
    

fn player_move_or_attack(dx: i32, dy: i32, game: &mut structures::Game, objects: &mut [structures::Object]) {
    // the coordinates the player is moving to/attacking
    let x = objects[config::PLAYER].x + dx;
    let y = objects[config::PLAYER].y + dy;

    // try to find an attackable object there
    let target_id = objects.iter().position(|object| object.attackable.is_some() && object.loc() == (x, y));

    // attack if target found, move otherwise
    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(config::PLAYER, target_id, objects);
            player.attack(target, game);
        }
        None => {
            move_by(config::PLAYER, dx, dy, &game.map, objects);
        }
    }
}

fn monster_move(id: usize, player_x: i32, player_y: i32, map: &structures::Map, objects: &mut [structures::Object]) {
    let dx = player_x - objects[id].x;
    let dy = player_y - objects[id].y;
    let distance = ((dx*dx + dy*dy) as f32).sqrt();

    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;

    move_by(id, dx, dy, map, objects);
}

fn ai_turn(id: usize, tcod: &structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object]) {
    let (monster_x, monster_y) = objects[id].loc();
    if tcod.fov.is_in_fov(monster_x, monster_y) {
        if objects[id].get_distance_to(&objects[config::PLAYER]) >= 2.0 {
            let (player_x, player_y) = objects[config::PLAYER].loc();
            monster_move(id, player_x, player_y, &game.map, objects);
        } 
        else if objects[config::PLAYER].attackable.map_or(false, |a| a.hp > 0) {
            let (monster, player) = mut_two(id, config::PLAYER, objects);
            monster.attack(player, game);
        }
    }
}

fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

fn render_bar(panel: &mut Offscreen, x: i32, y: i32, total_width: i32, name: &str, value: i32, maximum: i32, bar_color: Color, back_color: Color) {
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

    panel.set_default_background(back_color);
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }

    panel.set_default_foreground(WHITE);
    panel.print_ex(x + total_width / 2, y, BackgroundFlag::None, TextAlignment::Center, &format!("{}: {}/{}", name, value, maximum));
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
        panel: Offscreen::new(config::SCREEN_WIDTH, config::PANEL_HEIGHT),
    };

    let mut player = structures::Object::new(5, 5, '@', BLUE, "Player", true);

    player.alive = true;
    player.attackable = Some(structures::Attackable{max_hp: 100, hp: 100, armor: 10, damage: 10, on_death: structures::DeathCallback::Player});

    let mut objects = vec![player];

    let mut game = structures::Game {
        map: generate_map(&mut objects),
        messages: structures::Messages::new(),
        inventory: vec![],
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

    game.messages.add("Welcome to Dungeon. Prepare for danger!", RED);

    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {
        
        tcod.screen.clear();

        let fov_recompute = previous_player_position != (objects[config::PLAYER].x, objects[config::PLAYER].y);
        render(&mut tcod, &mut game, &objects, fov_recompute);
        tcod.root.flush();

        previous_player_position = objects[config::PLAYER].loc();
        let player_action = handle_keys(&mut tcod, &mut game, &mut objects);
        if player_action == structures::PlayerAction::Exit {
            break;
        }
        if objects[config::PLAYER].alive && player_action != structures::PlayerAction::DidnotTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_turn(id, &tcod, &mut game, &mut objects);
                }
            }
        }
    }
}

