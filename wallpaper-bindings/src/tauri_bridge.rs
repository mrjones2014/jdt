use more_wallpapers::Mode;

#[tauri::command]
pub fn set_wallpaper_from_file(file_path: &str) {
    println!("Setting wallpaper from file: {}", file_path);
    let images = vec![file_path];
    let _ = more_wallpapers::set_wallpapers_from_vec(images, "", Mode::Crop);
}