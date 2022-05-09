use crate::structures;
use crate::config;
use crate::myengine;

use tcod::console::*;

use std::thread;

use druid::widget::{Align, Button, Flex, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Map Editor");


#[derive(Clone, Data, Lens)]
pub struct HelloState {
    pub light_wall_color_r: String,
    pub light_wall_color_g: String,
    pub light_wall_color_b: String,
    pub dark_wall_color_r: String,
    pub dark_wall_color_g: String,
    pub dark_wall_color_b: String,
    pub light_ground_color_r: String,
    pub light_ground_color_g: String,
    pub light_ground_color_b: String,
    pub dark_ground_color_r: String,
    pub dark_ground_color_g: String,
    pub dark_ground_color_b: String,
}

pub fn map_editor() {
      
    let main_window = WindowDesc::new(build_root_widget)
    .title(WINDOW_TITLE)
    .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        light_wall_color_r: "".into(),
        light_wall_color_g: "".into(),
        light_wall_color_b: "".into(),
        dark_wall_color_r: "".into(),
        dark_wall_color_g: "".into(),
        dark_wall_color_b: "".into(),
        light_ground_color_r: "".into(),
        light_ground_color_g: "".into(),
        light_ground_color_b: "".into(),
        dark_ground_color_r: "".into(),
        dark_ground_color_g: "".into(),
        dark_ground_color_b: "".into(),
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

    thread::spawn(move|| {
        while !tcod.root.window_closed() {
            tcod.screen.clear();
            myengine::render(&mut tcod, &mut game, &objects, true);
            tcod.root.flush();
        }
    }); 
    
    let light_wall_r_textbox = TextBox::new()
        .with_placeholder("What is light wall red?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::light_wall_color_r);
    let light_wall_g_textbox = TextBox::new()
        .with_placeholder("What is light wall green?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::light_wall_color_g);
    let light_wall_b_textbox = TextBox::new()
        .with_placeholder("What is light wall blue?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::light_wall_color_b);

    let dark_wall_r_textbox = TextBox::new()
        .with_placeholder("What is dark wall red?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::dark_wall_color_r);
    let dark_wall_g_textbox = TextBox::new()
        .with_placeholder("What is dark wall green?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::dark_wall_color_g);
    let dark_wall_b_textbox = TextBox::new()
        .with_placeholder("What is dark wall blue?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::dark_wall_color_b);
    
    let light_ground_r_textbox = TextBox::new()
        .with_placeholder("What is light ground red?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::light_ground_color_r);
    let light_ground_g_textbox = TextBox::new()
        .with_placeholder("What is light ground green?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::light_ground_color_g);
    let light_ground_b_textbox = TextBox::new()
        .with_placeholder("What is light ground blue?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::light_ground_color_b);
        
    let dark_ground_r_textbox = TextBox::new()
        .with_placeholder("What is dark ground red?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::dark_ground_color_r);
    let dark_ground_g_textbox = TextBox::new()
        .with_placeholder("What is dark ground green?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::dark_ground_color_g);
    let dark_ground_b_textbox = TextBox::new()
        .with_placeholder("What is dark ground blue?")
        .fix_width(config::TEXT_BOX_WIDTH)
        .lens(HelloState::dark_ground_color_b);

    let _data: &HelloState;
    let generate_map_button = Button::new("generate map").on_click(move |_, _data: &mut HelloState, _| {
        myengine::write_map(_data);
        });

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(light_wall_r_textbox)
        .with_spacer(config::SMALL_VERTICAL_WIDGET_SPACING)
        .with_child(light_wall_g_textbox)
        .with_spacer(config::SMALL_VERTICAL_WIDGET_SPACING)
        .with_child(light_wall_b_textbox)
        .with_spacer(config::VERTICAL_WIDGET_SPACING)
        .with_child(dark_wall_r_textbox)
        .with_spacer(config::SMALL_VERTICAL_WIDGET_SPACING)
        .with_child(dark_wall_g_textbox)
        .with_spacer(config::SMALL_VERTICAL_WIDGET_SPACING)
        .with_child(dark_wall_b_textbox)
        .with_spacer(config::VERTICAL_WIDGET_SPACING)
        .with_child(light_ground_r_textbox)
        .with_spacer(config::SMALL_VERTICAL_WIDGET_SPACING)
        .with_child(light_ground_g_textbox)
        .with_spacer(config::SMALL_VERTICAL_WIDGET_SPACING)
        .with_child(light_ground_b_textbox)
        .with_spacer(config::VERTICAL_WIDGET_SPACING)
        .with_child(dark_ground_r_textbox)
        .with_spacer(config::SMALL_VERTICAL_WIDGET_SPACING)
        .with_child(dark_ground_g_textbox)
        .with_spacer(config::SMALL_VERTICAL_WIDGET_SPACING)
        .with_child(dark_ground_b_textbox)
        .with_spacer(config::VERTICAL_WIDGET_SPACING)
        .with_child(generate_map_button);
    
    // center the two widgets in the available space
    Align::centered(layout)
}
