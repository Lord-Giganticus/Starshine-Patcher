use thread_control::make_pair;
use indicatif::*;
use reqwest::blocking;
use reqwest::IntoUrl;
use std::fs::OpenOptions;
use std::thread;
use std::io;
use std::process::Command;
use std::env::current_dir;
use std::{path::Path, fs};

pub(crate) fn downloadfile<T: IntoUrl + Copy>(url: T, prog: &ProgressBar, last: bool) -> String {
    let mut req = blocking::get(url).unwrap();
    let msg = String::from(url.as_str());
    let rid = msg.rfind('/').unwrap();
    let msg = &msg[rid+1..];
    let mut file = OpenOptions::new().write(true).create(true).read(true).open(&msg).unwrap();
    let len = req.content_length().unwrap();
    prog.set_length(len);
    prog.set_style(ProgressStyle::default_bar()
    .progress_chars("#>-")
    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
    .unwrap());
    let (f, c) = make_pair();
    let h = thread::spawn(move || {
        while f.alive() {
            io::copy(&mut req, &mut file).unwrap();
            break;
        }
    });
    // It's likely a byte or two has been written at this point.
    let meta = std::fs::metadata;
    while !c.is_done() {
        prog.set_position(meta(msg).unwrap().len());
    }
    prog.set_position(prog.length().unwrap());
    if last {
        prog.finish();
    }
    h.join().unwrap();
    msg.to_owned()
}

const SETUP: &str = "https://github.com/Lord-Giganticus/SyatiSetup/releases/download/Auto/syatisetup.exe";

#[cfg(not(feature = "pyo3"))]
const RUSTPYTHON: &str = "https://cdn.discordapp.com/attachments/713385142277767200/1006382389561397359/rustpython.exe";

#[cfg(not(feature = "pyo3"))]
pub(crate) fn getrustpython(prog: &mut ProgressBar) {
    *prog = ProgressBar::new(0);
    downloadfile(RUSTPYTHON, prog, false);
}

pub(crate) fn usesyatisetup(prog: &ProgressBar) {
    let name = downloadfile(SETUP, prog, true);
    // Hide stdout to hide that we are downloading CW :troll:
    let mut p = current_dir().unwrap();
    p.push(name);
    std::fs::create_dir_all("deps").unwrap();
    std::env::set_current_dir("deps").unwrap();
    Command::new(p).output().unwrap();
}

pub(crate) fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}