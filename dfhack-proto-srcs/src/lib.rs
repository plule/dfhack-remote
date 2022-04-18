#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::path::PathBuf;

/// Include directory when building the protobuf (protoc -I option)
pub fn include_dir() -> &'static str {
    let out_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src/protos");
    out_dir
}

/// List of extracted .proto files
pub fn protos() -> Vec<PathBuf> {
    let pattern = concat!(env!("CARGO_MANIFEST_DIR"), "/src/protos/*.proto");
    let mut paths: Vec<PathBuf> = glob::glob(pattern).unwrap().map(|p| p.unwrap()).collect();
    paths.sort();
    paths
}

#[cfg(test)]
mod tests {
    use crate::protos;
    #[test]
    fn has_protos() {
        assert!(protos().len() > 0)
    }

    #[test]
    fn protos_exist() {
        for proto in protos() {
            assert!(proto.exists());
        }
    }
}
