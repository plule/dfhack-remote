use dfhack_remote::{protos, DfClient};

#[test]
fn connect() {
    let mut client = DfClient::connect().unwrap();

    let request = protos::CoreProtocol::EmptyMessage::new();
    let world_info: dfhack_remote::Result<protos::BasicApi::GetWorldInfoOut> = client.request(
        "wrongplugin".to_string(),
        "GetWorldInfo".to_string(),
        request,
    );
    assert!(matches!(
        world_info,
        Err(dfhack_remote::DfRemoteError::RpcError())
    ));

    let request = protos::CoreProtocol::EmptyMessage::new();
    let world_info: protos::BasicApi::GetWorldInfoOut = client.get_world_info(request).unwrap();

    println!(
        "Welcome to {}",
        world_info.get_world_name().get_english_name()
    );
}
