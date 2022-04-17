use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let proto_include_dir = dfhack_proto_srcs::include_dir();
    let protos = dfhack_proto_srcs::protos();

    // Recreate the generated folder
    // should go to OUT_DIR instead
    let out_path = PathBuf::from("src/generated");
    if out_path.exists() {
        std::fs::remove_dir_all("src/generated").unwrap();
    }
    std::fs::create_dir_all("src/generated").unwrap();

    // Generate the protobuf message files
    generate_messages_rs(&protos, &proto_include_dir, &out_path);
}

fn generate_messages_rs(protos: &Vec<PathBuf>, include_dir: &str, out_path: &PathBuf) {
    let mut out_path = out_path.clone();
    out_path.push("messages");
    std::fs::create_dir_all(&out_path).unwrap();
    messages_protoc_codegen(protos, include_dir, &out_path);
    messages_generate_mod_rs(protos, &out_path);
}

// Call the protoc code generation
fn messages_protoc_codegen(protos: &Vec<PathBuf>, include_dir: &str, out_path: &PathBuf) {
    protobuf_codegen_pure::Codegen::new()
        .out_dir(out_path)
        .inputs(protos)
        .include(include_dir)
        .run()
        .expect("Codegen failed.");
}

fn messages_generate_mod_rs(protos: &Vec<PathBuf>, out_path: &PathBuf) {
    // Generate the mod.rs file
    let mut mod_rs = Vec::new();

    for proto in protos {
        let mod_name = proto
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        mod_rs.push(format!("mod {};", mod_name));
        mod_rs.push(format!("pub use self::{}::*;", mod_name));
    }
    // Write mod.rs
    let mut mod_rs_path = out_path.clone();
    mod_rs_path.push("mod.rs");
    std::fs::write(mod_rs_path, mod_rs.join("\n")).unwrap();
}
