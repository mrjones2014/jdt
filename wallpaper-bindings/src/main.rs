use std::env;
use jdtwallpaper::set_wallpaper_from_file;

pub fn main() {
    let file_path = env::args().skip(1).next().unwrap();
    set_wallpaper_from_file(file_path.as_str());
}