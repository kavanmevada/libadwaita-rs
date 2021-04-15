use crate::PreferencesGroup;
use glib::subclass::prelude::*;
use gtk::subclass::widget::WidgetImpl;
use gtk::Widget;

pub trait PreferencesGroupImpl: WidgetImpl {}

unsafe impl<T: PreferencesGroupImpl> IsSubclassable<T> for PreferencesGroup {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}
