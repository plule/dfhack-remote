use dfhack_remote::DFHack;

fn main() {
    let mut client = DFHack::connect().unwrap();

    let df_version = client.core.get_df_version().unwrap();
    let dfhack_version = client.core.get_version().unwrap();

    let world_map = client.remote_fortress_reader.get_world_map().unwrap();
    let unit_list = client.remote_fortress_reader.get_unit_list().unwrap();

    println!(
        "DwarfFortress: {}\nDFHack: {}",
        df_version.get_value(),
        dfhack_version.get_value()
    );

    println!(
        "Welcome to {} ({}), home of {} souls. It is the year {}.",
        world_map.get_name(),
        world_map.get_name_english(),
        unit_list.get_creature_list().len(),
        world_map.get_cur_year(),
    );

    println!(
        "This world is {}x{}.",
        world_map.get_world_width(),
        world_map.get_world_height(),
    )
}
