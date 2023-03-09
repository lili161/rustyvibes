mod args;
mod keycode;
mod play_sound;
mod start;
use fltk::{prelude::*, *, image::PngImage};
use std::{collections::HashMap, fs, thread::JoinHandle,file};

const ASCII_ART: &str = r#"
██████  ██    ██ ███████ ████████ ██    ██ ██    ██ ██ ██████  ███████ ███████ 
██   ██ ██    ██ ██         ██     ██  ██  ██    ██ ██ ██   ██ ██      ██      
██████  ██    ██ ███████    ██      ████   ██    ██ ██ ██████  █████   ███████ 
██   ██ ██    ██      ██    ██       ██     ██  ██  ██ ██   ██ ██           ██ 
██   ██  ██████  ███████    ██       ██      ████   ██ ██████  ███████ ███████
"#;
fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Oxy);
    let mut sound_packs = Vec::<String>::new();
    let mut sound_packs_map = HashMap::<String, String>::new();
    println!("{}", ASCII_ART);
    let paths = fs::read_dir(".\\Soundpacks").unwrap();
    for path in paths {
        let path = path.unwrap();
        let file_name = path.file_name();
        if file_name.to_str().unwrap().starts_with("."){continue;}
        let file_path = path.path();
        sound_packs.push(file_name.clone().into_string().unwrap());
        sound_packs_map.insert(
            file_name.clone().into_string().unwrap(),
            file_path.into_os_string().into_string().unwrap(),
        );
    }
    let sound_packs_copy = sound_packs.clone();
    let sound_packs_map_copy = sound_packs_map.clone();
    let f = fs::read("./icon.png");
    let a = PngImage::from_data(&f.unwrap()).unwrap();
    let mut my_window = window::Window::new(100, 100, 350, 120, "rustyvibes -gui").center_screen();
    my_window.set_icon(Some(a));
    let flex = group::Flex::default()
        .with_size(200, 100)
        .column()
        .center_of_parent();
    let _label = frame::Frame::default().with_label("Select what kind you want ↓");
    let mut choice = menu::Choice::default().with_size(60, 20).center_of_parent();
    for item in &sound_packs {
        choice.add_choice(item);
    }
    choice.set_value(1);
    let mut btn_use = button::Button::default().with_label("Use It");
    btn_use.set_callback(move |_| {
        let index = choice.value().clone();
        let a = sound_packs_copy.get(index as usize).unwrap();
        print!("{:?}", a);

        create_service(sound_packs_map_copy.get(a).unwrap().to_string());
    });
    flex.end();
    my_window.end();
    my_window.show();
    app.run().unwrap();
}

fn create_service(sound_pack: String) -> JoinHandle<()> {
    print!("{}", sound_pack);
    start::rustyvibes::start_rustyvibes(sound_pack, 100)
}
