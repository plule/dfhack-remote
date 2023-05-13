fn main() {
    let mut client = dfhack_remote::connect().unwrap();
    let world_info = client.core().get_world_info().unwrap();

    println!(
        "Welcome to {} ({})",
        world_info.world_name.last_name(),
        world_info.world_name.english_name()
    );
}
