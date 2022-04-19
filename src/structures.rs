use tcod::colors::*;
use tcod::console::*;

use tcod::map::{Map as FovMap};

pub struct Tcod {
    pub root: Root,
    pub screen: Offscreen,
    pub fov: FovMap,
}


// all map is only tiles
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub collision_enabled: bool,
    pub is_visible: bool,
    pub is_explored: bool,
}

impl Tile{
    pub fn empty() -> Self {
        // we can not collide and see the empty tile
        Tile {
            collision_enabled: false,
            is_visible: false,
            is_explored: false,
        }
    }

    pub fn wall() -> Self {
        // we can collide and see the wall
        Tile {
            collision_enabled: true,
            is_visible: true,
            is_explored: false,
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect{
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        }
    }

    pub fn is_intersected_with(&self, second_rect: &Rect) -> bool{
        (self.x1 <= second_rect.x2) && (self.x2 >= second_rect.x2) && (self.y1 <= second_rect.y2) && (self.y2 >= second_rect.y1)
    }

    pub fn center(&self) -> (i32, i32){
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }
}


#[derive(Debug)]
pub struct Unit {
    pub x: i32,
    pub y: i32,
    pub symbol: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub attackable: Option<Attackable>,
    pub ai: Option<Ai>,
}

impl Unit {
    pub fn new(x: i32, y: i32, symbol: char, color: Color, name: &str, blocks: bool) -> Self{
        Unit{x, y, symbol, color, name: name.into(), blocks, alive: false, attackable: None, ai: None}
    }

    // pub fn move_by(&mut self, x_offset: i32, y_offset: i32, game: &Game) {
    //     if !game.map[(self.x + x_offset) as usize][(self.y + y_offset) as usize].collision_enabled {
    //         self.x += x_offset;
    //         self.y += y_offset;
    //     } 
    // }

    pub fn draw(&self, screen: &mut dyn Console) {
        screen.set_default_foreground(self.color);
        screen.put_char(self.x, self.y, self.symbol, BackgroundFlag::None);
    }

    pub fn loc(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_loc(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_distance_to(&self, target: &Unit) -> f32 {
        let dx = target.x - self.x;
        let dy = target.y - self.y;

        ((dx*dx + dy*dy) as f32).sqrt()
    }

    pub fn get_damage(&mut self, damage: i32) {
        if let Some(attackable) = self.attackable.as_mut() {
            if damage > 0 {
                attackable.hp -= damage;
            }
        }
        if let Some(attackable) = self.attackable {
            if attackable.hp <= 0 {
                self.alive = false;
                attackable.on_death.callback(self);
            }
        }
    }

    pub fn attack(&mut self, target: &mut Unit) {
        let damage = self.attackable.map_or(0, |a| a.damage) - target.attackable.map_or(0, |a| a.armor);
        if damage > 0 {
            println!("{} dealt {} damage to {}", self.name, damage, target.name);
            target.get_damage(damage);
        }
        else {
            println!("{}'s armor is stronger than {}'s damage", target.name, self.name);
        }


    }
}


// map is 2-dimension list of tiles
pub type Map = Vec<Vec<Tile>>;

pub struct Game{
    pub map: Map,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidnotTakeTurn,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attackable {
    pub max_hp: i32,
    pub hp: i32,
    pub armor: i32,
    pub damage: i32,
    pub on_death: DeathCallback,
}

#[derive(Clone, Debug, PartialEq)] 
pub enum Ai {
    Basic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    fn callback(self, unit: &mut Unit) {
        use DeathCallback::*;
        let callback: fn(&mut Unit) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(unit);
    }
}


fn player_death(player: &mut Unit) {
    // the game ended!
    println!("You died!");

    // for added effect, transform the player into a corpse!
    player.symbol = '%';
    player.color = DARK_RED;
}

fn monster_death(monster: &mut Unit) {
    // transform it into a nasty corpse! it doesn't block, can't be
    // attacked and doesn't move
    println!("{} is dead!", monster.name);
    monster.symbol = '%';
    monster.color = DARK_RED;
    monster.blocks = false;
    monster.attackable = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}