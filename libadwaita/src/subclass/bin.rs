use crate::Bin;
use glib::subclass::prelude::*;
use gtk::subclass::prelude::WidgetImpl;
use gtk::Widget;

pub trait BinImpl: WidgetImpl {}

unsafe impl<T: BinImpl> IsSubclassable<T> for Bin {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);
    }
}
