// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "AdwViewStackPage")]
    pub struct ViewStackPage(Object<ffi::AdwViewStackPage, ffi::AdwViewStackPageClass>);

    match fn {
        type_ => || ffi::adw_view_stack_page_get_type(),
    }
}

impl ViewStackPage {
    #[doc(alias = "adw_view_stack_page_get_badge_number")]
    #[doc(alias = "get_badge_number")]
    pub fn badge_number(&self) -> u32 {
        unsafe { ffi::adw_view_stack_page_get_badge_number(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_view_stack_page_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_view_stack_page_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_view_stack_page_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::adw_view_stack_page_get_icon_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_page_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_view_stack_page_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_view_stack_page_get_needs_attention")]
    #[doc(alias = "get_needs_attention")]
    pub fn needs_attention(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_view_stack_page_get_needs_attention(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_page_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_view_stack_page_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_view_stack_page_get_use_underline")]
    #[doc(alias = "get_use_underline")]
    pub fn uses_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_view_stack_page_get_use_underline(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_page_get_visible")]
    #[doc(alias = "get_visible")]
    pub fn is_visible(&self) -> bool {
        unsafe { from_glib(ffi::adw_view_stack_page_get_visible(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_view_stack_page_set_badge_number")]
    pub fn set_badge_number(&self, badge_number: u32) {
        unsafe {
            ffi::adw_view_stack_page_set_badge_number(self.to_glib_none().0, badge_number);
        }
    }

    #[doc(alias = "adw_view_stack_page_set_icon_name")]
    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::adw_view_stack_page_set_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_view_stack_page_set_name")]
    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::adw_view_stack_page_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_view_stack_page_set_needs_attention")]
    pub fn set_needs_attention(&self, needs_attention: bool) {
        unsafe {
            ffi::adw_view_stack_page_set_needs_attention(
                self.to_glib_none().0,
                needs_attention.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_view_stack_page_set_title")]
    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::adw_view_stack_page_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_view_stack_page_set_use_underline")]
    pub fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::adw_view_stack_page_set_use_underline(
                self.to_glib_none().0,
                use_underline.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_view_stack_page_set_visible")]
    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::adw_view_stack_page_set_visible(self.to_glib_none().0, visible.into_glib());
        }
    }

    #[doc(alias = "badge-number")]
    pub fn connect_badge_number_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_badge_number_trampoline<F: Fn(&ViewStackPage) + 'static>(
            this: *mut ffi::AdwViewStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::badge-number\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_badge_number_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&ViewStackPage) + 'static>(
            this: *mut ffi::AdwViewStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&ViewStackPage) + 'static>(
            this: *mut ffi::AdwViewStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "needs-attention")]
    pub fn connect_needs_attention_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_needs_attention_trampoline<F: Fn(&ViewStackPage) + 'static>(
            this: *mut ffi::AdwViewStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::needs-attention\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_needs_attention_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ViewStackPage) + 'static>(
            this: *mut ffi::AdwViewStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-underline")]
    pub fn connect_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<F: Fn(&ViewStackPage) + 'static>(
            this: *mut ffi::AdwViewStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible")]
    pub fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<F: Fn(&ViewStackPage) + 'static>(
            this: *mut ffi::AdwViewStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ViewStackPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ViewStackPage")
    }
}
