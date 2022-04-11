
use dfhack_remote::{protos, DfClient};

#[test]
fn connect() {
    let mut client = DfClient::connect().unwrap();

    let result = client
        .bind_method::<protos::CoreProtocol::EmptyMessage, protos::BasicApi::GetWorldInfoOut>(
            "GetWorldInfo".to_string(),
            "wrongplugin".to_string(),
        );
    assert!(matches!(
        result,
        Err(dfhack_remote::DfRemoteError::RpcError())
    ));

    let id = client
        .bind_method::<protos::CoreProtocol::EmptyMessage, protos::BasicApi::GetWorldInfoOut>(
            "GetWorldInfo".to_string(),
            "".to_string(),
        )
        .unwrap();
    let request = protos::CoreProtocol::EmptyMessage::new();
    let world_info: protos::BasicApi::GetWorldInfoOut = client.request(id, request).unwrap();

    println!(
        "Welcome to {}",
        world_info.get_world_name().get_english_name()
    );
    assert!(id > 0);
}
