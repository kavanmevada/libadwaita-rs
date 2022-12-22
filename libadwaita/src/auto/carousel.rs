// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{SpringParams, Swipeable};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "AdwCarousel")]
    pub struct Carousel(Object<ffi::AdwCarousel, ffi::AdwCarouselClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, Swipeable, gtk::Orientable;

    match fn {
        type_ => || ffi::adw_carousel_get_type(),
    }
}

impl Carousel {
    #[doc(alias = "adw_carousel_new")]
    pub fn new() -> Carousel {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_carousel_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Carousel`] objects.
    ///
    /// This method returns an instance of [`CarouselBuilder`](crate::builders::CarouselBuilder) which can be used to create [`Carousel`] objects.
    pub fn builder() -> CarouselBuilder {
        CarouselBuilder::new()
    }

    #[doc(alias = "adw_carousel_append")]
    pub fn append(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_carousel_append(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_carousel_get_allow_long_swipes")]
    #[doc(alias = "get_allow_long_swipes")]
    pub fn allows_long_swipes(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_carousel_get_allow_long_swipes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_carousel_get_allow_mouse_drag")]
    #[doc(alias = "get_allow_mouse_drag")]
    pub fn allows_mouse_drag(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_carousel_get_allow_mouse_drag(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_carousel_get_allow_scroll_wheel")]
    #[doc(alias = "get_allow_scroll_wheel")]
    pub fn allows_scroll_wheel(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_carousel_get_allow_scroll_wheel(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_carousel_get_interactive")]
    #[doc(alias = "get_interactive")]
    pub fn is_interactive(&self) -> bool {
        unsafe { from_glib(ffi::adw_carousel_get_interactive(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_carousel_get_n_pages")]
    #[doc(alias = "get_n_pages")]
    pub fn n_pages(&self) -> u32 {
        unsafe { ffi::adw_carousel_get_n_pages(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_carousel_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self) -> f64 {
        unsafe { ffi::adw_carousel_get_position(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_carousel_get_reveal_duration")]
    #[doc(alias = "get_reveal_duration")]
    pub fn reveal_duration(&self) -> u32 {
        unsafe { ffi::adw_carousel_get_reveal_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_carousel_get_scroll_params")]
    #[doc(alias = "get_scroll_params")]
    pub fn scroll_params(&self) -> SpringParams {
        unsafe { from_glib_full(ffi::adw_carousel_get_scroll_params(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_carousel_get_spacing")]
    #[doc(alias = "get_spacing")]
    pub fn spacing(&self) -> u32 {
        unsafe { ffi::adw_carousel_get_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_carousel_insert")]
    pub fn insert(&self, child: &impl IsA<gtk::Widget>, position: i32) {
        unsafe {
            ffi::adw_carousel_insert(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "adw_carousel_prepend")]
    pub fn prepend(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_carousel_prepend(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_carousel_remove")]
    pub fn remove(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_carousel_remove(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_carousel_reorder")]
    pub fn reorder(&self, child: &impl IsA<gtk::Widget>, position: i32) {
        unsafe {
            ffi::adw_carousel_reorder(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "adw_carousel_scroll_to")]
    pub fn scroll_to(&self, widget: &impl IsA<gtk::Widget>, animate: bool) {
        unsafe {
            ffi::adw_carousel_scroll_to(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                animate.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_carousel_set_allow_long_swipes")]
    pub fn set_allow_long_swipes(&self, allow_long_swipes: bool) {
        unsafe {
            ffi::adw_carousel_set_allow_long_swipes(
                self.to_glib_none().0,
                allow_long_swipes.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_carousel_set_allow_mouse_drag")]
    pub fn set_allow_mouse_drag(&self, allow_mouse_drag: bool) {
        unsafe {
            ffi::adw_carousel_set_allow_mouse_drag(
                self.to_glib_none().0,
                allow_mouse_drag.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_carousel_set_allow_scroll_wheel")]
    pub fn set_allow_scroll_wheel(&self, allow_scroll_wheel: bool) {
        unsafe {
            ffi::adw_carousel_set_allow_scroll_wheel(
                self.to_glib_none().0,
                allow_scroll_wheel.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_carousel_set_interactive")]
    pub fn set_interactive(&self, interactive: bool) {
        unsafe {
            ffi::adw_carousel_set_interactive(self.to_glib_none().0, interactive.into_glib());
        }
    }

    #[doc(alias = "adw_carousel_set_reveal_duration")]
    pub fn set_reveal_duration(&self, reveal_duration: u32) {
        unsafe {
            ffi::adw_carousel_set_reveal_duration(self.to_glib_none().0, reveal_duration);
        }
    }

    #[doc(alias = "adw_carousel_set_scroll_params")]
    pub fn set_scroll_params(&self, params: &SpringParams) {
        unsafe {
            ffi::adw_carousel_set_scroll_params(self.to_glib_none().0, params.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_carousel_set_spacing")]
    pub fn set_spacing(&self, spacing: u32) {
        unsafe {
            ffi::adw_carousel_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "page-changed")]
    pub fn connect_page_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_changed_trampoline<F: Fn(&Carousel, u32) + 'static>(
            this: *mut ffi::AdwCarousel,
            index: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), index)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "allow-long-swipes")]
    pub fn connect_allow_long_swipes_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_allow_long_swipes_trampoline<F: Fn(&Carousel) + 'static>(
            this: *mut ffi::AdwCarousel,
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
                b"notify::allow-long-swipes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_allow_long_swipes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "allow-mouse-drag")]
    pub fn connect_allow_mouse_drag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_allow_mouse_drag_trampoline<F: Fn(&Carousel) + 'static>(
            this: *mut ffi::AdwCarousel,
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

    #[doc(alias = "allow-scroll-wheel")]
    pub fn connect_allow_scroll_wheel_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_allow_scroll_wheel_trampoline<F: Fn(&Carousel) + 'static>(
            this: *mut ffi::AdwCarousel,
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
                b"notify::allow-scroll-wheel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_allow_scroll_wheel_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "interactive")]
    pub fn connect_interactive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interactive_trampoline<F: Fn(&Carousel) + 'static>(
            this: *mut ffi::AdwCarousel,
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
                b"notify::interactive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interactive_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "n-pages")]
    pub fn connect_n_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_pages_trampoline<F: Fn(&Carousel) + 'static>(
            this: *mut ffi::AdwCarousel,
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
                b"notify::n-pages\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position")]
    pub fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&Carousel) + 'static>(
            this: *mut ffi::AdwCarousel,
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
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "reveal-duration")]
    pub fn connect_reveal_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_duration_trampoline<F: Fn(&Carousel) + 'static>(
            this: *mut ffi::AdwCarousel,
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
                b"notify::reveal-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll-params")]
    pub fn connect_scroll_params_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scroll_params_trampoline<F: Fn(&Carousel) + 'static>(
            this: *mut ffi::AdwCarousel,
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
                b"notify::scroll-params\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scroll_params_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "spacing")]
    pub fn connect_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<F: Fn(&Carousel) + 'static>(
            this: *mut ffi::AdwCarousel,
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
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Carousel {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Carousel`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CarouselBuilder {
    builder: glib::object::ObjectBuilder<'static, Carousel>,
}

impl CarouselBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn allow_long_swipes(self, allow_long_swipes: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("allow-long-swipes", allow_long_swipes),
        }
    }

    pub fn allow_mouse_drag(self, allow_mouse_drag: bool) -> Self {
        Self {
            builder: self.builder.property("allow-mouse-drag", allow_mouse_drag),
        }
    }

    pub fn allow_scroll_wheel(self, allow_scroll_wheel: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("allow-scroll-wheel", allow_scroll_wheel),
        }
    }

    pub fn interactive(self, interactive: bool) -> Self {
        Self {
            builder: self.builder.property("interactive", interactive),
        }
    }

    pub fn reveal_duration(self, reveal_duration: u32) -> Self {
        Self {
            builder: self.builder.property("reveal-duration", reveal_duration),
        }
    }

    pub fn scroll_params(self, scroll_params: &SpringParams) -> Self {
        Self {
            builder: self
                .builder
                .property("scroll-params", scroll_params.clone()),
        }
    }

    pub fn spacing(self, spacing: u32) -> Self {
        Self {
            builder: self.builder.property("spacing", spacing),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: gtk::AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn orientation(self, orientation: gtk::Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Carousel`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Carousel {
        self.builder.build()
    }
}

impl fmt::Display for Carousel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Carousel")
    }
}
