extern crate sdl3;

use std::collections::HashMap;

use sdl3::{
    rect::Rect,
    render::{Canvas, Texture, TextureCreator}, 
    surface::{Surface}, 
    video::{Window, WindowContext}
};

use crate::{entities::camera::Camera, events::{
    collision_handler::{CollisionDetector, HitBox}, key_handler::{ Direction, Keys}
}, tiles::tile_handler::{self, Map, TileHandler}};


/// Manages a sprite animation sequence.
/// 
/// Each animation consists of multiple texture frames that cycle to create
/// the illusion of movement (e.g., a walking animation with 2 frames).
struct Animations<'a> {
    frames: Vec<Texture<'a>>,  // Collection of sprite textures
    current_frame: usize        // Index of the currently displayed frame
}

impl<'a> Animations<'a> {
    /// Creates an animation from a list of file paths.
    /// 
    /// # Arguments
    /// * `paths` - Slice of file paths to BMP sprite images
    /// * `texture_creator` - SDL texture creator tied to the rendering context
    /// 
    /// # Returns
    /// An `Animations` instance with all frames loaded and ready to display
    fn create_animations(
        paths: &[&str], 
        texture_creator: &'a TextureCreator<WindowContext>
    ) -> Animations<'a> {
        let mut frames = Vec::new();

        // Load each BMP file and convert it to a GPU texture
        for path in paths {
            let texture = Surface::load_bmp(path)
                .unwrap()
                .as_texture(texture_creator)
                .ok()
                .unwrap();
            frames.push(texture);
        }
        
        // Start at frame 0
        Animations { frames, current_frame: 0 }
    }
}

/// Represents the player character in the game.
/// 
/// The player has position, movement speed, direction, and direction-specific
/// walking animations. The sprite automatically changes based on movement direction
/// and cycles through animation frames while moving.
pub struct Player<'a> {
    speed: i32,                                           // Pixels moved per update
    pub rect: Rect,                                       // Position and size (hitbox)
    direction: Direction,                                 // Current facing direction
    walking_animation: HashMap<Direction, Animations<'a>>, // Animation for each direction
    pub change_sprite: bool,                             // Signal from main loop to advance frame
    pub on_collision: bool
}

impl<'a> Player<'a> {
    /// Creates a new player instance with all walking animations loaded.
    /// 
    /// # Arguments
    /// * `x` - Initial X position in pixels
    /// * `y` - Initial Y position in pixels
    /// * `speed` - Movement speed in pixels per frame
    /// * `tile_size` - Size of the player sprite (width and height)
    /// * `texture_creator` - SDL texture creator for loading sprite textures
    /// 
    /// # Returns
    /// A fully initialized `Player` ready to be updated and rendered
    pub fn new(
        x: i32, 
        y: i32, 
        speed: i32, 
        tile_size: u32,
        texture_creator: &'a TextureCreator<WindowContext>
    ) -> Self {
        // Define sprite file paths for each direction (2 frames per direction)
        let walking_down_paths = vec![
            "res/walking_sprites/boy_down_1.bmp", 
            "res/walking_sprites/boy_down_2.bmp"
        ];
        let walking_up_paths = vec![
            "res/walking_sprites/boy_up_1.bmp",
            "res/walking_sprites/boy_up_2.bmp"
        ];
        let walking_left_paths = vec![
            "res/walking_sprites/boy_left_1.bmp",
            "res/walking_sprites/boy_left_2.bmp"
        ];
        let walking_right_paths = vec![
            "res/walking_sprites/boy_right_1.bmp",
            "res/walking_sprites/boy_right_2.bmp"
        ];

        // Build the animation HashMap - one animation set per direction
        let mut walking_animations: HashMap<Direction, Animations> = HashMap::new();

        walking_animations.insert(
            Direction::Down, 
            Animations::create_animations(&walking_down_paths, &texture_creator)
        );
        walking_animations.insert(
            Direction::Up, 
            Animations::create_animations(&walking_up_paths, &texture_creator)
        );
        walking_animations.insert(
            Direction::Left, 
            Animations::create_animations(&walking_left_paths, &texture_creator)
        );
        walking_animations.insert(
            Direction::Right, 
            Animations::create_animations(&walking_right_paths, &texture_creator)
        );

        Self {
            rect: Rect::new(x, y, tile_size, tile_size),
            speed: speed,
            direction: Direction::Down,  // Start facing down
            walking_animation: walking_animations,
            change_sprite: false,
            on_collision: false
        }
    }
    
    /// Returns a mutable reference to the animation for the current direction.
    /// Used when we need to modify the animation (e.g., changing frames).
    fn select_animation_from_direction_mut(&mut self) -> &mut Animations<'a> {
        match self.direction {
            Direction::Up => self.walking_animation.get_mut(&Direction::Up).unwrap(),
            Direction::Down => self.walking_animation.get_mut(&Direction::Down).unwrap(),
            Direction::Left => self.walking_animation.get_mut(&Direction::Left).unwrap(),
            Direction::Right => self.walking_animation.get_mut(&Direction::Right).unwrap(),
        }
    }

    /// Returns an immutable reference to the animation for the current direction.
    /// Used when we only need to read animation data (e.g., during rendering).
    fn select_animation_from_direction(&self) -> &Animations<'a> {
        match self.direction {
            Direction::Up => self.walking_animation.get(&Direction::Up).unwrap(),
            Direction::Down => self.walking_animation.get(&Direction::Down).unwrap(),
            Direction::Left => self.walking_animation.get(&Direction::Left).unwrap(),
            Direction::Right => self.walking_animation.get(&Direction::Right).unwrap(),
        }
    }

    /// Advances the animation frame when signaled by the main loop.
    /// 
    /// This method toggles between frame 0 and frame 1 to create a simple
    /// two-frame walking animation. The actual timing is controlled externally
    /// via the `change_sprite` flag set in main.rs.
    fn sprite_change_handler(&mut self) {
        // Early exit if no frame change is needed
        if !self.change_sprite {
            return;
        }
        
        // Get mutable access to current direction's animation
        let animations = self.select_animation_from_direction_mut();
        
        // Toggle between frame 0 and frame 1
        animations.current_frame = if animations.current_frame == 1 { 0 } else { 1 };
        
        // Reset the signal flag
        self.change_sprite = false;
    }

    /// Updates the player's position and direction based on key input.
    /// 
    /// This method is called once per game frame. Movement is applied
    /// independently on each axis, allowing diagonal movement when multiple
    /// keys are pressed.
    /// 
    /// # Arguments
    /// * `keys` - Current state of WASD keys
    pub fn update(
        &mut self, 
        keys: &mut Keys, 
        map: &Map,
        collision_handler: &CollisionDetector, 
        tile_handler: &TileHandler,
        camera: &Camera
    ) {
        let hit_box = HitBox::new(
            camera.camera_x.abs() + self.rect.x + 8,
            camera.camera_y.abs() + self.rect.y + 16,
            (camera.camera_x.unsigned_abs() + self.rect.x.unsigned_abs()) / tile_handler.tile_size + 1,
            (camera.camera_y.unsigned_abs() + self.rect.y.unsigned_abs()) / tile_handler.tile_size + 1
        );
        println!("Player position: X: {}, Y: {}", self.rect.x, self.rect.y);
        
        self.on_collision = collision_handler.check(hit_box, map, &self.direction);

        if keys.w {
            self.direction = Direction::Up;
        }
        if keys.a {
            self.direction = Direction::Left;
        }
        if keys.s {
            self.direction = Direction::Down;
        }
        if keys.d {
            self.direction = Direction::Right;
        }
       
    }

    /// Renders the player sprite to the screen.
    /// 
    /// The sprite is automatically selected based on the current direction,
    /// and the animation frame advances when the player is moving. When
    /// stationary, the player displays the current frame without animating.
    /// 
    /// # Arguments
    /// * `canvas` - SDL canvas to draw on
    /// * `keys` - Current key states to determine if player is moving
    pub fn render(&mut self, canvas: &mut Canvas<Window>, keys: &Keys) {
        // Check if any movement key is pressed
        if keys.w || keys.a || keys.d || keys.s {
            // Player is moving: handle animation frame changes
            self.sprite_change_handler();
        }
        
        // Get the current animation for the player's direction
        let animations = self.select_animation_from_direction();
        let image = &animations.frames[animations.current_frame];
        
        // Query the texture to get its dimensions
        let image_attributes = image.query();
        
        // Define source rectangle (full texture)
        let src_rect = Rect::new(0, 0, image_attributes.width, image_attributes.height);

        // Copy the texture to the canvas at the player's position
        canvas.copy(image, src_rect, self.rect).ok().unwrap();
    }
}