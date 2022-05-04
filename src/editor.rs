
use crate::structures;
use crate::config;
use crate::myengine;

use druid::widget::{Align, Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, FileDialogOptions, Widget, WindowDesc, WidgetExt, FileSpec};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Monsters Editor");


#[derive(Clone, Data, Lens)]
struct HelloState {
    symbol: String,
    name: String,
    max_hp: String,
    damage: String,
    armor: String,
    color: String,
}

pub fn monsters_editor() {
      
    let main_window = WindowDesc::new(build_root_widget)
    .title(WINDOW_TITLE)
    .window_size((400.0, 400.0));

    let initial_state = HelloState {
        symbol: "".into(),
        name: "".into(),
        max_hp: "".into(),
        damage: "".into(),
        armor: "".into(),
        color: "".into(),
    };


    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");    
}

fn build_root_widget() -> impl Widget<HelloState> {

    let symbol_textbox = TextBox::new()
        .with_placeholder("What is symbol?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::symbol);

    let name_textbox = TextBox::new()
        .with_placeholder("What is name?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

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

    let save = Button::new("Save this monster").on_click(move |_, _, _| {
            myengine::write();
        });

    let remove = Button::new("Remove existing config").on_click(move |_, _, _| {
            myengine::remove();
        });

    let layout = Flex::column()
        .with_child(symbol_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(name_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(max_hp_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(damage_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(armor_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(color_textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(save)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(remove);

    Align::centered(layout)
}

pub fn play_menu() {
    let main_window = WindowDesc::new(build_root_menu_widget)
    .title(WINDOW_TITLE)
    .window_size((400.0, 400.0));

    let initial_state = HelloState {
        symbol: "".into(),
        name: "".into(),
        max_hp: "".into(),
        damage: "".into(),
        armor: "".into(),
        color: "".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");    
}

fn build_root_menu_widget() -> impl Widget<HelloState> {

    let monsters_editor_button = Button::new("Monsters editor").on_click(move |_, _, _| {
        monsters_editor();
    });
    let map_editor_button = Button::new("Map Editor").on_click(move |_, _, _| {

    });
    let play_button = Button::new("Play").on_click(move |_, _, _| {
        
    });


    let layout = Flex::column()
        .with_child(monsters_editor_button)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(map_editor_button)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(play_button);


    Align::centered(layout)
}