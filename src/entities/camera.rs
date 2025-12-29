extern crate sdl3;

use sdl3::{
    pixels::Color, rect::Rect, render::Canvas, video::Window
};

use crate::{
    events::key_handler::Keys, 
    tiles::tile_handler::{self, Map, TileHandler}
};

pub struct Camera{
    speed: i32,
    camera_x: i32,
    camera_y: i32,
    camera_screen_width: u32,
    camera_screen_height: u32
}

impl Camera {
    pub fn new(x: i32, y: i32, camera_width: u32, camera_height: u32, speed: i32) -> Self{
        Self { 
            speed: speed,
            camera_x: x,
            camera_y: y,
            camera_screen_width: camera_width,
            camera_screen_height: camera_height,          
        }
    }

    pub fn update(&mut self, keys: &mut Keys) {
        if keys.w {
            self.camera_y += self.speed;
        }
        if keys.a {
            self.camera_x += self.speed;
        }
        if keys.s {
            self.camera_y -= self.speed;
        }
        if keys.d {
            self.camera_x -= self.speed;
        }
    }

    pub fn draw_camera(
        &self, 
        max_world_row: u32,
        max_world_col: u32,
        max_screen_row: u32,
        max_screen_col: u32,
        tile_handler: &TileHandler,
        canvas: &mut Canvas<Window>, 
        map: &Map){

        let start_world_tile_x = self.camera_x.unsigned_abs()/ tile_handler.tile_size ;
        let start_world_tile_y = self.camera_y.unsigned_abs()/ tile_handler.tile_size;
        let mut world_tile_x = start_world_tile_x;
        let mut world_tile_y = start_world_tile_y;
        let mut x = self.camera_x;
        let mut y= self.camera_y;
       
        println!("X: {}, Y:{}", self.camera_x, self.camera_y);

        for row in 0 .. max_world_row{
            for col in 0  .. max_world_col{
                if row >= start_world_tile_y &&
                   row < start_world_tile_y + max_screen_row + 1 &&
                   col >= start_world_tile_x &&
                   col < start_world_tile_x + max_screen_col + 1
                {
                    let image = &tile_handler.tiles[map.map[row as usize][col as usize] as usize].image;
           
                    // Define source rectangle (full texture)
                    let src_rect = Rect::new(0, 0, tile_handler.tile_size as u32, tile_handler.tile_size as u32);

                    let dest_rect = Rect::new(x, y, tile_handler.tile_size as u32, tile_handler.tile_size as u32);

                    // Copy the texture to the canvas at the player's position
                    canvas.copy(image, src_rect, dest_rect).ok().unwrap();
                }
                x += tile_handler.tile_size as i32;
            }
            x = self.camera_x;
            y += tile_handler.tile_size as i32;

        }  
        
    }

}