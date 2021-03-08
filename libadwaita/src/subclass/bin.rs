use crate::Bin;
use glib::subclass::prelude::*;
use gtk::subclass::prelude::WidgetImpl;
use gtk::Widget;

pub trait BinImpl: WidgetImpl {}

unsafe impl<T: BinImpl> IsSubclassable<T> for Bin {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);
    }
}
