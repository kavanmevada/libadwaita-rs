// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 46f24ac)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}
