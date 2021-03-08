use crate::Window;
use gtk::subclass::prelude::*;

pub trait AdwWindowImpl: WindowImpl {}

unsafe impl<T: AdwWindowImpl> IsSubclassable<T> for Window {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::Window as IsSubclassable<T>>::class_init(class);
    }
}
