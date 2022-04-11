use dfhack_proto;

#[test]
fn sanity_check() {
    let mut message = dfhack_proto::CoreProtocol::IntMessage::new();
    message.set_value(12);
    assert!(message.get_value() == 12);
}
