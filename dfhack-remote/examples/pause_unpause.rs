use dfhack_remote::DFHack;
use dfhack_remote::RemoteFortressReader::SingleBool;

fn main() {
    let mut client = DFHack::connect().unwrap();

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
