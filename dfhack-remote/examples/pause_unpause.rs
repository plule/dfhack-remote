use dfhack_remote::messages::SingleBool;
use dfhack_remote::DFHack;

fn main() {
    let mut client = DFHack::connect("127.0.0.1:5000").unwrap();

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
