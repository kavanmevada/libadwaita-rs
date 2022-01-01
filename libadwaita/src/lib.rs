#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::needless_doctest_main)]
#![doc(
    html_logo_url = "https://gitlab.gnome.org/GNOME/libadwaita/-/raw/main/doc/libadwaita.svg",
    html_favicon_url = "https://gitlab.gnome.org/GNOME/libadwaita/-/raw/main/demo/data/org.gnome.Adwaita1.Demo-symbolic.svg"
)]
//! # Rust Adwaita bindings
//!
//! This library contains safe Rust bindings for [Adwaita](https://gitlab.gnome.org/GNOME/libadwaita), a library that offers
//! building blocks for modern GNOME applications.
//!
//! See also
//!
//! - [GTK 4 Rust bindings documentation](mod@gtk)
//! - [Libadwaita documentation](https://gnome.pages.gitlab.gnome.org/libadwaita/)
//! - [gtk-rs project overview](https://gtk-rs.org/)
//!
//! # Example
//!
//! Adwaita needs to be initialized before use by calling [`fn@init`] on
//! [`startup`](fn@gio::prelude::ApplicationExt::connect_startup).
//!
//! The [`libadwaita`](mod@crate) crate is usually renamed to `adw`. You can
//! do this globally in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies.adw]
//! package = "libadwaita"
//! version = "0.x.y"
//! ```
//!
//! ```no_run
//! # use libadwaita as adw;
//! use adw::prelude::*;
//!
//! use adw::{ActionRow, ApplicationWindow, HeaderBar};
//! use gtk::{Application, Box, ListBox, Orientation};
//!
//! fn main() {
//!     let application = Application::builder()
//!         .application_id("com.example.FirstAdwaitaApp")
//!         .build();
//!
//!     application.connect_startup(|_| {
//!         adw::init();
//!     });
//!
//!     application.connect_activate(|app| {
//!         // ActionRows are only available in Adwaita
//!         let row = ActionRow::builder()
//!             .activatable(true)
//!             .selectable(false)
//!             .title("Click me")
//!             .build();
//!         row.connect_activated(|_| {
//!             eprintln!("Clicked!");
//!         });
//!
//!         let list = ListBox::builder()
//!             .margin_top(32)
//!             .margin_end(32)
//!             .margin_bottom(32)
//!             .margin_start(32)
//!             // the content class makes the list look nicer
//!             .css_classes(vec![String::from("content")])
//!             .build();
//!         list.append(&row);
//!
//!         // Combine the content in a box
//!         let content = Box::new(Orientation::Vertical, 0);
//!         // Adwaitas' ApplicationWindow does not include a HeaderBar
//!         content.append(
//!             &HeaderBar::builder()
//!                 .title_widget(&adw::WindowTitle::new("First App", ""))
//!                 .build(),
//!         );
//!         content.append(&list);
//!
//!         let window = ApplicationWindow::builder()
//!             .application(app)
//!             .default_width(350)
//!             // add content to window
//!             .content(&content)
//!             .build();
//!         window.show();
//!     });
//!
//!     application.run();
//! }
//! ```

// Re-export the -sys bindings
pub use ffi;
#[doc(hidden)]
pub use gtk;

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

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
#[allow(clippy::type_complexity)]
mod auto;

mod application;
mod tab_bar;

pub use auto::functions::*;
pub use auto::*;

pub mod builders;
pub mod prelude;
pub mod subclass;
