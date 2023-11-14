use more_wallpapers::Mode;
use std::path::PathBuf;

pub fn set_wallpaper_from_file(path: &PathBuf) {
    set_wallpaper(path)
}

fn set_wallpaper(file_path: &PathBuf) {
    println!("Setting wallpaper from file: {}", file_path);
    let images = vec![file_path];

    let _ = more_wallpapers::set_wallpapers_from_vec(images, "", Mode::Crop);
}