use crate::Window;
use glib::subclass::prelude::*;

pub trait WindowImpl: gtk::subclass::window::WindowImpl {}

unsafe impl<T: WindowImpl> IsSubclassable<T> for Window {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <gtk::Window as IsSubclassable<T>>::override_vfuncs(class);
    }
}
