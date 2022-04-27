
use rand::Rng;
use tcod::colors::*;
use tcod::console::*;

mod config; 
mod structures;
mod skills;
mod myengine;


fn handle_keys(tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &mut Vec<structures::Object>) -> structures::PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use structures::PlayerAction::*;

    let key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[config::PLAYER].alive;

    match (key, key.text(), player_alive) {

        (Key { code: Escape, .. }, _, _) => Exit,

        (Key { code: Number5, .. }, _, true) => {
            let player_on_stairs = objects.iter().any(|object| object.loc() == objects[config::PLAYER].loc() && object.name == "door");
            if player_on_stairs {
                myengine::next_level(tcod, game, objects);
            }
            DidnotTakeTurn
        }

        (Key { code: Number4, .. }, _, true) => {
            // show character information
            let player = &objects[config::PLAYER];
            let level = player.level;
            let level_up_xp = config::LEVEL_UP_XP_BASE + player.level * config::LEVEL_UP_XP_PER_LEVEL;
            if let Some(attackable) = player.attackable.as_ref() {
                let msg = format!(
                    "Character information
        
    Level: {}
    XP: {}
    XP to lvl up: {}
        
    Maximum HP: {}
    Attack: {}
    Defense: {}",
                    level, attackable.xp, level_up_xp - attackable.xp, attackable.max_hp, attackable.damage, attackable.armor
                );
                myengine::msgbox(&msg, config::CHARACTER_SCREEN_WIDTH, &mut tcod.root);
            }
        
            DidnotTakeTurn
        }
        
        (Key { code: Up, .. }, _, true) => {
            myengine::player_move_or_attack(0, -1, game, objects);
            TookTurn
        },
        (Key { code: Down, .. }, _, true) => {
            myengine::player_move_or_attack(0, 1, game, objects);
            TookTurn
        },
        (Key { code: Left, .. }, _, true) => {
            myengine::player_move_or_attack(-1, 0, game, objects);
            TookTurn
        },
        (Key { code: Right, .. }, _, true) => {
            myengine::player_move_or_attack(1, 0, game, objects);
            TookTurn
        },
        
        (Key { code: Number1, .. }, _, true) => {
            //println!("Tried to pick up");
            let item_id = objects
                .iter()
                .position(|object| object.loc() == objects[config::PLAYER].loc() && object.item.is_some());
            if let Some(item_id) = item_id {
                myengine::pick_item_up(item_id, game, objects);
            }
            DidnotTakeTurn
        },

        (Key { code: Number2, .. }, _, true) => {
            let inventory_index = myengine::inventory_menu(
                &game.inventory,
                "Press the key to use an item or any other to cancel.\n",
                &mut tcod.root,
            );
            if let Some(inventory_index) = inventory_index {
                myengine::use_item(inventory_index, tcod, game, objects);
            }
            DidnotTakeTurn
        }

        _ => DidnotTakeTurn,
    }
    
}
    
fn ai_turn(id: usize, tcod: &structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object]) {
    use structures::Ai::*;
    if let Some(ai) = objects[id].ai.take() {
        let new_ai = match ai {
            Basic => ai_basic(id, tcod, game, objects),
            Blind {prev_ai, num_turns} => ai_blind(id, tcod, game, objects, prev_ai, num_turns),
        };
        objects[id].ai = Some(new_ai);
    }
}

fn ai_basic(id: usize, tcod: &structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object]) -> structures::Ai {
    let (monster_x, monster_y) = objects[id].loc();
    if tcod.fov.is_in_fov(monster_x, monster_y) {
        if objects[id].get_distance_to(&objects[config::PLAYER]) >= 2.0 {
            let (player_x, player_y) = objects[config::PLAYER].loc();
            myengine::monster_move(id, player_x, player_y, &game.map, objects);
        } else if objects[config::PLAYER].attackable.map_or(false, |f| f.hp > 0) {
            let (monster, player) = myengine::mut_two(id, config::PLAYER, objects);
            monster.attack(player, game);
        }
    }
    structures::Ai::Basic
}

fn ai_blind(id: usize, _tcod: &structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object], previous_ai: Box<structures::Ai>, num_turns: i32) -> structures::Ai {
    if num_turns >= 0 {
        myengine::move_by( id, rand::thread_rng().gen_range(-1..2), rand::thread_rng().gen_range(-1..2), &game.map, objects);
        structures::Ai::Blind{prev_ai: previous_ai, num_turns: num_turns - 1}
    } else {
        game.messages.add(format!("The {} is no longer confused!", objects[id].name), RED);
        *previous_ai
    }
}

fn level_up(tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object]) {
    let player = &mut objects[config::PLAYER];
    let level_up_xp = config::LEVEL_UP_XP_BASE + player.level * config::LEVEL_UP_XP_PER_LEVEL;

    if player.attackable.as_ref().map_or(0, |f| f.xp) >= level_up_xp {
        player.level += 1;
        game.messages.add(format!("You reached level {}!", player.level), YELLOW);
        let attackable = player.attackable.as_mut().unwrap();
    let mut choice = None;
    while choice.is_none() {
        choice = myengine::menu("Level up! Choose a stat to raise:\n",
            &[
                format!("(+{} HP, from {})", config::PLAYER_MAX_HP_PER_LEVEL, attackable.max_hp),
                format!("(+{} attack, from {})", config::PLAYER_DAMAGE_PER_LEVEL, attackable.damage),
                format!("(+{} defense, from {})", config::PLAYER_ARMOR_PER_LEVEL, attackable.armor),
            ],
            config::LEVEL_SCREEN_WIDTH,
            &mut tcod.root,
        );
    }
    attackable.xp -= level_up_xp;
    match choice.unwrap() {
        0 => {
            attackable.max_hp += config::PLAYER_MAX_HP_PER_LEVEL;
        }
        1 => {
            attackable.damage += config::PLAYER_DAMAGE_PER_LEVEL;
        }
        2 => {
            attackable.armor += config::PLAYER_ARMOR_PER_LEVEL;
        }
        _ => unreachable!(),
    }
    attackable.hp = attackable.max_hp;
    }
}

fn main() {
    tcod::system::set_fps(config::LIMIT_FPS);

    let game_name = "Roguelike Game Prototype";
    let root = myengine::set_root(game_name);
    let mut tcod = myengine::set_tcod(root);

    let player = myengine::create_player();
    
    let mut objects = vec![player];

    let mut game = structures::Game {
        map: myengine::generate_map(&mut objects),
        messages: structures::Messages::new(),
        inventory: vec![],
        level: 1,
    };
    for y in 0..config::MAP_HEIGHT {
        for x in 0..config::MAP_WIDTH {
            tcod.fov.set(x, y, !game.map[x as usize][y as usize].is_visible, !game.map[x as usize][y as usize].collision_enabled);
        }
    }

    game.messages.add("Welcome to Dungeon. Prepare for danger!", RED);

    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {
        
        tcod.screen.clear();

        let fov_recompute = previous_player_position != (objects[config::PLAYER].x, objects[config::PLAYER].y);
        myengine::render(&mut tcod, &mut game, &objects, fov_recompute);
        tcod.root.flush();
        level_up(&mut tcod, &mut game, &mut objects);
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

