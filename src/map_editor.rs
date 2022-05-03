
use winit::event_loop::EventLoop;

use crate::structures;
use crate::config;
use crate::myengine;

use std::env;
use std::fs::File;
use std::io::Write;

use druid::widget::{Align, Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, FileDialogOptions, Widget, WindowDesc, WidgetExt, FileSpec};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Map Editor");

const CONFIG_FILE_NAME: &str = "my_config.txt";


#[derive(Clone, Data, Lens)]
struct HelloState {
    light_wall_color: String,
    dark_wall_color: String,
    light_ground_color: String,
    dark_ground_color: String,
}

pub fn map_editor() {
      
    let main_window = WindowDesc::new(build_root_widget)
    .title(WINDOW_TITLE)
    .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        light_wall_color: "".into(),
        dark_wall_color: "".into(),
        light_ground_color: "".into(),
        dark_ground_color: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");    
}

fn build_root_widget() -> impl Widget<HelloState> {

    let game_name = "Editor";
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
            game.map[x as usize][y as usize].is_explored = true;
            tcod.fov.set(x, y, !game.map[x as usize][y as usize].is_visible, !game.map[x as usize][y as usize].collision_enabled);
        }
    }

    myengine::render(&mut tcod, &mut game, &objects, true);
    tcod.root.flush();
    
    let light_wall_textbox = TextBox::new()
        .with_placeholder("What is RGB?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::light_wall_color);

    let dark_wall_textbox = TextBox::new()
        .with_placeholder("What is RGB?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::dark_wall_color);
    
    let light_ground_textbox = TextBox::new()
        .with_placeholder("What is RGB?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::light_ground_color);
        
    let dark_ground_textbox = TextBox::new()
        .with_placeholder("What is RGB?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::dark_ground_color);

    let generate_map_button = Button::new("generate map").on_click(move |_, _, _| {
        });

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(light_wall_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(dark_wall_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(light_ground_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(dark_ground_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(generate_map_button);

    // center the two widgets in the available space
    Align::centered(layout)
}
