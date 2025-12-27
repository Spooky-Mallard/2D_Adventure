extern crate sdl3;

use sdl3::{
    event::Event,
};
use std::time::{Instant};

mod entities;
use entities::{
    player::Player
};

mod events;
use events::{
    key_handler::Keys,
    key_handler::key_handler
};

mod tiles;
use tiles::{
    tile_handler::TileHandler
};

/// Entry point for the 2D Adventure game.
/// 
/// This function initializes SDL3, creates the game window, and runs the main game loop
/// at a fixed 60 FPS using delta time accumulation for frame-independent timing.
fn main() -> Result< (), String> {
    // ========== SCREEN CONFIGURATION ==========
    // Define the base tile size and scaling factor for pixel art rendering
    const ORIGINAL_TILE_SIZE: u32 = 16; // Base 16x16 pixel tiles
    const SCALE: u32 = 3;                // Scale up by 3x for better visibility

    // Calculate scaled tile size and screen dimensions
    let tile_size = ORIGINAL_TILE_SIZE * SCALE; // Results in 48x48 pixel tiles
    let max_screen_col = 16;                     // 16 tiles horizontally
    let max_screen_row = 12;                     // 12 tiles vertically
    let screen_width = tile_size * max_screen_col;   // 768 pixels wide
    let screen_height = tile_size * max_screen_row;  // 576 pixels tall

    // ========== SDL3 INITIALIZATION ==========
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // Create centered game window
    let window = video_subsystem.window("2D Adventure", screen_width, screen_height)
        .position_centered()
        .build()
        .expect("Failed to build window");

    // ========== RENDERING SETUP ==========
    // Convert window to canvas for 2D rendering
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let tile_handler = TileHandler::new(&texture_creator);
    canvas.clear();

    // ========== INPUT STATE ==========
    // Event pump processes SDL events each frame
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // Tracks the state of WASD keys (pressed/released)
    let mut keys = Keys{
        w: false,
        a: false,
        s: false,
        d: false
    };

    // ========== PLAYER INITIALIZATION ==========
    // Create player at position (100, 100) with speed 3 pixels per frame
    let mut player: Player = Player::new(100, 100, 3, tile_size, &texture_creator);

    // ========== FRAME TIMING SETUP ==========
    // Fixed timestep game loop: run at exactly 60 FPS
    const FPS: u128 = 60;
    let draw_interval = 1_000_000_000u128 / FPS; // Nanoseconds per frame (~16.67ms)
    
    let mut last_time = Instant::now();  // Track previous frame time
    let mut delta = 0;                   // Accumulated time since last update
    let mut sprite_counter = 0;          // Counter for animation frame switching

    // ========== MAIN GAME LOOP ==========
    // This loop continues until the user closes the window
    'running: loop {
        // Only update/render when enough time has accumulated for a full frame
        if delta >= draw_interval {
            // ===== EVENT PROCESSING =====
            // Poll all pending events and handle them
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running; // Exit game on window close
                    }
                    _ => {
                        // Delegate key events to the key handler
                        key_handler(event, &mut keys);
                    }
                }
            }

            // ===== UPDATE PHASE =====
            // Update player position based on current key states
            player.update(&mut keys);

            // ===== ANIMATION TIMING =====
            // Switch animation frame every 12 game frames (~200ms at 60 FPS)
            // This creates a visible walking animation without being too fast
            player.change_sprite = if sprite_counter >= 12 {
                sprite_counter = 0;
                true  // Signal to change to next animation frame
            } else {
                sprite_counter += 1;
                false // Keep current animation frame
            };

            // ===== RENDER PHASE =====
            tile_handler.draw_map( max_screen_row, &mut canvas, &tile_handler.maps[0]);
                                                   // Clear previous frame
            player.render(&mut canvas, &keys);      // Draw player sprite
            canvas.present();                       // Display rendered frame

            // ===== FRAME TIME MANAGEMENT =====
            // Subtract one frame's worth of time from delta
            // If multiple frames worth of time accumulated, the loop will
            // run multiple times to catch up (fixed timestep)
            delta -= draw_interval;
        }
        else {
            // ===== DELTA TIME ACCUMULATION =====
            // Not enough time has passed for a frame update yet
            // Accumulate the elapsed time since last measurement
            let current_time = Instant::now();
            delta += current_time.duration_since(last_time).as_nanos();
            last_time = current_time;
        }
    }

    Ok(())
}