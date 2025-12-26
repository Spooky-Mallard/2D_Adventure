extern crate sdl3;

fn main() -> Result< (), String> {
    const ORIGINAL_TILE_SIZE: u32 = 16; // 16x16 tile
    const SCALE: u32 = 3;


    // SCREEN SETTINGS
    let tile_size = ORIGINAL_TILE_SIZE * SCALE; // 48x48 tile
    let max_screen_col = 16;
    let max_screen_row = 12;
    let screen_width = tile_size * max_screen_col; //768
    let screen_height = tile_size * max_screen_row; //576



    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let _window = video_subsystem.window("2D Adventure", screen_width, screen_height)
        .position_centered()
        .build()
        .expect("Failed to build window");

    println!("This went through and the windoow was successfully created");
    Ok(())
}
