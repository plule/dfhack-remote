use std::{
    fs::File,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    // Download the file
    let dfhack_archive_path = Path::new(&out_dir).join("dfhack.zip");
    let mut dfhack_archive = File::create(&dfhack_archive_path).unwrap();
    let mut dfhack_download_request = reqwest::blocking::get(
        "https://codeload.github.com/DFHack/dfhack/zip/refs/tags/0.47.05-r4",
    )
    .unwrap();
    std::io::copy(&mut dfhack_download_request, &mut dfhack_archive).unwrap();

    // Extract the protos
    let mut protos = Vec::new();
    let dfhack_archive = File::open(&dfhack_archive_path).unwrap();
    let mut dfhack_archive = zip::ZipArchive::new(dfhack_archive).unwrap();
    for i in 0..dfhack_archive.len() {
        let mut file = dfhack_archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if outpath.extension().unwrap_or_default() != "proto" {
            continue;
        }

        let mut dest = PathBuf::from(out_dir.clone());
        dest.push(PathBuf::from(outpath.file_name().unwrap()));

        let mut outfile = File::create(&dest).unwrap();
        std::io::copy(&mut file, &mut outfile).unwrap();
        protos.push(dest);
    }
}
