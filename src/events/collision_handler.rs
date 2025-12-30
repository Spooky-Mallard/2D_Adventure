use crate::{events::key_handler::Direction, tiles::tile_handler::{Map, Tile, TileHandler}};

extern crate sdl3;

pub struct HitBox{
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    tile_x: u32,
    tile_y: u32
}

pub struct CollisionDetector<'a>{
    tiles: &'a Vec<Tile<'a>>,
    tile_size: u32
}

impl HitBox {
    pub fn new(x: i32, y: i32, tile_x: u32, tile_y: u32) -> Self{
        Self { x, y, width: 32, height: 32, tile_x, tile_y }
    }
}

impl<'a> CollisionDetector<'a> {
    pub fn new(tile_handler: &'a  TileHandler, tile_size: u32) -> Self{
        Self { tiles: &tile_handler.tiles , tile_size}
    }

    pub fn check(&self, hit_box: HitBox, map: &Map, direction: &Direction) -> bool{
        let result: Result<bool, &str> = (|| {
            let top_tile = map.map
                .get(hit_box.tile_y as usize + 1).ok_or("Top Tile Failed")?
                .get(hit_box.tile_x as usize).ok_or("Top Tile Failed")?;
            let bottom_tile = map.map
                .get(hit_box.tile_y as usize - 1).ok_or("Bottom Tile Failed")?
                .get(hit_box.tile_x as usize).ok_or("Bottom Tile Failed")?;
            let left_tile = map.map
                .get(hit_box.tile_y as usize).ok_or("Left Tile Failed")?
                .get(hit_box.tile_x as usize - 1 ).ok_or("Left Tile Failed")?;
            let right_tile = map.map
                .get(hit_box.tile_y as usize).ok_or("Right Tile Failed")?
                .get(hit_box.tile_x as usize + 1).ok_or("Right Tile Failed")?;

            match direction{
                Direction::Up => {
                    if self.tiles[top_tile.index as usize].immovable {
                        if hit_box.x + hit_box.width as i32 >= top_tile.x &&
                           hit_box.x <= top_tile.x + self.tile_size as i32 &&
                           hit_box.y + hit_box.height as i32 >= top_tile.y&&
                           hit_box.y <= top_tile.y + self.tile_size as i32
                        {
                            return Ok(true);
                        }    
                    }
                },
                Direction::Down => {
                    if self.tiles[bottom_tile.index as usize].immovable {
                        if hit_box.x + hit_box.width as i32 >= bottom_tile.x &&
                           hit_box.x <= bottom_tile.x + self.tile_size as i32 &&
                           hit_box.y + hit_box.height as i32 >= bottom_tile.y &&
                           hit_box.y <= bottom_tile.y + self.tile_size as i32
                        {
                            return Ok(true);
                        }   
                    }
                },
                Direction::Left => {
                    if self.tiles[left_tile.index as usize].immovable {
                        if hit_box.x + hit_box.width as i32 >= left_tile.x &&
                           hit_box.x <= left_tile.x + self.tile_size as i32 &&
                           hit_box.y + hit_box.height as i32 >= left_tile.y &&
                           hit_box.y <= left_tile.y + self.tile_size as i32
                        {
                            return Ok(true);
                        }   
                    }
                },
                Direction::Right => {
                    if self.tiles[right_tile.index as usize].immovable {
                        if hit_box.x + hit_box.width as i32 >= right_tile.x &&
                           hit_box.x <= right_tile.x + self.tile_size as i32 &&
                           hit_box.y + hit_box.height as i32 >= right_tile.y &&
                           hit_box.y <= right_tile.y + self.tile_size as i32
                        {
                            return Ok(true);
                        }   
                    }
                },
            }
            Ok(false)
        })();
        
        match result{
            Ok(v) => {v},
            Err(e) => {
               if * direction == Direction::Up && e == "Top Tile Failed"{
                    return true;
               }
               else if * direction == Direction::Down && e == "Bottom Tile Failed"{
                    return true;
               }
               else if * direction == Direction::Left && e == "Left Tile Failed"{
                    return true;
               }
               else if * direction == Direction::Right && e == "Right Tile Failed"{
                    return true;
               }
               else{
                    return false;
               }
            },
        }

    }

}