extern crate sdl3;
use sdl3::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
    rect::Rect
};
use std::time::{Instant};

struct Keys{
    w: bool,
    a: bool,
    s: bool,
    d: bool
}


fn key_handler(event: Event, keys: &mut Keys){
    match event {
        // Key Down
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
        //Key Up
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
        _ => ()
    }
}

fn update(rect: &mut Rect, keys: &mut Keys){
    let speed = 3;

    if keys.w {
        rect.y -= speed;
    }
    if keys.a {
        rect.x -= speed;
    }
    if keys.s {
        rect.y += speed;
    }
    if keys.d {
        rect.x += speed;
    }
}

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
    
    let window = video_subsystem.window("2D Adventure", screen_width, screen_height)
        .position_centered()
        .build()
        .expect("Failed to build window");

    //CANVAS

    let mut canvas = window.into_canvas();


    canvas.clear();

    // GAME LOOP

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut keys = Keys{
        w: false,
        a: false,
        s: false,
        d: false
    };

    //PLAYER CHARACTER
    let mut rect = Rect::new(100, 100, tile_size, tile_size);

    // Timing
    const FPS :u128 = 60;
    let draw_interval = 1_000_000_000u128 / FPS;
    let mut last_time = Instant::now();
    let mut delta = 0;


    'running: loop {
        if delta >= draw_interval{

            // Event Handler
            for event in event_pump.poll_iter(){
                match event{
                    Event::Quit {..} => {
                        break 'running;
                    }
                    _ => {
                        key_handler(event, &mut keys);
                    }
                }
            
            }

            //UPDATE
            update(&mut rect, &mut keys);

            //RENDER
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            canvas.set_draw_color(Color::WHITE);
            canvas.fill_rect(rect).unwrap();
            canvas.present();


            // DELTA RESET
            delta -= draw_interval;
        }
        else{
            let current_time = Instant::now();
            delta += current_time.duration_since(last_time).as_nanos();
            last_time = current_time;
        }
    }


    Ok(())
}
