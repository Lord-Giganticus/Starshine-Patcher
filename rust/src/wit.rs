use crate::consts::WIT_PATH;
use crate::region::Region;
use std::{process::Command, path::PathBuf};

pub(crate) fn extract() {
    let args = "extract -s ../ -1 -n SB4.01 . smg2.d --psel=DATA -ovv";
    Command::new(WIT_PATH).args(args.split(" ")).spawn().unwrap().wait().unwrap();
}

pub(crate) fn patchxml(dir: &PathBuf, reg: Region) {
    crate::consts::extractpatches();
    let mut wit = dir.clone();
    wit.push(WIT_PATH);
    let args = vec![String::from("dolpatch"),
    format!("loader\\{}.dol", reg), String::from("xml=patches.xml")];
    Command::new(wit).args(args).spawn().unwrap().wait().unwrap();
}

#[cfg(not(feature = "pyo3"))]
pub(crate) fn repack() {
    crate::consts::extractrepack();
    let mut p = std::env::current_dir().unwrap();
    p.push("Syati\\rustpython.exe");
    Command::new(p).arg("repack.py").spawn().unwrap().wait().unwrap();
}

#[cfg(feature = "pyo3")]
pub(crate) fn repack() {
    use pyo3::prelude::*;
    use crate::consts::REPACK;
    Python::with_gil(|py| {
        py.run(REPACK, None, None).unwrap();
    });
}

pub(crate) fn getregion() -> Region {
    let args = "ID6 -s ../ -1 -n SB4.01 .";
    let proc = Command::new(WIT_PATH).args(args.split(" ")).output().unwrap();
    let text = String::from_utf8(proc.stdout).unwrap();
    let text = text.chars().collect::<Vec<_>>().get(3).unwrap().to_string();
    let num: isize = text.parse().unwrap_or(69);
    num.into()
}