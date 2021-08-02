use crate::Application;

use gtk::subclass::prelude::*;

pub trait AdwApplicationImpl: GtkApplicationImpl {}

unsafe impl<T: AdwApplicationImpl> IsSubclassable<T> for Application {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::Application as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gtk::Application as IsSubclassable<T>>::instance_init(instance);
    }
}
