use dfhack_remote;

fn main() {
    let mut client = dfhack_remote::connect().unwrap();

    let status = client.remote_fortress_reader().get_pause_state().unwrap();

    client
        .remote_fortress_reader()
        .set_pause_state(!status)
        .unwrap();
}
