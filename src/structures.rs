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
}

impl Unit {
    pub fn new(x: i32, y: i32, symbol: char, color: Color) -> Self{
        Unit{x, y, symbol, color}
    }

    pub fn move_by(&mut self, x_offset: i32, y_offset: i32, game: &Game) {
        if !game.map[(self.x + x_offset) as usize][(self.y + y_offset) as usize].collision_enabled {
            self.x += x_offset;
            self.y += y_offset;
        } 
    }

    pub fn draw(&self, screen: &mut dyn Console) {
        screen.set_default_foreground(self.color);
        screen.put_char(self.x, self.y, self.symbol, BackgroundFlag::None);
    }
}


// map is 2-dimension list of tiles
pub type Map = Vec<Vec<Tile>>;

pub struct Game{
    pub map: Map,
}