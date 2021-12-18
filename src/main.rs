use std::cmp;

use rand::Rng;
use tcod::colors::*;
use tcod::console::*;

mod config;



struct Tcod {
    root: Root,
    screen: Offscreen,
}

// all map is only tiles
#[derive(Clone, Copy, Debug)]
struct Tile {
    collision_enabled: bool,
    is_visible: bool,
}

impl Tile{
    fn empty() -> Self {
        // we can not collide and see the empty tile
        Tile {
            collision_enabled: false,
            is_visible: false,
        }
    }

    fn wall() -> Self {
        // we can collide and see the wall
        Tile {
            collision_enabled: true,
            is_visible: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
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
struct Unit {
    x: i32,
    y: i32,
    symbol: char,
    color: Color,
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
type Map = Vec<Vec<Tile>>;

struct Game{
    map: Map,
}

fn create_room(room: Rect, map: &mut Map){
    for x in (room.x1 + 1).. room.x2 {
        for y in (room.y1 + 1).. room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_hor_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_ver_tunnel(x: i32, y1: i32, y2: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn generate_map(player: &mut Unit) -> Map {
    let mut map = vec![vec![Tile::wall(); config::MAP_HEIGHT as usize]; config::MAP_WIDTH as usize];
    
    let mut rooms = vec![];

    for _ in 0..config::MAX_ROOMS {
        // random width and height
        let width = rand::thread_rng().gen_range(config::ROOM_MIN_SIZE..config::ROOM_MAX_SIZE + 1);
        let height = rand::thread_rng().gen_range(config::ROOM_MIN_SIZE..config::ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map
        let x = rand::thread_rng().gen_range(0..config::MAP_WIDTH - width);
        let y = rand::thread_rng().gen_range(0..config::MAP_HEIGHT - height);

        let new_room = Rect::new(x, y, width, height);

        // run through the other rooms and see if they intersect with this one
        let failed = rooms
            .iter()
            .any(|other_room| new_room.is_intersected_with(other_room));

        if !failed {
            // this means there are no intersections, so this room is valid

            // "paint" it to the map's tiles
            create_room(new_room, &mut map);

            // center coordinates of the new room, will be useful later
            let (new_x, new_y) = new_room.center();
            println!("new {}, {}", new_x, new_y);

            if rooms.is_empty() {
                // this is the first room, where the player starts at
                player.x = new_x;
                player.y = new_y;
            } else {
                // all rooms after the first:
                // connect it to the previous room with a tunnel

                // center coordinates of the previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                println!("prev {}, {}", prev_x, prev_y);

                // toss a coin (random bool value -- either true or false)
                if rand::random() {
                    // first move horizontally, then vertically
                    create_hor_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_ver_tunnel(new_x, prev_y, new_y,  &mut map);
                } else {
                    // first move vertically, then horizontally
                    create_ver_tunnel(prev_x, prev_y, new_y, &mut map);
                    create_hor_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }

            // finally, append the new room to the list
            rooms.push(new_room);
        }
    }

    map
}

fn render(tcod: &mut Tcod, game: &Game, units: &[Unit]) {
    // go through all tiles, and set their background color
    for y in 0..config::MAP_HEIGHT {
        for x in 0..config::MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].is_visible;
            if wall {
                tcod.screen
                    .set_char_background(x, y, config::COLOR_WALL, BackgroundFlag::Set);
            } else {
                tcod.screen
                    .set_char_background(x, y, config::COLOR_GROUND, BackgroundFlag::Set);
            }
        }
    }

    // draw all objects in the list
    for unit in units {
        unit.draw(&mut tcod.screen);
    }

    // blit the contents of "con" to the root console
    blit(
        &tcod.screen,
        (0, 0),
        (config::MAP_WIDTH, config::MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Unit) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        // Key {
        //     code: Enter,
        //     alt: true,
        //     ..
        // } => {
        //     // Alt+Enter: toggle fullscreen
        //     let fullscreen = tcod.root.is_fullscreen();
        //     tcod.root.set_fullscreen(!fullscreen);
        // }
        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),

        _ => {}
    }
    false
}
    

fn main() {
    tcod::system::set_fps(config::LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title("Roguelike Game Prototype")
        .init();


    let screen = Offscreen::new(config::MAP_WIDTH, config::MAP_HEIGHT);

    let mut tcod = Tcod {root, screen};

    let player = Unit::new(0, 0, '@', WHITE);

    // create an NPC
    let npc = Unit::new(config::SCREEN_WIDTH / 2 - 5, config::SCREEN_HEIGHT / 2, '$', GREEN);

    // the list of objects with those two
    let mut units = [player, npc];

    let game = Game {
        // generate map (at this point it's not drawn to the screen)
        map: generate_map(&mut units[0]),
    };

    while !tcod.root.window_closed() {
        
        tcod.screen.clear();

         // render the screen
         render(&mut tcod, &game, &units);

         tcod.root.flush();
 
         // handle keys and exit game if needed
         let player = &mut units[0];
         let exit = handle_keys(&mut tcod, &game, player);
         if exit {
             break;
         }
    }
}

