
use winit::event_loop::EventLoop;
use std::env;
use std::fs::File;
use std::io::Write;

use crate::structures;
use crate::config;
use crate::myengine;

pub fn write() {
    // Create a temporary file.
    let temp_directory = env::temp_dir();
    let temp_file = temp_directory.join("file");

    // Open a file in write-only (ignoring errors).
    // This creates the file if it does not exist (and empty the file if it exists).
    let mut file = File::create(temp_file).unwrap();

    // Write a &str in the file (ignoring the result).
    writeln!(&mut file, "Hello World!").unwrap();

    // Write a byte string.
    file.write(b"Bytes\n").unwrap();
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


