use std::path::Path;
use std::fs::File;
use std::env;

use zip::ZipArchive;

pub(crate) fn extractarchive<S: AsRef<Path>>(s: S) {
    let file = File::open(s).unwrap();
    let mut arch = ZipArchive::new(file).unwrap();
    let dir = env::current_dir().unwrap();
    arch.extract(dir).unwrap();
}