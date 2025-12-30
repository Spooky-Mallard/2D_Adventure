extern crate sdl3;
use sdl3::{
    keyboard::Keycode,
    event::Event
};

/// Tracks the pressed/released state of the WASD movement keys.
/// 
/// Each boolean represents whether a specific key is currently held down.
/// This allows for smooth, simultaneous multi-directional input (e.g., diagonal movement).
pub struct Keys {
    pub w: bool,  // Move up
    pub a: bool,  // Move left
    pub s: bool,  // Move down
    pub d: bool   // Move right
}

/// Represents the four cardinal directions the player can face.
/// Used as keys in the animation HashMap to select appropriate sprite sets.
#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


/// Processes SDL keyboard events and updates the key state accordingly.
/// 
/// This function handles both KeyDown and KeyUp events for the WASD keys,
/// maintaining accurate state even when keys are pressed simultaneously.
/// 
/// # Arguments
/// * `event` - SDL event to process (typically from the event pump)
/// * `keys` - Mutable reference to the Keys struct to update
/// 
/// # Example Flow
/// 1. User presses W key → KeyDown event → keys.w = true
/// 2. Player moves up based on keys.w being true
/// 3. User releases W key → KeyUp event → keys.w = false
/// 4. Player stops moving up
pub fn key_handler(event: Event, keys: &mut Keys) {
    match event {
        // ===== KEY PRESS HANDLERS =====
        // Set the corresponding key state to true when pressed
        Event::KeyDown { keycode: Some(Keycode::W), ..} => {
            keys.w = true;
        },
        Event::KeyDown { keycode: Some(Keycode::A), ..} => {
            keys.a = true;
        },
        Event::KeyDown { keycode: Some(Keycode::S), ..} => {
            keys.s = true;
        },
        Event::KeyDown { keycode: Some(Keycode::D), ..} => {
            keys.d = true;
        },
        
        // ===== KEY RELEASE HANDLERS =====
        // Set the corresponding key state to false when released
        Event::KeyUp { keycode: Some(Keycode::W), ..} => {
            keys.w = false;
        },
        Event::KeyUp { keycode: Some(Keycode::A), ..} => {
            keys.a = false;
        },
        Event::KeyUp { keycode: Some(Keycode::S), ..} => {
            keys.s = false;
        },
        Event::KeyUp { keycode: Some(Keycode::D), ..} => {
            keys.d = false;
        }
        
        // Ignore all other events (mouse, window events, etc.)
        _ => ()
    }
}