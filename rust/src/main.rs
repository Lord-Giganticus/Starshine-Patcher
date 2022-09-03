mod funcs;
mod sevenzip;
mod consts;
mod wit;
mod region;
mod syati;
mod starshine;
use git2::Repository;

const SMS: &str = "https://github.com/SPG64/Super-Mario-Starshine/releases/download/v1.4/Super.Mario.Starshine.Demo.v1.4.zip";
const SYATI: &str = "https://github.com/SunakazeKun/Syati.git";
const WIT: &str = "https://cdn.discordapp.com/attachments/886616711925751829/993881870666305578/wit.zip";

#[test]
fn d() {
    delete();
}

fn delete() {
    std::fs::remove_dir_all("Syati").unwrap_or_default();
    std::fs::remove_dir_all("wit").unwrap_or_default();
    std::fs::remove_dir_all("Super Mario Starshine Demo v1.4").unwrap_or_default();
    std::fs::remove_dir_all("smg2.d").unwrap_or_default();
    std::fs::remove_file("Super.Mario.Starshine.Demo.v1.4.zip").unwrap_or_default();
    std::fs::remove_file("wit.zip").unwrap_or_default();
    std::fs::remove_file("repack.py").unwrap_or_default();
}

fn main() {
    // Windows sanity check.
    if !cfg!(target_os = "windows") {
        panic!("{} is NOT supported ATM due to a build step requiring a windows only program.",
        std::env::consts::OS);
    }
    println!("Downloading deps...");
    let mut path = std::env::current_dir().unwrap();
    path.push("Syati");
    if path.exists() && path.is_dir() {
        std::fs::remove_dir_all(&path).unwrap_or_default();
    }
    Repository::clone(SYATI, path).unwrap();
    let mut prog = indicatif::ProgressBar::new(0);
    let msg = funcs::downloadfile(SMS, &prog, false);
    sevenzip::extractarchive(&msg);
    prog = indicatif::ProgressBar::new(0);
    let msg = funcs::downloadfile(WIT, &prog, false);
    sevenzip::extractarchive(&msg);
    let dir = std::env::current_dir().unwrap();
    std::env::set_current_dir("Syati").unwrap();
    prog = indicatif::ProgressBar::new(0);
    funcs::getrustpython(&prog);
    prog = indicatif::ProgressBar::new(0);
    funcs::usesyatisetup(&prog);
    std::env::set_current_dir(&dir).unwrap();
    println!("Extracting SMG2...");
    wit::extract();
    let region = wit::getregion();
    println!("Patching SMG2...");
    syati::copydol(region);
    std::env::set_current_dir("Syati").unwrap();
    syati::patch(region);
    wit::patchxml(&dir, region);
    std::env::set_current_dir(dir).unwrap();
    syati::copyback(region);
    println!("Copying over Starshine files...");
    starshine::copy_files();
    println!("Rebuilding SMG2...");
    wit::repack();
    delete();
    println!("Complete!!");
}
