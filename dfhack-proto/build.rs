extern crate protobuf_codegen_pure;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

fn main() {
    // Get dfhack sources
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dfhack.zip");
    let mut dfhack_out = File::create(&dest_path).unwrap();

    let mut resp = reqwest::blocking::get(
        "https://codeload.github.com/DFHack/dfhack/zip/refs/tags/0.47.05-r4",
    )
    .unwrap();
    std::io::copy(&mut resp, &mut dfhack_out).unwrap();
    let dfhack_file = std::fs::File::open(&dest_path).unwrap();
    let mut archive = zip::ZipArchive::new(dfhack_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if outpath.extension().unwrap_or_default() != "proto" {
            continue;
        }

        let mut dest = PathBuf::from(out_dir.clone());
        dest.push(PathBuf::from(outpath.file_name().unwrap()));

        let mut outfile = std::fs::File::create(&dest).unwrap();
        std::io::copy(&mut file, &mut outfile).unwrap();
    }

    let protos: Vec<String> = glob::glob(format!("{}/*.proto", out_dir.to_str().unwrap()).as_str())
        .unwrap()
        .map(|path| path.unwrap().to_str().unwrap().to_string())
        .collect();

    assert!(protos.len() > 0);

    let mod_rs: Vec<String> = glob::glob(format!("{}/*.proto", out_dir.to_str().unwrap()).as_str())
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
    let out_path = PathBuf::from("src/generated");
    std::fs::remove_dir_all(&out_path).unwrap();
    std::fs::create_dir_all(&out_path).unwrap();
    protobuf_codegen_pure::Codegen::new()
        .out_dir(out_path)
        .inputs(protos)
        .include(out_dir)
        .run()
        .expect("Codegen failed.");

    // Write mod.rs
    let mut mod_rs_path = PathBuf::from("src/generated");
    mod_rs_path.push("mod.rs");
    std::fs::write(mod_rs_path, mod_rs.join("\n")).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
