use tcod::colors::*;

use crate::structures;
use crate::config;

pub fn cast_heal(_inventory_id: usize, _tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object]) -> structures::UseResult {
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

pub fn cast_dd(_inventory_id: usize, _tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object]) -> structures::UseResult {
    if let Some(attackable) = objects[config::PLAYER].attackable {
        if attackable.armor <= 0 {
            game.messages.add("You can't reduce your armor", RED);
            return structures::UseResult::Cancelled;
        }
        game.messages.add("You have double damage, but armor reduced by half!", LIGHT_BLUE);
        objects[config::PLAYER].use_double_damage();
        return structures::UseResult::UsedUp;
    }
    structures::UseResult::Cancelled
}

pub fn cast_fire(_inventory_id: usize, tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object]) -> structures::UseResult {
    let monster_id = closest_monster(tcod, objects, config::FIRE_RANGE);
    if let Some(monster_id) = monster_id {
        game.messages.add(
            format!("A fire mark burns the {}! The damage is {} hit points.", objects[monster_id].name, config::FIRE_DAMAGE),LIGHT_ORANGE);
        if let Some(xp) = objects[monster_id].get_damage(config::FIRE_DAMAGE, game) {
            objects[config::PLAYER].attackable.as_mut().unwrap().xp += xp;
        }
        structures::UseResult::UsedUp
    } else {
        game.messages
            .add("No enemy is close enough to burn.", RED);
            structures::UseResult::Cancelled
    }
}


pub fn cast_blind(_inventory_id: usize, tcod: &mut structures::Tcod, game: &mut structures::Game, objects: &mut [structures::Object]) -> structures::UseResult {
    
    let monster_id = closest_monster(tcod, objects, config::BLIND_RANGE);
    if let Some(monster_id) = monster_id {
        let old_ai = objects[monster_id].ai.take().unwrap_or(structures::Ai::Basic);
        objects[monster_id].ai = Some(structures::Ai::Blind {prev_ai: Box::new(old_ai), num_turns: config::BLIND_NUM_TURNS});
        game.messages.add(
            format!("The eyes of {} look vacant, as he starts to stumble around!", objects[monster_id].name), LIGHT_GREEN);
        structures::UseResult::UsedUp
    } else {
        game.messages.add("No enemy is close enough to strike.", RED);
        structures::UseResult::Cancelled
    }
}

fn closest_monster(tcod: &structures::Tcod, objects: &[structures::Object], max_range: i32) -> Option<usize> {
    let mut closest_enemy = None;
    let mut closest_dist = (max_range + 1) as f32; // start with (slightly more than) maximum range

    for (id, object) in objects.iter().enumerate() {
        if (id != config::PLAYER)
            && object.attackable.is_some()
            && object.ai.is_some()
            && tcod.fov.is_in_fov(object.x, object.y)
        {
            let dist = objects[config::PLAYER].get_distance_to(object);
            if dist < closest_dist {
                closest_enemy = Some(id);
                closest_dist = dist;
            }
        }
    }
    closest_enemy
}
