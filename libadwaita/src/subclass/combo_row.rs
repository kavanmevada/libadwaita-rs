use super::action_row::ActionRowImpl;
use crate::{ActionRow, ComboRow};
use glib::subclass::prelude::*;

pub trait ComboRowImpl: ActionRowImpl {}

unsafe impl<T: ComboRowImpl> IsSubclassable<T> for ComboRow {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <ActionRow as IsSubclassable<T>>::override_vfuncs(class);
    }
}
