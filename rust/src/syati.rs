#[cfg(not(feature = "pyo3"))]
use std::{process::Command, env::current_dir};
use std::fs::copy;
use crate::region::Region;

#[cfg(not(feature = "pyo3"))]
pub(crate) fn patch(reg: Region) {
    let mut dir = current_dir().unwrap();
    dir.push("rustpython.exe");
    let args = format!("buildloader.py {}", reg);
    Command::new(dir).args(args.split(" ")).spawn().unwrap().wait().unwrap();
}

#[cfg(feature = "pyo3")]
pub(crate) fn patch(reg: Region) {
    use pyo3::ffi::*;
    use std::ffi::CString;
    use widestring::*;
    use libc::fopen;
    let mut name = u16cstr!("buildloader.py").to_owned();
    unsafe { Py_SetProgramName(name.as_ptr()) };
    unsafe { Py_Initialize() };
    let mut arg = U16CString::from_str(format!("{}", reg)).unwrap();
    let mut argv = vec![name.as_mut_ptr(), arg.as_mut_ptr()];
    unsafe { PySys_SetArgv(argv.len() as i32, argv.as_mut_ptr()) };
    let name = CString::new("buildloader.py").unwrap();
    let mode = CString::new("r").unwrap();
    let file = unsafe { fopen(name.as_ptr(), mode.as_ptr()) };
    unsafe { PyRun_SimpleFile(file, name.as_ptr()) };
    unsafe { Py_Finalize() };
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