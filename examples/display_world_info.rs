fn main() {
    let mut client = dfhack_remote::connect().unwrap();
    let world_map = client.remote_fortress_reader().get_world_map().unwrap();

    println!(
        "Welcome to {} ({}). It is the year {}. This world is {}x{} tiles.",
        world_map.name(),
        world_map.name_english(),
        world_map.cur_year(),
        world_map.world_width(),
        world_map.world_height(),
    );
}
