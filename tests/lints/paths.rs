use std::fmt::Display;
use std::path::{Path, PathBuf};

pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub fn absolute(path: &Path) -> PathBuf {
    Path::new(ROOT).join(path)
}

pub fn error<E: Display>(path: &Path) -> impl Fn(E) -> String + '_ {
    move |e| format!("{}: {e}", path.display())
}
