// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::NavigationDirection;
use crate::SwipeTracker;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct Swipeable(Interface<ffi::AdwSwipeable, ffi::AdwSwipeableInterface>) @requires gtk::Widget, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_swipeable_get_type(),
    }
}

pub const NONE_SWIPEABLE: Option<&Swipeable> = None;

pub trait SwipeableExt: 'static {
    #[doc(alias = "adw_swipeable_emit_child_switched")]
    fn emit_child_switched(&self, index: u32, duration: i64);

    #[doc(alias = "adw_swipeable_get_cancel_progress")]
    fn cancel_progress(&self) -> f64;

    #[doc(alias = "adw_swipeable_get_distance")]
    fn distance(&self) -> f64;

    #[doc(alias = "adw_swipeable_get_progress")]
    fn progress(&self) -> f64;

    #[doc(alias = "adw_swipeable_get_snap_points")]
    fn snap_points(&self) -> Vec<f64>;

    #[doc(alias = "adw_swipeable_get_swipe_area")]
    fn swipe_area(
        &self,
        navigation_direction: NavigationDirection,
        is_drag: bool,
    ) -> gdk::Rectangle;

    #[doc(alias = "adw_swipeable_get_swipe_tracker")]
    fn swipe_tracker(&self) -> Option<SwipeTracker>;

    #[doc(alias = "adw_swipeable_switch_child")]
    fn switch_child(&self, index: u32, duration: i64);

    fn connect_child_switched<F: Fn(&Self, u32, i64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Swipeable>> SwipeableExt for O {
    fn emit_child_switched(&self, index: u32, duration: i64) {
        unsafe {
            ffi::adw_swipeable_emit_child_switched(self.as_ref().to_glib_none().0, index, duration);
        }
    }

    fn cancel_progress(&self) -> f64 {
        unsafe { ffi::adw_swipeable_get_cancel_progress(self.as_ref().to_glib_none().0) }
    }

    fn distance(&self) -> f64 {
        unsafe { ffi::adw_swipeable_get_distance(self.as_ref().to_glib_none().0) }
    }

    fn progress(&self) -> f64 {
        unsafe { ffi::adw_swipeable_get_progress(self.as_ref().to_glib_none().0) }
    }

    fn snap_points(&self) -> Vec<f64> {
        unsafe {
            let mut n_snap_points = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::adw_swipeable_get_snap_points(
                    self.as_ref().to_glib_none().0,
                    n_snap_points.as_mut_ptr(),
                ),
                n_snap_points.assume_init() as usize,
            );
            ret
        }
    }

    fn swipe_area(
        &self,
        navigation_direction: NavigationDirection,
        is_drag: bool,
    ) -> gdk::Rectangle {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            ffi::adw_swipeable_get_swipe_area(
                self.as_ref().to_glib_none().0,
                navigation_direction.to_glib(),
                is_drag.to_glib(),
                rect.to_glib_none_mut().0,
            );
            rect
        }
    }

    fn swipe_tracker(&self) -> Option<SwipeTracker> {
        unsafe {
            from_glib_none(ffi::adw_swipeable_get_swipe_tracker(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn switch_child(&self, index: u32, duration: i64) {
        unsafe {
            ffi::adw_swipeable_switch_child(self.as_ref().to_glib_none().0, index, duration);
        }
    }

    fn connect_child_switched<F: Fn(&Self, u32, i64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn child_switched_trampoline<P, F: Fn(&P, u32, i64) + 'static>(
            this: *mut ffi::AdwSwipeable,
            index: libc::c_uint,
            duration: i64,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Swipeable>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Swipeable::from_glib_borrow(this).unsafe_cast_ref(),
                index,
                duration,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-switched\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_switched_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Swipeable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Swipeable")
    }
}