use dfhack_remote;
use dfhack_remote::messages::SingleBool;

fn main() {
    let mut client = dfhack_remote::connect().unwrap();

    let status = client
        .remote_fortress_reader
        .get_pause_state()
        .unwrap()
        .get_Value();

    let mut request = SingleBool::new();
    request.set_Value(!status);
    client
        .remote_fortress_reader
        .set_pause_state(request)
        .unwrap();
}
