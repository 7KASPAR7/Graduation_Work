use tcod::colors::*;
use tcod::console::*;

use tcod::map::{Map as FovMap};


use serde;
use serde_derive::*;
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct MonsterConfig {
    pub symbol: char,
    pub name: String,
    pub max_hp: i32,
    pub damage: i32,
    pub armor: i32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MonsterConfigJson {
    pub saved_configs: Vec<MonsterConfig>,
}


pub struct Tcod {
    pub root: Root,
    pub screen: Offscreen,
    pub fov: FovMap,
    pub panel: Offscreen,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
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
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub symbol: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub attackable: Option<Attackable>,
    pub ai: Option<Ai>,
    pub item: Option<Item>,
    pub always_visible: bool,
    pub level: i32,
}

impl Object {
    pub fn new(x: i32, y: i32, symbol: char, color: Color, name: &str, blocks: bool) -> Self {
        Object{x, y, symbol, color, name: name.into(), blocks, alive: false, attackable: None, ai: None, item: None, always_visible: false, level: 1}
    }



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

    pub fn get_distance_to(&self, target: &Object) -> f32 {
        let dx = target.x - self.x;
        let dy = target.y - self.y;

        ((dx*dx + dy*dy) as f32).sqrt()
    }

    pub fn get_damage(&mut self, damage: i32, game: &mut Game) -> Option<i32> {
        if let Some(attackable) = self.attackable.as_mut() {
            if damage > 0 {
                if attackable.hp - damage > 0 {
                    attackable.hp -= damage;
                }
                else {
                    attackable.hp = 0;
                }
            }
        }
        if let Some(attackable) = self.attackable {
            if attackable.hp <= 0 {
                self.alive = false;
                attackable.on_death.callback(self, game);
                return Some(attackable.xp);
            }
        }
        None
    }

    pub fn attack(&mut self, target: &mut Object, game: &mut Game) {
        let damage = self.attackable.map_or(0, |a| a.damage) - target.attackable.map_or(0, |a| a.armor);
        if damage > 0 {
            game.messages.add(format!("{} dealt {} damage to {}", self.name, damage, target.name), WHITE);
            if let Some(xp) = target.get_damage(damage, game) {
                self.attackable.as_mut().unwrap().xp += xp;
            }
        }
        else {
            game.messages.add(format!("{}'s armor is stronger than {}'s damage", target.name, self.name), WHITE);
        }
    }

    pub fn heal(&mut self, amount: i32) {
        if let Some(ref mut attackable) = self.attackable {
            attackable.hp += amount;
            if attackable.hp > attackable.max_hp {
                attackable.hp = attackable.max_hp;
            }
        }
    }

    pub fn use_double_damage(&mut self) {
        println!("Use DD");
        if let Some(ref mut attackable) = self.attackable {
            attackable.armor /= 2;
            attackable.damage *= 2;
        }
    }

}


// map is 2-dimension list of tiles
pub type Map = Vec<Vec<Tile>>;

pub struct Game{
    pub map: Map,
    pub messages: Messages,
    pub inventory: Vec<Object>,
    pub level: u32,
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
    pub xp: i32,
    pub on_death: DeathCallback,
}

#[derive(Clone, Debug, PartialEq)] 
pub enum Ai {
    Basic,
    Blind {
        prev_ai: Box<Ai>, 
        num_turns: i32
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    fn callback(self, object: &mut Object, game: &mut Game) {
        use DeathCallback::*;
        let callback = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object, game);
    }
}


fn player_death(player: &mut Object, game: &mut Game) {
    game.messages.add("You died!", RED);
    game.messages.add("Game Over!", RED);

    player.symbol = '%';
    player.color = DARK_RED;
}

fn monster_death(monster: &mut Object, game: &mut Game) {
    game.messages.add(format!("{} is dead! You gain {} experience points", monster.name, monster.attackable.unwrap().xp), ORANGE);
    monster.symbol = '%';
    monster.color = DARK_RED;
    monster.blocks = false;
    monster.attackable = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}

pub struct Messages {
    messages: Vec<(String, Color)>,
}

impl Messages {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color));
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
        self.messages.iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
    Fire,
    DoubleDamage,
    Blind,
}

pub enum UseResult {
    UsedUp,
    Cancelled,
}