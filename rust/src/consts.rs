pub(crate) const PATCHES: &[u8] = include_bytes!("..\\patches.xml");

#[cfg(not(feature = "pyo3"))]
pub(crate) const REPACK: &[u8] = include_bytes!("..\\repack.py");

#[cfg(feature = "pyo3")]
pub(crate) const REPACK: &str = include_str!("..\\repack.py");

pub(crate) const WIT_PATH: &str = "wit\\wit.exe";

use std::fs::OpenOptions;
use std::io::Cursor;
use std::io;

macro_rules! impl_extract {
    ($item:expr, $name:ident, $file:expr) => {
        pub(crate) fn $name() {
            let mut stream = Cursor::new($item);
            let mut file = OpenOptions::new().write(true).create(true).truncate(true).
            open($file).unwrap();
            io::copy(&mut stream, &mut file).unwrap();
        }
    };
}

impl_extract!(PATCHES, extractpatches, "patches.xml");
#[cfg(not(feature = "pyo3"))]
impl_extract!(REPACK, extractrepack, "repack.py");