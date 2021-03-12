use crate::Window;
use gtk::subclass::prelude::*;

pub trait AdwWindowImpl: WindowImpl {}

unsafe impl<T: AdwWindowImpl> IsSubclassable<T> for Window {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::Window as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gtk::Window  as IsSubclassable<T>>::instance_init(instance);
    }
}
