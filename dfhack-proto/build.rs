extern crate protobuf_codegen_pure;
use std::path::PathBuf;

fn main() {
    // Should build into OUT_DIR, but I can't manage to include the whole directory in the build.
    let protos: Vec<String> = glob::glob("protos/*.proto")
        .unwrap()
        .map(|path| path.unwrap().to_str().unwrap().to_string())
        .collect();
    let mod_rs: Vec<String> = glob::glob("protos/*.proto")
        .unwrap()
        .map(|path| {
            path.unwrap()
                .with_extension("")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .map(|mod_name| format!("pub mod {};", mod_name))
        .collect();

    // Write the .rs from the proto
    let out_path = PathBuf::from("src/");
    protobuf_codegen_pure::Codegen::new()
        .out_dir(out_path)
        .inputs(protos)
        .include("protos")
        .run()
        .expect("Codegen failed.");

    // Write mod.rs
    let mut mod_rs_path = PathBuf::from("src/");
    mod_rs_path.push("lib.rs");
    std::fs::write(mod_rs_path, mod_rs.join("\n")).unwrap();
}
