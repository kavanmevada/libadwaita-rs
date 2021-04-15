use crate::PreferencesRow;
use glib::subclass::prelude::*;
use gtk::subclass::list_box_row::ListBoxRowImpl;
use gtk::ListBoxRow;

pub trait PreferencesRowImpl: ListBoxRowImpl {}

unsafe impl<T: PreferencesRowImpl> IsSubclassable<T> for PreferencesRow {
    fn class_init(class: &mut glib::Class<Self>) {
        <ListBoxRow as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <ListBoxRow as IsSubclassable<T>>::instance_init(instance);
    }
}
