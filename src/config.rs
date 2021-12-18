use tcod::colors::*;

// actual size of the window
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

// size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

pub const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 10;

pub const COLOR_WALL: Color = Color {r: 0, g: 104, b: 142};
pub const COLOR_GROUND: Color = Color {r: 114, g: 28, b: 34};