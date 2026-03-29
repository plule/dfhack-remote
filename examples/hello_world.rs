fn main() {
    let mut client = dfhack_remote::connect().unwrap();
    let world_info = client.core().get_world_info().unwrap();
    let world_name = world_info.world_name.as_ref().unwrap();

    println!(
        "Welcome to {} ({})",
        world_name.last_name(),
        world_name.english_name()
    );
}
