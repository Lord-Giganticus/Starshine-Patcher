use std::path::Path;
use std::process::Command;

use path_absolutize::*;

pub(crate) fn extractarchive<S: AsRef<Path>>(s: &S) {
    let path = s.as_ref();
    let path = path.absolutize().unwrap();
    let path = path.as_ref();
    let sevenz = Path::new("7z.exe");
    if !sevenz.exists() {
        crate::consts::extractsevenzip();
    }
    Command::new("7z.exe").arg("x").arg(path).spawn().unwrap().wait().unwrap();
}