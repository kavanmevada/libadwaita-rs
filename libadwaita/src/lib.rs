extern crate bitflags;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate lazy_static;
extern crate libc;
extern crate pango;
// Re-export the -sys bindings
pub use ffi;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("Libhandy may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using libhandy.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use glib::Error;
mod auto;
pub use auto::functions::*;
pub use auto::*;

pub mod subclass;

pub mod prelude;
