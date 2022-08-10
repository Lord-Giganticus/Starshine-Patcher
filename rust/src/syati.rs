use std::process::Command;
use std::env::current_dir;
use std::fs::copy;
use crate::region::Region;

pub(crate) fn patch(reg: Region) {
    let mut dir = current_dir().unwrap();
    dir.push("rustpython.exe");
    let args = format!("buildloader.py {}", reg);
    Command::new(dir).args(args.split(" ")).spawn().unwrap().wait().unwrap();
}

pub(crate) fn copydol(reg: Region) {
    std::fs::create_dir_all("Syati\\dols").unwrap();
    let rfile = format!("Syati\\dols\\{}.dol", reg);
    let ifile = "smg2.d\\sys\\main.dol";
    copy(ifile, rfile).unwrap();
}

pub(crate) fn copyback(reg: Region) {
    let ifile = format!("Syati\\loader\\{}.dol", reg);
    let rfile = "smg2.d\\sys\\main.dol";
    copy(ifile, rfile).unwrap();
}