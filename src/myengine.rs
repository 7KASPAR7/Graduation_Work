use tcod::colors::*;

use std::cmp;
use rand::Rng;
use tcod::console::*;

use std::env;
use std::fs::File;
use std::io::Write;

use tcod::map::{Map as FovMap};

use crate::skills;
use crate::structures;
use crate::config as config; // Change for other game
use crate::editor;

use std::path::Path;

extern crate ajson;


pub fn create_room(room: structures::Rect, map: &mut structures::Map) {
    for x in (room.x1 + 1).. room.x2 {
        for y in (room.y1 + 1).. room.y2 {
            map[x as usize][y as usize] = structures::Tile::empty();
        }
    }
}

pub fn pick_item_up(object_id: usize, game: &mut structures::Game, objects: &mut Vec<structures::Object>) {
    if game.inventory.len() >= 26 {
        game.messages.add(
            format!("Your inventory is full, cannot pick up {}.", objects[object_id].name), RED);
    } else {
        let item = objects.swap_remove(object_id);
        game.messages.add(format!("You picked up a {}!", item.name), GREEN);
        game.inventory.push(item);
    }
}

pub fn use_item(inventory_id: usize, tcod: &mut structures::Tcod, game: &mut  structures::Game, objects: &mut [ structures::Object]) {
    use structures::Item::*;
    if let Some(item) = game.inventory[inventory_id].item {
        let on_use = match item {
            Heal => skills::cast_heal,
            Fire => skills::cast_fire,
            DoubleDamage => skills::cast_dd,
            Blind => skills::cast_blind,
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


pub fn render(tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &[structures::Object], fov_recompute: bool) {

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
    let mut to_draw: Vec<_> = objects.iter().filter(|o| {tcod.fov.is_in_fov(o.x, o.y) || (o.always_visible && game.map[o.x as usize][o.y as usize].is_explored)}).collect();
    to_draw.sort_by(|o1, o2| { o1.blocks.cmp(&o2.blocks) });
    for object in &to_draw {
        object.draw(&mut tcod.screen);
    }

    blit(&tcod.screen, (0, 0), (config::MAP_WIDTH, config::MAP_HEIGHT), &mut tcod.root, (0, 0), 1.0, 1.0);

    
    tcod.root.set_default_foreground(WHITE);
    if let Some(attackable) = objects[config::PLAYER].attackable {
        tcod.root.print_ex(1, config::SCREEN_HEIGHT - 2, BackgroundFlag::None, TextAlignment::Left, format!("HP: {}/{} ", attackable.hp, attackable.max_hp));
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

    tcod.panel.print_ex(1, 3, BackgroundFlag::None, TextAlignment::Left, format!("Dungeon level: {}", game.level));

    blit(&tcod.panel, (0, 0), (config::SCREEN_WIDTH, config::PANEL_HEIGHT), &mut tcod.root, (0, config::PANEL_Y), 1.0, 1.0);

}

pub fn render_bar(panel: &mut Offscreen, x: i32, y: i32, total_width: i32, name: &str, value: i32, maximum: i32, bar_color: Color, back_color: Color) {
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

pub fn generate_map(objects: &mut Vec<structures::Object>) -> structures::Map {

    let mut map = vec![vec![structures::Tile::wall(); config::MAP_HEIGHT as usize]; config::MAP_WIDTH as usize];

    assert_eq!(&objects[config::PLAYER] as *const _, &objects[0] as *const _);
    objects.truncate(1);
    
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

    let (last_room_x, last_room_y) = rooms[rooms.len() - 1].center();
    let mut door = structures::Object::new(last_room_x, last_room_y, '<', WHITE, "door", false);
    door.always_visible = true;
    objects.push(door);

    map
}

fn get_config() ->  Vec<structures::MonsterConfig> {
    let my_existing_file = std::fs::File::open(config::CONFIG_FILE_NAME).unwrap();
    let json = ajson::parse_from_read(my_existing_file).unwrap();   
    let monsters_list = json.get("saved_configs").unwrap();
    let mut deserialized: Vec<structures::MonsterConfig> = serde_json::from_str(&monsters_list.as_str()).unwrap();
    deserialized
}

fn spawn_objects(room: structures::Rect, map: &structures::Map, objects: &mut Vec<structures::Object>) {
    
    let monster_num = rand::thread_rng().gen_range(0..config::MAX_ROOM_MONSTERS + 1);
    let monsters_list = get_config();

    for _ in 0..monster_num {
        let x = rand::thread_rng().gen_range(room.x1..room.x2+1);
        let y = rand::thread_rng().gen_range(room.y1..room.y2+1);
        if !is_blocked(x, y, map, objects) {
            let mut monster;
            let num = rand::thread_rng().gen_range(0..monsters_list.len());
            let data = &monsters_list[num];
            let color = Color {r: data.r, g: data.g, b: data.b};
            monster = structures::Object::new(x, y, data.symbol, color, &data.name, true);
            monster.attackable = Some(structures::Attackable{max_hp: data.max_hp, hp: data.max_hp, armor: data.armor, damage: data.damage, xp: 75, on_death: structures::DeathCallback::Monster});
            monster.alive = true;
            monster.ai = Some(structures::Ai::Basic);
            objects.push(monster);
        }
    }

    let num_items = rand::thread_rng().gen_range(0..config::MAX_ROOM_ITEMS + 1);

    for _ in 0..num_items {
        let x = rand::thread_rng().gen_range(room.x1 + 1..room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1..room.y2);

        if !is_blocked(x, y, map, objects) {
            let chance = rand::random::<f32>();
            let item = if chance < config::HEAL_SPAWN_CHANCE {
                let mut object = structures::Object::new(x, y, '!',  VIOLET, "healing potion", false);
                object.item = Some(structures::Item::Heal);
                object.always_visible = true;
                object
            } else if chance < config::HEAL_SPAWN_CHANCE + config::FIRE_SCROLL_SPAWN_CHANCE {
                let mut object = structures::Object::new(x, y, '#', LIGHT_ORANGE, "scroll of fire mark", false);
                object.item = Some(structures::Item::Fire);
                object.always_visible = true;
                object
            }
            else if chance < config::HEAL_SPAWN_CHANCE + config::FIRE_SCROLL_SPAWN_CHANCE + config::DOUBLE_DAMAGE_SPAWN_CHANCE {
                let mut object = structures::Object::new(x, y, '$', LIGHT_BLUE, "double damage", false);
                object.item = Some(structures::Item::DoubleDamage);
                object.always_visible = true;
                object
            } else {
                let mut object = structures::Object::new(x, y, '?', BLACK, "Flesh", false);
                object.item = Some(structures::Item::Blind);
                object.always_visible = true;
                object
            }
            ;
            objects.push(item);
        }
    }

}

fn is_blocked(x: i32, y: i32, map: &structures::Map, objects: &[structures::Object]) -> bool {
    if map[x as usize][y as usize].collision_enabled {
        return true;
    }
    objects.iter().any(|object| object.blocks && object.loc() == (x,y))
}

pub fn create_hor_tunnel(x1: i32, x2: i32, y: i32, map: &mut structures::Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = structures::Tile::empty();
    }
}

pub fn create_ver_tunnel(x: i32, y1: i32, y2: i32, map: &mut structures::Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = structures::Tile::empty();
    }
}


pub fn move_by(id: usize, dx: i32, dy: i32, map: &structures::Map, objects: &mut [structures::Object]) {
    let (x, y) = objects[id].loc();
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_loc(x + dx, y + dy);
    }
}


fn initialise_fov(tcod: &mut structures::Tcod, map: &structures::Map) {
    for y in 0..config::MAP_HEIGHT {
        for x in 0..config::MAP_WIDTH {
            tcod.fov.set(x, y, !map[x as usize][y as usize].is_visible, !map[x as usize][y as usize].collision_enabled);
        }
    }
}

pub fn next_level(tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &mut Vec<structures::Object>) {
    game.messages.add("You are healing", VIOLET);
    let heal_hp = objects[config::PLAYER].attackable.map_or(0, |f| f.max_hp / 2);
    objects[config::PLAYER].heal(heal_hp);

    game.level += 1;
    game.messages.add(format!("Prepare to danger on the {} level. Monsters became stronger!", game.level), RED);
    game.map = generate_map(objects);
    monsters_level_up(game, objects);
    initialise_fov(tcod, &game.map);
}

pub fn set_root(name: &str) -> tcod::console::Root {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title(name)
        .init();
    root
}

pub fn set_tcod(root: tcod::console::Root) -> structures::Tcod{
    tcod::system::set_fps(config::LIMIT_FPS);
 
    let tcod = structures::Tcod {
        root,
        screen: Offscreen::new(config::MAP_WIDTH, config::MAP_HEIGHT),  
        fov: FovMap::new(config::MAP_WIDTH, config::MAP_HEIGHT), 
        panel: Offscreen::new(config::SCREEN_WIDTH, config::PANEL_HEIGHT),
    };
    tcod
}

pub fn create_player() -> structures::Object {
    let mut player = structures::Object::new(5, 5, '@', BLUE, "Player", true);

    player.alive = true;
    player.attackable = Some(structures::Attackable{max_hp: 100, hp: 100, armor: 6, damage: 10, xp: 0, on_death: structures::DeathCallback::Player});

    player
}

pub fn inventory_menu(inventory: &[structures::Object], header: &str, root: &mut Root) -> Option<usize> {
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


fn monsters_level_up(game: &mut structures::Game, objects: &mut Vec<structures::Object>) {
    for id in 1..objects.len() {
        if let Some(mut attackable) = objects[id].attackable.as_mut() {
            attackable.damage += config::DAMAGE_PER_LEVEL * game.level as i32;
            attackable.armor += config::ARMOR_PER_LEVEL * game.level as i32;
            attackable.max_hp += config::MAX_HP_PER_LEVEL * game.level as i32;
            attackable.hp = attackable.max_hp;
            attackable.xp += config::XP_PER_LEVEL * game.level as i32;
        }
    }
}

pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut structures::Game, objects: &mut [structures::Object]) {
    let x = objects[config::PLAYER].x + dx;
    let y = objects[config::PLAYER].y + dy;

    let target_id = objects.iter().position(|object| object.attackable.is_some() && object.loc() == (x, y));


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

pub fn msgbox(text: &str, width: i32, root: &mut Root) {
    let options: &[&str] = &[];
    menu(text, options, width, root);
}


pub fn menu<T: AsRef<str>>(header: &str, options: &[T], width: i32, root: &mut Root) -> Option<usize> {
    assert!(options.len() <= 26, "Cannot have a menu with more than 26 options.");

    // calculate total height for the header (after auto-wrap) and one line per option
    let header_height = root.get_height_rect(0, 0, width, config::SCREEN_HEIGHT, header);
    let height = options.len() as i32 + header_height;

    // create an off-screen console that represents the menu's window
    let mut window = Offscreen::new(width, height);

    // print the header, with auto-wrap
    window.set_default_foreground(WHITE);
    window.print_rect_ex(0, 0, width, height, BackgroundFlag::None, TextAlignment::Left, header);

    // print all the options
    for (index, option_text) in options.iter().enumerate() {
        let menu_letter = (b'a' + index as u8) as char;
        let text = format!("({}) {}", menu_letter, option_text.as_ref());
        window.print_ex(0, header_height + index as i32, BackgroundFlag::None, TextAlignment::Left, text);
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


pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

pub fn monster_move(id: usize, player_x: i32, player_y: i32, map: &structures::Map, objects: &mut [structures::Object]) {
    let dx = player_x - objects[id].x;
    let dy = player_y - objects[id].y;
    let distance = ((dx*dx + dy*dy) as f32).sqrt();

    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;

    move_by(id, dx, dy, map, objects);
}

pub fn write(data: &editor::HelloState) {

    let monster = structures::MonsterConfig { 
        symbol: data.symbol.chars().nth(0).unwrap() as char, 
        name: data.name.to_string(),
        max_hp: data.max_hp.parse::<i32>().unwrap(),
        damage: data.damage.parse::<i32>().unwrap(),
        armor: data.armor.parse::<i32>().unwrap(),
        r: data.r.parse::<u8>().unwrap(),
        g: data.g.parse::<u8>().unwrap(),
        b: data.b.parse::<u8>().unwrap(),
    };

    if !Path::new(config::CONFIG_FILE_NAME).exists(){
        let mut my_file = std::fs::File::create(config::CONFIG_FILE_NAME).expect("creation failed");
        let mut monsterConfigList = vec![monster];
        let mut monsterConfigJson = structures::MonsterConfigJson {
            saved_configs: monsterConfigList,
        };
        let serialized = serde_json::to_string(&monsterConfigJson).unwrap();
        my_file.write(serialized.as_bytes());

    } else {
        let my_existing_file = std::fs::File::open(config::CONFIG_FILE_NAME).unwrap();
        let json = ajson::parse_from_read(my_existing_file).unwrap();   
        let monsters_list = json.get("saved_configs").unwrap();
        let mut deserialized: Vec<structures::MonsterConfig> = serde_json::from_str(&monsters_list.as_str()).unwrap();
        deserialized.push(monster);
        let mut monsterConfigJson = structures::MonsterConfigJson {
            saved_configs: deserialized,
        };
        let mut my_file = std::fs::File::create(config::CONFIG_FILE_NAME).expect("creation failed");
        let serialized = serde_json::to_string(&monsterConfigJson).unwrap();
        my_file.write(serialized.as_bytes());
    }
     //let deserialized: Vec<structures::MonsterConfig> = serde_json::from_str(&serialized).unwrap();
    // //my_file.write_all("Hello Chercher.tech".as_bytes()).expect("write failed");
    // my_file.write(data.name.as_bytes()).expect("Name write failed");
    // //my_file.write_all("	 Learning is Fun".as_bytes()).expect("write failed");
    // println!("The tag line of Chercher.tech has been added, open the file to see :)");
    
    


    // my_file.write(serialized.as_bytes());

    // println!("Serialized: {}", serialized);
    // let deserialized: structures::MonsterConfig = serde_json::from_str(&serialized).unwrap();
    // println!("Deserialized: {:?}", deserialized);
}

pub fn remove() {
    std::fs::remove_file(config::CONFIG_FILE_NAME).expect("could not remove file");
    println!("The file has been removed !");
}