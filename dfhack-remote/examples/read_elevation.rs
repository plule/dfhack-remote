use bmp::{px, Image, Pixel};
use dfhack_remote;

fn main() {
    let mut client = dfhack_remote::connect().unwrap();

    let world_map = client.remote_fortress_reader.get_world_map_new().unwrap();
    let width = world_map.get_world_width() as usize;
    let height = world_map.get_world_height() as usize;

    let mut img = Image::new(width as u32, height as u32);
    let tiles = world_map.get_region_tiles();
    for x in 0..width {
        for y in 0..height {
            let tile = &tiles[x + y * width];
            let elevation = tile.get_elevation();
            let water_elevation = tile.get_water_elevation();
            let b = elevation;
            let above_water = elevation > water_elevation;
            let g = match above_water {
                true => elevation,
                false => 0,
            };
            let r = g;

            img.set_pixel(x as u32, y as u32, px!(r, g, b));
        }
    }
    img.save("elevation.bmp").unwrap();
}
