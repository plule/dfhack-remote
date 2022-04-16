use std::path::PathBuf;

pub fn include_dir() -> &'static str {
    let out_dir = env!("OUT_DIR");
    out_dir
}

pub fn protos() -> Vec<PathBuf> {
    let pattern = concat!(env!("OUT_DIR"), "/*.proto");
    glob::glob(pattern).unwrap().map(|p| p.unwrap()).collect()
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
