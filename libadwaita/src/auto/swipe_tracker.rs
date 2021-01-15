// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::NavigationDirection;
use crate::Swipeable;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct SwipeTracker(Object<ffi::AdwSwipeTracker, ffi::AdwSwipeTrackerClass>) @implements gtk::Orientable;

    match fn {
        get_type => || ffi::adw_swipe_tracker_get_type(),
    }
}

impl SwipeTracker {
    #[doc(alias = "adw_swipe_tracker_new")]
    pub fn new<P: IsA<Swipeable>>(swipeable: &P) -> SwipeTracker {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::adw_swipe_tracker_new(
                swipeable.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_swipe_tracker_get_allow_mouse_drag")]
    pub fn get_allow_mouse_drag(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_swipe_tracker_get_allow_mouse_drag(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_swipe_tracker_get_enabled")]
    pub fn get_enabled(&self) -> bool {
        unsafe { from_glib(ffi::adw_swipe_tracker_get_enabled(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_swipe_tracker_get_reversed")]
    pub fn get_reversed(&self) -> bool {
        unsafe { from_glib(ffi::adw_swipe_tracker_get_reversed(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_swipe_tracker_get_swipeable")]
    pub fn get_swipeable(&self) -> Option<Swipeable> {
        unsafe { from_glib_none(ffi::adw_swipe_tracker_get_swipeable(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_swipe_tracker_set_allow_mouse_drag")]
    pub fn set_allow_mouse_drag(&self, allow_mouse_drag: bool) {
        unsafe {
            ffi::adw_swipe_tracker_set_allow_mouse_drag(
                self.to_glib_none().0,
                allow_mouse_drag.to_glib(),
            );
        }
    }

    #[doc(alias = "adw_swipe_tracker_set_enabled")]
    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::adw_swipe_tracker_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    #[doc(alias = "adw_swipe_tracker_set_reversed")]
    pub fn set_reversed(&self, reversed: bool) {
        unsafe {
            ffi::adw_swipe_tracker_set_reversed(self.to_glib_none().0, reversed.to_glib());
        }
    }

    #[doc(alias = "adw_swipe_tracker_shift_position")]
    pub fn shift_position(&self, delta: f64) {
        unsafe {
            ffi::adw_swipe_tracker_shift_position(self.to_glib_none().0, delta);
        }
    }

    pub fn connect_begin_swipe<F: Fn(&SwipeTracker, NavigationDirection, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn begin_swipe_trampoline<
            F: Fn(&SwipeTracker, NavigationDirection, bool) + 'static,
        >(
            this: *mut ffi::AdwSwipeTracker,
            direction: ffi::AdwNavigationDirection,
            direct: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(direction),
                from_glib(direct),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"begin-swipe\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    begin_swipe_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_end_swipe<F: Fn(&SwipeTracker, i64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn end_swipe_trampoline<F: Fn(&SwipeTracker, i64, f64) + 'static>(
            this: *mut ffi::AdwSwipeTracker,
            duration: i64,
            to: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), duration, to)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"end-swipe\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    end_swipe_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_update_swipe<F: Fn(&SwipeTracker, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn update_swipe_trampoline<F: Fn(&SwipeTracker, f64) + 'static>(
            this: *mut ffi::AdwSwipeTracker,
            progress: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), progress)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"update-swipe\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    update_swipe_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_allow_mouse_drag_notify<F: Fn(&SwipeTracker) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_allow_mouse_drag_trampoline<F: Fn(&SwipeTracker) + 'static>(
            this: *mut ffi::AdwSwipeTracker,
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
                b"notify::allow-mouse-drag\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_allow_mouse_drag_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_enabled_notify<F: Fn(&SwipeTracker) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<F: Fn(&SwipeTracker) + 'static>(
            this: *mut ffi::AdwSwipeTracker,
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
                b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enabled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_reversed_notify<F: Fn(&SwipeTracker) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_reversed_trampoline<F: Fn(&SwipeTracker) + 'static>(
            this: *mut ffi::AdwSwipeTracker,
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
                b"notify::reversed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reversed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct SwipeTrackerBuilder {
    allow_mouse_drag: Option<bool>,
    enabled: Option<bool>,
    reversed: Option<bool>,
    swipeable: Option<Swipeable>,
    orientation: Option<gtk::Orientation>,
}

impl SwipeTrackerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> SwipeTracker {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref allow_mouse_drag) = self.allow_mouse_drag {
            properties.push(("allow-mouse-drag", allow_mouse_drag));
        }
        if let Some(ref enabled) = self.enabled {
            properties.push(("enabled", enabled));
        }
        if let Some(ref reversed) = self.reversed {
            properties.push(("reversed", reversed));
        }
        if let Some(ref swipeable) = self.swipeable {
            properties.push(("swipeable", swipeable));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new::<SwipeTracker>(&properties).expect("object new");
        ret
    }

    pub fn allow_mouse_drag(mut self, allow_mouse_drag: bool) -> Self {
        self.allow_mouse_drag = Some(allow_mouse_drag);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    pub fn reversed(mut self, reversed: bool) -> Self {
        self.reversed = Some(reversed);
        self
    }

    pub fn swipeable<P: IsA<Swipeable>>(mut self, swipeable: &P) -> Self {
        self.swipeable = Some(swipeable.clone().upcast());
        self
    }

    pub fn orientation(mut self, orientation: gtk::Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for SwipeTracker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SwipeTracker")
    }
}
