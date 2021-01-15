// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Window(Object<ffi::AdwWindow, ffi::AdwWindowClass>) @extends gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;

    match fn {
        get_type => || ffi::adw_window_get_type(),
    }
}

impl Window {
    #[doc(alias = "adw_window_new")]
    pub fn new() -> Window {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_full(ffi::adw_window_new()).unsafe_cast() }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_WINDOW: Option<&Window> = None;

pub trait WindowExt: 'static {
    #[doc(alias = "adw_window_get_child")]
    fn get_child(&self) -> Option<gtk::Widget>;

    #[doc(alias = "adw_window_set_child")]
    fn set_child<P: IsA<gtk::Widget>>(&self, child: Option<&P>);
}

impl<O: IsA<Window>> WindowExt for O {
    fn get_child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_window_get_child(self.as_ref().to_glib_none().0)) }
    }

    fn set_child<P: IsA<gtk::Widget>>(&self, child: Option<&P>) {
        unsafe {
            ffi::adw_window_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Window")
    }
}
