use crate::ApplicationWindow;
use gtk::subclass::prelude::*;

pub trait AdwApplicationWindowImpl: ApplicationWindowImpl {}

unsafe impl<T: AdwApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::ApplicationWindow as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gtk::ApplicationWindow  as IsSubclassable<T>>::instance_init(instance);
    }
}
