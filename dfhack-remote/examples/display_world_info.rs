use dfhack_remote::DFHack;

fn main() {
    let mut client = DFHack::connect().unwrap();
    let world_map = client.remote_fortress_reader.get_world_map().unwrap();

    println!(
        "Welcome to {} ({}). It is the year {}. This world is {}x{} tiles.",
        world_map.get_name(),
        world_map.get_name_english(),
        world_map.get_cur_year(),
        world_map.get_world_width(),
        world_map.get_world_height(),
    );
}
