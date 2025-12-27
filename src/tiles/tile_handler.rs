extern crate sdl3;

use std::{fs, io::BufReader};
use std::io::BufRead;

use sdl3::{rect::Rect, render::{Canvas, Texture, TextureCreator}, surface::Surface, video::{Window, WindowContext}};

struct Tile<'a> {
    image: Texture<'a>,
    collision: bool
}

pub struct Map{
    map: [[i32; 16]; 12]
}

pub struct TileHandler<'a>{
    tiles: Vec<Tile<'a>>,
    pub maps: Vec<Map>

}

impl<'a> Tile<'a> {
    fn new(image: Texture<'a>) -> Self {
        Self { 
            image: image,
            collision: false
        }
    }
}

impl<'a> TileHandler<'a> {

    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut tile_handler = Self { tiles: Vec::new() , maps: Vec::new()};
        tile_handler.load_tiles(texture_creator);
        tile_handler.load_maps();
        tile_handler
    }
    
    fn load_tiles(&mut self, texture_creator: &'a TextureCreator<WindowContext>){
        let mut tile_paths = Vec::new();

        for entry in fs::read_dir("res/tiles").unwrap(){
            let path = entry
                .ok()
                .unwrap()
                .path();
            
            if path.is_file(){
                let mut _path = String::new();
                _path.insert_str(0, path.to_str().unwrap());
                tile_paths.push(_path);
            }

        }
        tile_paths.sort();
        println!("{tile_paths:?}");

        // Load each BMP file and convert it to a GPU texture
        for path in tile_paths {
            let texture = Surface::load_bmp(path)
                .unwrap()
                .as_texture(texture_creator)
                .ok()
                .unwrap();
            self.tiles.push(Tile::new(texture));
        }

    }

    fn load_maps(&mut self){
        let mut map_paths = Vec::new();

        for entry in fs::read_dir("res/maps").unwrap(){
            let path = entry
                .ok()
                .unwrap()
                .path();
            
            if path.is_file(){
                let mut _path = String::new();
                _path.insert_str(0, path.to_str().unwrap());
                map_paths.push(_path);
            }
        }
        map_paths.sort();
        println!("{map_paths:?}");

        for path in map_paths{
            let f = fs::File::open(path).ok().unwrap();
            let mut reader = BufReader::new(f);
            let mut map = Map{map: [[0; 16]; 12]};
            let mut row = 0;
            
            'loop1: loop{
                let mut line = String::new();
                let len = reader.read_line(&mut line)
                    .expect("failed to read");
                if len == 0{break 'loop1;}
                
                'loop2: for (col,tile) in line.split_whitespace().enumerate(){
                    if tile == "\n" {continue 'loop2}
                    map.map[row][col] = tile.parse().unwrap();
                }
                row += 1;

            }
            self.maps.push(map);
        }
    }

    pub fn draw_map(&self, max_screen_row: u32, canvas: &mut Canvas<Window>, map: &Map){
        let mut col = 0;
        let mut row = 0;
        let mut x = 0;
        let mut y= 0;

        while row < max_screen_row {
            let image = &self.tiles[map.map[row as usize][col as usize] as usize].image;

            // Query the texture to get its dimensions
            let image_attributes = image.query();
            
            // Define source rectangle (full texture)
            let src_rect = Rect::new(0, 0, image_attributes.width, image_attributes.height);


            let dest_rect = Rect::new(x, y, image_attributes.width, image_attributes.height);


            // Copy the texture to the canvas at the player's position
            canvas.copy(image, src_rect, dest_rect).ok().unwrap();
            if col == 15 {
                col = 0;
                row += 1;
                x = 0;
                y += image_attributes.height as i32;
            }
            else{
                col += 1;
                x += image_attributes.width as i32;
            }
        }
        
    }
}