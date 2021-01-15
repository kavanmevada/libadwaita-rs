use crate::PreferencesPage;
use glib::subclass::prelude::*;
use gtk::subclass::widget::WidgetImpl;
use gtk::Widget;

pub trait PreferencesPageImpl: WidgetImpl {}

unsafe impl<T: PreferencesPageImpl> IsSubclassable<T> for PreferencesPage {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);
    }
}
