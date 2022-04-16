extern crate protobuf_codegen_pure;
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
    protoc_codegen(&protos, proto_include_dir, &out_path);

    // Generate the mod.rs file
    generate_mod_rs(protos, &out_path);
}

// Call the protoc code generation
fn protoc_codegen(protos: &Vec<PathBuf>, include_dir: &str, out_path: &PathBuf) {
    protobuf_codegen_pure::Codegen::new()
        .out_dir(out_path)
        .inputs(protos)
        .include(include_dir)
        .run()
        .expect("Codegen failed.");
}

fn generate_mod_rs(protos: Vec<PathBuf>, out_path: &PathBuf) {
    // Generate the mod.rs file
    let mod_rs: Vec<String> = protos
        .iter()
        .map(|path| {
            path.with_extension("")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .map(|mod_name| format!("pub mod {};", mod_name))
        .collect();
    // Write mod.rs
    let mut mod_rs_path = out_path.clone();
    mod_rs_path.push("mod.rs");
    std::fs::write(mod_rs_path, mod_rs.join("\n")).unwrap();
}
