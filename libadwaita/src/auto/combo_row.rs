// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ActionRow;
use crate::PreferencesRow;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct ComboRow(Object<ffi::AdwComboRow, ffi::AdwComboRowClass>) @extends ActionRow, PreferencesRow, gtk::ListBoxRow, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;

    match fn {
        type_ => || ffi::adw_combo_row_get_type(),
    }
}

impl ComboRow {
    #[doc(alias = "adw_combo_row_new")]
    pub fn new() -> ComboRow {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_combo_row_new()).unsafe_cast() }
    }
}

impl Default for ComboRow {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_COMBO_ROW: Option<&ComboRow> = None;

pub trait ComboRowExt: 'static {
    #[doc(alias = "adw_combo_row_get_expression")]
    fn expression(&self) -> Option<gtk::Expression>;

    #[doc(alias = "adw_combo_row_get_factory")]
    fn factory(&self) -> Option<gtk::ListItemFactory>;

    #[doc(alias = "adw_combo_row_get_list_factory")]
    fn list_factory(&self) -> Option<gtk::ListItemFactory>;

    #[doc(alias = "adw_combo_row_get_model")]
    fn model(&self) -> Option<gio::ListModel>;

    #[doc(alias = "adw_combo_row_get_selected")]
    fn selected(&self) -> u32;

    #[doc(alias = "adw_combo_row_get_selected_item")]
    fn selected_item(&self) -> Option<glib::Object>;

    #[doc(alias = "adw_combo_row_get_use_subtitle")]
    fn uses_subtitle(&self) -> bool;

    #[doc(alias = "adw_combo_row_set_factory")]
    fn set_factory<P: IsA<gtk::ListItemFactory>>(&self, factory: Option<&P>);

    #[doc(alias = "adw_combo_row_set_list_factory")]
    fn set_list_factory<P: IsA<gtk::ListItemFactory>>(&self, factory: Option<&P>);

    #[doc(alias = "adw_combo_row_set_model")]
    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>);

    #[doc(alias = "adw_combo_row_set_selected")]
    fn set_selected(&self, position: u32);

    #[doc(alias = "adw_combo_row_set_use_subtitle")]
    fn set_use_subtitle(&self, use_subtitle: bool);

    fn connect_property_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_list_factory_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selected_item_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_use_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<ComboRow>> ComboRowExt for O {
    fn expression(&self) -> Option<gtk::Expression> {
        unsafe {
            from_glib_none(ffi::adw_combo_row_get_expression(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn factory(&self) -> Option<gtk::ListItemFactory> {
        unsafe {
            from_glib_none(ffi::adw_combo_row_get_factory(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_factory(&self) -> Option<gtk::ListItemFactory> {
        unsafe {
            from_glib_none(ffi::adw_combo_row_get_list_factory(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::adw_combo_row_get_model(self.as_ref().to_glib_none().0)) }
    }

    fn selected(&self) -> u32 {
        unsafe { ffi::adw_combo_row_get_selected(self.as_ref().to_glib_none().0) }
    }

    fn selected_item(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::adw_combo_row_get_selected_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uses_subtitle(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_combo_row_get_use_subtitle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_factory<P: IsA<gtk::ListItemFactory>>(&self, factory: Option<&P>) {
        unsafe {
            ffi::adw_combo_row_set_factory(
                self.as_ref().to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_list_factory<P: IsA<gtk::ListItemFactory>>(&self, factory: Option<&P>) {
        unsafe {
            ffi::adw_combo_row_set_list_factory(
                self.as_ref().to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            ffi::adw_combo_row_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_selected(&self, position: u32) {
        unsafe {
            ffi::adw_combo_row_set_selected(self.as_ref().to_glib_none().0, position);
        }
    }

    fn set_use_subtitle(&self, use_subtitle: bool) {
        unsafe {
            ffi::adw_combo_row_set_use_subtitle(
                self.as_ref().to_glib_none().0,
                use_subtitle.into_glib(),
            );
        }
    }

    fn connect_property_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ComboRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_factory_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ComboRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::factory\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_factory_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_list_factory_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_list_factory_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ComboRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::list-factory\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_list_factory_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ComboRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ComboRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_selected_item_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_item_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ComboRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected-item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_item_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_subtitle_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_subtitle_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwComboRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ComboRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ComboRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ComboRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ComboRow")
    }
}
