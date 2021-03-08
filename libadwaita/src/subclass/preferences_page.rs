use crate::PreferencesPage;
use glib::subclass::prelude::*;
use gtk::subclass::widget::WidgetImpl;
use gtk::Widget;

pub trait PreferencesPageImpl: WidgetImpl {}

unsafe impl<T: PreferencesPageImpl> IsSubclassable<T> for PreferencesPage {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);
    }
}
