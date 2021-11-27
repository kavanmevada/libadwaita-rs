// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{AnimationTarget, CallbackAnimationTarget};
use glib::object::Cast;
use glib::translate::*;
use std::boxed::Box as Box_;

impl CallbackAnimationTarget {
    #[doc(alias = "adw_callback_animation_target_new")]
    pub fn new<F: Fn(f64) + 'static>(callback: F) -> Self {
        assert_initialized_main_thread!();
        let callback_data: Box_<F> = Box_::new(callback);
        unsafe extern "C" fn callback_func<F: Fn(f64) + 'static>(
            user_data: glib::ffi::gpointer,
            value: libc::c_double,
        ) {
            let callback: &F = &*(user_data as *mut _);
            callback(value)
        }

        unsafe extern "C" fn destroy_func<F: Fn(f64) + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<F> = Box_::from_raw(data as *mut _);
        }

        unsafe {
            AnimationTarget::from_glib_full(ffi::adw_callback_animation_target_new(
                Some(callback_func::<F>),
                Box_::into_raw(callback_data) as *mut _,
                Some(destroy_func::<F> as _),
            ))
            .unsafe_cast()
        }
    }
}
