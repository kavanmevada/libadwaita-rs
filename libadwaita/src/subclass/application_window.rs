use crate::ApplicationWindow;
use gtk::subclass::prelude::*;

pub trait AdwApplicationWindowImpl: ApplicationWindowImpl {}

unsafe impl<T: AdwApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <gtk::ApplicationWindow as IsSubclassable<T>>::override_vfuncs(class);
    }
}
