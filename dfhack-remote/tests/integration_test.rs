use dfhack_remote;

#[test]
fn sanity_check() {
    let mut message = dfhack_remote::CoreProtocol::IntMessage::new();
    message.set_value(12);
    assert!(message.get_value() == 12);
}
