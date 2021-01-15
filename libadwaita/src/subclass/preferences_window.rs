use super::window::AdwWindowImpl;
use crate::{PreferencesWindow, Window};
use glib::subclass::prelude::*;

pub trait PreferencesWindowImpl: AdwWindowImpl {}

unsafe impl<T: PreferencesWindowImpl> IsSubclassable<T> for PreferencesWindow {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Window as IsSubclassable<T>>::override_vfuncs(class);
    }
}
