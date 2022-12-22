// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{Animation, AnimationTarget, Easing};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "AdwTimedAnimation")]
    pub struct TimedAnimation(Object<ffi::AdwTimedAnimation, ffi::AdwTimedAnimationClass>) @extends Animation;

    match fn {
        type_ => || ffi::adw_timed_animation_get_type(),
    }
}

impl TimedAnimation {
    #[doc(alias = "adw_timed_animation_new")]
    pub fn new(
        widget: &impl IsA<gtk::Widget>,
        from: f64,
        to: f64,
        duration: u32,
        target: impl IsA<AnimationTarget>,
    ) -> TimedAnimation {
        skip_assert_initialized!();
        unsafe {
            Animation::from_glib_none(ffi::adw_timed_animation_new(
                widget.as_ref().to_glib_none().0,
                from,
                to,
                duration,
                target.upcast().into_glib_ptr(),
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`TimedAnimation`] objects.
    ///
    /// This method returns an instance of [`TimedAnimationBuilder`](crate::builders::TimedAnimationBuilder) which can be used to create [`TimedAnimation`] objects.
    pub fn builder() -> TimedAnimationBuilder {
        TimedAnimationBuilder::new()
    }

    #[doc(alias = "adw_timed_animation_get_alternate")]
    #[doc(alias = "get_alternate")]
    pub fn is_alternate(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_timed_animation_get_alternate(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_timed_animation_get_duration")]
    #[doc(alias = "get_duration")]
    pub fn duration(&self) -> u32 {
        unsafe { ffi::adw_timed_animation_get_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_timed_animation_get_easing")]
    #[doc(alias = "get_easing")]
    pub fn easing(&self) -> Easing {
        unsafe { from_glib(ffi::adw_timed_animation_get_easing(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_timed_animation_get_repeat_count")]
    #[doc(alias = "get_repeat_count")]
    pub fn repeat_count(&self) -> u32 {
        unsafe { ffi::adw_timed_animation_get_repeat_count(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_timed_animation_get_reverse")]
    #[doc(alias = "get_reverse")]
    pub fn is_reverse(&self) -> bool {
        unsafe { from_glib(ffi::adw_timed_animation_get_reverse(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_timed_animation_get_value_from")]
    #[doc(alias = "get_value_from")]
    pub fn value_from(&self) -> f64 {
        unsafe { ffi::adw_timed_animation_get_value_from(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_timed_animation_get_value_to")]
    #[doc(alias = "get_value_to")]
    pub fn value_to(&self) -> f64 {
        unsafe { ffi::adw_timed_animation_get_value_to(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_timed_animation_set_alternate")]
    pub fn set_alternate(&self, alternate: bool) {
        unsafe {
            ffi::adw_timed_animation_set_alternate(self.to_glib_none().0, alternate.into_glib());
        }
    }

    #[doc(alias = "adw_timed_animation_set_duration")]
    pub fn set_duration(&self, duration: u32) {
        unsafe {
            ffi::adw_timed_animation_set_duration(self.to_glib_none().0, duration);
        }
    }

    #[doc(alias = "adw_timed_animation_set_easing")]
    pub fn set_easing(&self, easing: Easing) {
        unsafe {
            ffi::adw_timed_animation_set_easing(self.to_glib_none().0, easing.into_glib());
        }
    }

    #[doc(alias = "adw_timed_animation_set_repeat_count")]
    pub fn set_repeat_count(&self, repeat_count: u32) {
        unsafe {
            ffi::adw_timed_animation_set_repeat_count(self.to_glib_none().0, repeat_count);
        }
    }

    #[doc(alias = "adw_timed_animation_set_reverse")]
    pub fn set_reverse(&self, reverse: bool) {
        unsafe {
            ffi::adw_timed_animation_set_reverse(self.to_glib_none().0, reverse.into_glib());
        }
    }

    #[doc(alias = "adw_timed_animation_set_value_from")]
    pub fn set_value_from(&self, value: f64) {
        unsafe {
            ffi::adw_timed_animation_set_value_from(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "adw_timed_animation_set_value_to")]
    pub fn set_value_to(&self, value: f64) {
        unsafe {
            ffi::adw_timed_animation_set_value_to(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "alternate")]
    pub fn connect_alternate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alternate_trampoline<F: Fn(&TimedAnimation) + 'static>(
            this: *mut ffi::AdwTimedAnimation,
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
                b"notify::alternate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alternate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "duration")]
    pub fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<F: Fn(&TimedAnimation) + 'static>(
            this: *mut ffi::AdwTimedAnimation,
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
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "easing")]
    pub fn connect_easing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_easing_trampoline<F: Fn(&TimedAnimation) + 'static>(
            this: *mut ffi::AdwTimedAnimation,
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
                b"notify::easing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_easing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "repeat-count")]
    pub fn connect_repeat_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_repeat_count_trampoline<F: Fn(&TimedAnimation) + 'static>(
            this: *mut ffi::AdwTimedAnimation,
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
                b"notify::repeat-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_repeat_count_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "reverse")]
    pub fn connect_reverse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reverse_trampoline<F: Fn(&TimedAnimation) + 'static>(
            this: *mut ffi::AdwTimedAnimation,
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
                b"notify::reverse\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reverse_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value-from")]
    pub fn connect_value_from_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_from_trampoline<F: Fn(&TimedAnimation) + 'static>(
            this: *mut ffi::AdwTimedAnimation,
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
                b"notify::value-from\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_from_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value-to")]
    pub fn connect_value_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_to_trampoline<F: Fn(&TimedAnimation) + 'static>(
            this: *mut ffi::AdwTimedAnimation,
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
                b"notify::value-to\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_to_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for TimedAnimation {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`TimedAnimation`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TimedAnimationBuilder {
    builder: glib::object::ObjectBuilder<'static, TimedAnimation>,
}

impl TimedAnimationBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn alternate(self, alternate: bool) -> Self {
        Self {
            builder: self.builder.property("alternate", alternate),
        }
    }

    pub fn duration(self, duration: u32) -> Self {
        Self {
            builder: self.builder.property("duration", duration),
        }
    }

    pub fn easing(self, easing: Easing) -> Self {
        Self {
            builder: self.builder.property("easing", easing),
        }
    }

    pub fn repeat_count(self, repeat_count: u32) -> Self {
        Self {
            builder: self.builder.property("repeat-count", repeat_count),
        }
    }

    pub fn reverse(self, reverse: bool) -> Self {
        Self {
            builder: self.builder.property("reverse", reverse),
        }
    }

    pub fn value_from(self, value_from: f64) -> Self {
        Self {
            builder: self.builder.property("value-from", value_from),
        }
    }

    pub fn value_to(self, value_to: f64) -> Self {
        Self {
            builder: self.builder.property("value-to", value_to),
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn follow_enable_animations_setting(self, follow_enable_animations_setting: bool) -> Self {
        Self {
            builder: self.builder.property(
                "follow-enable-animations-setting",
                follow_enable_animations_setting,
            ),
        }
    }

    pub fn target(self, target: &impl IsA<AnimationTarget>) -> Self {
        Self {
            builder: self.builder.property("target", target.clone().upcast()),
        }
    }

    pub fn widget(self, widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("widget", widget.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`TimedAnimation`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> TimedAnimation {
        self.builder.build()
    }
}

impl fmt::Display for TimedAnimation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TimedAnimation")
    }
}
