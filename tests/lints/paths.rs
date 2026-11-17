use std::fmt::Display;
use std::path::Path;

pub fn error<E: Display>(path: &Path) -> impl Fn(E) -> String + '_ {
    move |e| format!("{}: {e}", path.display())
}
