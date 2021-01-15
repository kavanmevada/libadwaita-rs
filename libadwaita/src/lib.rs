// Re-export the -sys bindings
pub use ffi;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("libadwaita may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using libadwaita.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use glib::Error;
#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
#[allow(clippy::type_complexity)]
mod auto;
pub use auto::functions::*;
pub use auto::*;

pub mod subclass;

pub mod prelude;
