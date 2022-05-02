
use winit::event_loop::EventLoop;

use crate::structures;
use crate::config;
use crate::myengine;

use std::env;
use std::fs::File;
use std::io::Write;

pub fn write() {
    // Create a temporary file.
    let mut my_file = std::fs::File::create("my_document.txt").expect("creation failed");
    my_file.write_all("Hello Chercher.tech".as_bytes()).expect("write failed");
    my_file.write_all("	 Learning is Fun".as_bytes()).expect("write failed");
    println!("The tag line of Chercher.tech has been added, open the file to see :)");
}

pub fn first() {
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

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
            tcod.fov.set(x, y, !game.map[x as usize][y as usize].is_visible, !game.map[x as usize][y as usize].collision_enabled);
        }
    }

    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use structures::PlayerAction::*;

    let key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[config::PLAYER].alive;

    match (key) {

        Key {code: Escape, ..} => {
            println!("escape is pressed");
            
            myengine::render(&mut tcod, &mut game, &objects, true);
            tcod.root.flush();
        },

        // (Key { code: Number5, .. }, _, true) => {
        //     let player_on_stairs = objects.iter().any(|object| object.loc() == objects[config::PLAYER].loc() && object.name == "door");
        //     if player_on_stairs {
        //         myengine::next_level(tcod, game, objects);
        //     }
        //     DidnotTakeTurn
        // }
        _ => {

        }
    }
    
    
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;
    
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        *control_flow = ControlFlow::Wait;
    
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                println!("Redrawing");

                
                // Application update code.
    
                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.

                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            },
            _ => ()
        }
    });
}

use tcod::input::Key;
use tcod::input::KeyCode::*;
use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Monsters Editor");

#[derive(Clone, Data, Lens)]
struct HelloState {
    symbol: String,
    max_hp: String,
    damage: String,
    armor: String,
    color: String,
}

pub fn second() {
      
    let main_window = WindowDesc::new(build_root_widget)
    .title(WINDOW_TITLE)
    .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        symbol: "".into(),
        max_hp: "".into(),
        damage: "".into(),
        armor: "".into(),
        color: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");    
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a textbox that modifies `name`.

    let symbol_textbox = TextBox::new()
    .with_placeholder("What is symbol?")
    .fix_width(TEXT_BOX_WIDTH)
    .lens(HelloState::symbol);

    let max_hp_textbox = TextBox::new()
        .with_placeholder("What is max HP?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::max_hp);

    let damage_textbox = TextBox::new()
        .with_placeholder("What is damage?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::damage);

    let armor_textbox = TextBox::new()
        .with_placeholder("What is armor?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::armor);
    
    let color_textbox = TextBox::new()
        .with_placeholder("What is color?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::color);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(symbol_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(max_hp_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(damage_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(armor_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(color_textbox);

    // center the two widgets in the available space
    Align::centered(layout)
}

