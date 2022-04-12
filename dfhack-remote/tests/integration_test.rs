use dfhack_proto;
use dfhack_remote::DFHack;

#[test]
fn connect() {
    let mut client = DFHack::connect().unwrap();

    let world_info: dfhack_proto::BasicApi::GetWorldInfoOut = client.core.get_world_info().unwrap();
    let df_version = client.core.get_df_version().unwrap();

    let world_map = client.remote_fortress_reader.get_world_map().unwrap();

    println!(
        "Welcome to {}, in the year {}.",
        world_info.get_world_name().get_english_name(),
        world_map.get_cur_year(),
    );

    println!("DwarfFortress {}", df_version.get_value());
}
