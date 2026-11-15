use std::path::{Path, PathBuf};

pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub fn absolute(path: &Path) -> PathBuf {
    Path::new(ROOT).join(path)
}
