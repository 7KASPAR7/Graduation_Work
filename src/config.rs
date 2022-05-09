use tcod::map::{FovAlgorithm};

// actual size of the window
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

// size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;

// bar and panel sizes
pub const BAR_WIDTH: i32 = 20;
pub const PANEL_HEIGHT: i32 = 7;
pub const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;
pub const INVENTORY_WIDTH: i32 = 50;
pub const LEVEL_SCREEN_WIDTH: i32 = 40;
pub const CHARACTER_SCREEN_WIDTH: i32 = 30;

// chances
pub const HEAL_SPAWN_CHANCE: f32 = 0.25;
pub const FIRE_SCROLL_SPAWN_CHANCE: f32 = 0.25;
pub const DOUBLE_DAMAGE_SPAWN_CHANCE: f32 = 0.25;

// skills 
pub const HEAL_AMOUNT: i32 = 10;
pub const FIRE_DAMAGE: i32 = 10;
pub const FIRE_RANGE: i32 = 10;
pub const BLIND_RANGE: i32 = 3;
pub const BLIND_NUM_TURNS: i32 = 10;

// level up
pub const LEVEL_UP_XP_BASE: i32 = 200;
pub const LEVEL_UP_XP_PER_LEVEL: i32 = 150;

pub const DAMAGE_PER_LEVEL: i32 = 2;
pub const ARMOR_PER_LEVEL: i32 = 1;
pub const MAX_HP_PER_LEVEL: i32 = 5;
pub const XP_PER_LEVEL: i32 = 20;

pub const PLAYER_DAMAGE_PER_LEVEL: i32 = 5;
pub const PLAYER_ARMOR_PER_LEVEL: i32 = 3;
pub const PLAYER_MAX_HP_PER_LEVEL: i32 = 25;

// messages
pub const MESSAGES_X: i32 = BAR_WIDTH + 2;
pub const MESSAGES_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
pub const MESSAGES_HEIGHT: usize = PANEL_HEIGHT as usize - 1;


// rooms
pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 19;
pub const MAX_ROOM_MONSTERS: i32 = 3;
pub const MAX_ROOM_ITEMS: i32 = 2;

// default colors
// pub const COLOR_LIGHT_WALL: Color = Color {r: 106, g: 10, b: 171};
// pub const COLOR_DARK_WALL: Color = Color {r: 42, g: 23, b: 103};
// pub const COLOR_LIGHT_GROUND: Color = Color {r: 255, g: 207, b: 0};
// pub const COLOR_DARK_GROUND: Color = Color {r: 77, g: 50, b: 0};

// FoV
pub const FOV_ALG: FovAlgorithm = FovAlgorithm::Basic; 
pub const FOV_LIGHT_WALLS: bool = true; 
pub const FOV_RADIUS: i32 = 10;

// config
pub const PLAYER: usize = 0;

// fps
pub const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

pub const CONFIG_MONSTER_FILE_NAME: &str = "my_monster_config.json";
pub const CONFIG_MAP_FILE_NAME: &str = "my_map_config.json";
pub const VERTICAL_WIDGET_SPACING: f64 = 20.0;
pub const SMALL_VERTICAL_WIDGET_SPACING: f64 = 10.0;
pub const TEXT_BOX_WIDTH: f64 = 200.0;