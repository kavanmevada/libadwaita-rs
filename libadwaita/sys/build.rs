// Generated by gir (https://github.com/gtk-rs/gir @ a69abbe5ee1a)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 46f24acbe4c2)
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
