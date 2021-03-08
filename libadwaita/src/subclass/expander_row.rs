use super::preferences_row::PreferencesRowImpl;
use crate::{ExpanderRow, PreferencesRow};
use glib::subclass::prelude::*;

pub trait ExpanderRowImpl: PreferencesRowImpl {}

unsafe impl<T: ExpanderRowImpl> IsSubclassable<T> for ExpanderRow {
    fn class_init(class: &mut glib::Class<Self>) {
        <PreferencesRow as IsSubclassable<T>>::class_init(class);
    }
}
