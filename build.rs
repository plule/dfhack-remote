extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/protos")
        .inputs(&[
            "protos/Basic.proto",
            "protos/BasicApi.proto",
            "protos/CoreProtocol.proto",
        ])
        .include("protos")
        .run()
        .expect("Codegen failed.");
    /*
    .include("protos")
    .run()
    .expect("Codegen failed.");*/
}
