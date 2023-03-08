mod args;
mod keycode;
mod play_sound;
mod start;
use crate::args::ArgParser;
use clap::Parser;
use fltk::{enums::*, prelude::*, *};
use stop_thread::kill_thread_forcibly_exit_code;

use std::{
    collections::HashMap,
    fs,
    thread::{self, JoinHandle},
};

const ASCII_ART: &str = r#"
██████  ██    ██ ███████ ████████ ██    ██ ██    ██ ██ ██████  ███████ ███████ 
██   ██ ██    ██ ██         ██     ██  ██  ██    ██ ██ ██   ██ ██      ██      
██████  ██    ██ ███████    ██      ████   ██    ██ ██ ██████  █████   ███████ 
██   ██ ██    ██      ██    ██       ██     ██  ██  ██ ██   ██ ██           ██ 
██   ██  ██████  ███████    ██       ██      ████   ██ ██████  ███████ ███████
"#;

fn main() {
    let app = app::App::default();
    // =============================
    let mut soundPacks = Vec::<String>::new();
    let mut soundPacks_map = HashMap::<String, String>::new();
    // println!("{}", ASCII_ART);
    let a = ArgParser::parse();
    // let soundpack = a.soundpack;
    let soundpack = String::from("D:\\KeyBoardSound\\Soundpacks\\cherrymx-blue-pbt");
    // let paths = fs::read_dir("D:\\Work\\rustyviber\\rustyvibes\\Soundpacks").unwrap();
    let paths = fs::read_dir(".\\Soundpacks").unwrap();

    for path in paths {
        let path = path.unwrap();
        let file_name = path.file_name();
        let file_path = path.path();
        soundPacks.push(file_name.clone().into_string().unwrap());
        soundPacks_map.insert(
            file_name.clone().into_string().unwrap(),
            file_path.into_os_string().into_string().unwrap(),
        );
    }
    println!("{:#?}", soundPacks_map);
    let handle =create_service(soundpack);

    // =============================




    let mut my_window = window::Window::new(100, 100, 400, 300, "Daily Report");
    let mut menubar = menu::MenuBar::new(0, 0, 400, 40, "1");
    let flex = group::Flex::default()
        .with_size(200, 100)
        .column()
        .center_of_parent();
    let label = frame::Frame::default().with_label("Write down what you doing here ↓");
    let mut input = input::MultilineInput::default();
    input.set_value("");
    flex.end();
    let mut button = button::Button::default()
        .with_size(50, 30)
        .with_pos(175, 220)
        .with_label("Submit");
    my_window.end();
    my_window.show();

  
    app.run().unwrap();
}

fn kill_service(handle: JoinHandle<()>) {
    unsafe {
        kill_thread_forcibly_exit_code(handle, 0);
    }
}

fn create_service(sound_pack:String) -> JoinHandle<()>{
    start::rustyvibes::start_rustyvibes(sound_pack, 100)
}
