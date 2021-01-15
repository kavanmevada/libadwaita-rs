use crate::PreferencesRow;
use glib::subclass::prelude::*;
use gtk::subclass::list_box_row::ListBoxRowImpl;
use gtk::ListBoxRow;

pub trait PreferencesRowImpl: ListBoxRowImpl {}

unsafe impl<T: PreferencesRowImpl> IsSubclassable<T> for PreferencesRow {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <ListBoxRow as IsSubclassable<T>>::override_vfuncs(class);
    }
}
