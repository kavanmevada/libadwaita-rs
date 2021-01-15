use crate::Window;
use gtk::subclass::prelude::*;

pub trait AdwWindowImpl: WindowImpl {}

unsafe impl<T: AdwWindowImpl> IsSubclassable<T> for Window {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <gtk::Window as IsSubclassable<T>>::override_vfuncs(class);
    }
}
