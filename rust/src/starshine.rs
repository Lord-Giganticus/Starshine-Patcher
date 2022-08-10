use std::fs::rename;
use std::process::Command;

pub(crate) fn copy_files() {
    crate::consts::extractcopy();
    rename("Syati\\rustpython.exe", "rustpython.exe").unwrap();
    let mut p = std::env::current_dir().unwrap();
    p.push("rustpython.exe");
    Command::new(p).arg("copy.py").spawn().unwrap().wait().unwrap();
    rename("rustpython.exe", "Syati\\rustpython.exe").unwrap();
}