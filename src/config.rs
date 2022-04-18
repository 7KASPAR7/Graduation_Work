use tcod::colors::*;

use tcod::map::{FovAlgorithm};


// actual size of the window
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

// size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

pub const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 15;
pub const MAX_ROOM_MONSTERS: i32 = 3;


pub const COLOR_LIGHT_WALL: Color = Color {r: 106, g: 10, b: 171};
pub const COLOR_DARK_WALL: Color = Color {r: 42, g: 23, b: 103};
pub const COLOR_LIGHT_GROUND: Color = Color {r: 255, g: 207, b: 0};
pub const COLOR_DARK_GROUND: Color = Color {r: 77, g: 50, b: 0};

// FoV
pub const FOV_ALG: FovAlgorithm = FovAlgorithm::Basic; 
pub const FOV_LIGHT_WALLS: bool = true; 
pub const FOV_RADIUS: i32 = 10;

pub const PLAYER: usize = 0;