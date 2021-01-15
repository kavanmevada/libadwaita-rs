use glib::subclass::prelude::*;

use crate::ApplicationWindow;

pub trait ApplicationWindowImpl: gtk::subclass::application_window::ApplicationWindowImpl {}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <gtk::ApplicationWindow as IsSubclassable<T>>::override_vfuncs(class);
    }
}
