use super::action_row::ActionRowImpl;
use crate::{ActionRow, ComboRow};
use glib::subclass::prelude::*;

pub trait ComboRowImpl: ActionRowImpl {}

unsafe impl<T: ComboRowImpl> IsSubclassable<T> for ComboRow {
    fn class_init(class: &mut glib::Class<Self>) {
        <ActionRow as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <ActionRow as IsSubclassable<T>>::instance_init(instance);
    }
}
