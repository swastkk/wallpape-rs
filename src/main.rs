use wallpaper;

fn main() {
   println!("{:?}", wallpaper::get());
   wallpaper::set_from_path("/home/swastik/Downloads/test.png").unwrap();
   wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
   println!("{:?}", wallpaper::get());
}
