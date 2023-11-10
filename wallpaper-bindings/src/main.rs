mod tauri_bridge;
use std::env;

pub fn set_wallpaper(filepath: &str){
    tauri_bridge::set_wallpaper_from_file(filepath);
}

fn main() {
    let file_path = env::args().skip(1).next().unwrap();
    set_wallpaper(file_path.as_str());
}