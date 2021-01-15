use crate::PreferencesGroup;
use glib::subclass::prelude::*;
use gtk::subclass::widget::WidgetImpl;
use gtk::Widget;

pub trait PreferencesGroupImpl: WidgetImpl {}

unsafe impl<T: PreferencesGroupImpl> IsSubclassable<T> for PreferencesGroup {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);
    }
}
