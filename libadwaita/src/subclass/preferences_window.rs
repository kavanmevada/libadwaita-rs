use super::window::AdwWindowImpl;
use crate::{PreferencesWindow, Window};
use glib::subclass::prelude::*;

pub trait PreferencesWindowImpl: AdwWindowImpl {}

unsafe impl<T: PreferencesWindowImpl> IsSubclassable<T> for PreferencesWindow {
    fn class_init(class: &mut glib::Class<Self>) {
        <Window as IsSubclassable<T>>::class_init(class);
    }
}
