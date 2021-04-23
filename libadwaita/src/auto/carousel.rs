// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

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

    #[doc(alias = "adw_carousel_append")]
    pub fn append<P: IsA<gtk::Widget>>(&self, child: &P) {
        unsafe {
            ffi::adw_carousel_append(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_carousel_get_allow_mouse_drag")]
    pub fn allows_mouse_drag(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_carousel_get_allow_mouse_drag(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_carousel_get_animation_duration")]
    pub fn animation_duration(&self) -> u32 {
        unsafe { ffi::adw_carousel_get_animation_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_carousel_get_interactive")]
    pub fn is_interactive(&self) -> bool {
        unsafe { from_glib(ffi::adw_carousel_get_interactive(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_carousel_get_n_pages")]
    pub fn n_pages(&self) -> u32 {
        unsafe { ffi::adw_carousel_get_n_pages(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_carousel_get_nth_page")]
    pub fn nth_page(&self, n: u32) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_carousel_get_nth_page(self.to_glib_none().0, n)) }
    }

    #[doc(alias = "adw_carousel_get_position")]
    pub fn position(&self) -> f64 {
        unsafe { ffi::adw_carousel_get_position(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_carousel_get_reveal_duration")]
    pub fn reveal_duration(&self) -> u32 {
        unsafe { ffi::adw_carousel_get_reveal_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_carousel_get_spacing")]
    pub fn spacing(&self) -> u32 {
        unsafe { ffi::adw_carousel_get_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_carousel_insert")]
    pub fn insert<P: IsA<gtk::Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::adw_carousel_insert(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "adw_carousel_prepend")]
    pub fn prepend<P: IsA<gtk::Widget>>(&self, child: &P) {
        unsafe {
            ffi::adw_carousel_prepend(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_carousel_remove")]
    pub fn remove<P: IsA<gtk::Widget>>(&self, child: &P) {
        unsafe {
            ffi::adw_carousel_remove(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_carousel_reorder")]
    pub fn reorder<P: IsA<gtk::Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::adw_carousel_reorder(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "adw_carousel_scroll_to")]
    pub fn scroll_to<P: IsA<gtk::Widget>>(&self, widget: &P) {
        unsafe {
            ffi::adw_carousel_scroll_to(self.to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_carousel_scroll_to_full")]
    pub fn scroll_to_full<P: IsA<gtk::Widget>>(&self, widget: &P, duration: i64) {
        unsafe {
            ffi::adw_carousel_scroll_to_full(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                duration,
            );
        }
    }

    #[doc(alias = "adw_carousel_set_allow_mouse_drag")]
    pub fn set_allow_mouse_drag(&self, allow_mouse_drag: bool) {
        unsafe {
            ffi::adw_carousel_set_allow_mouse_drag(
                self.to_glib_none().0,
                allow_mouse_drag.to_glib(),
            );
        }
    }

    #[doc(alias = "adw_carousel_set_animation_duration")]
    pub fn set_animation_duration(&self, duration: u32) {
        unsafe {
            ffi::adw_carousel_set_animation_duration(self.to_glib_none().0, duration);
        }
    }

    #[doc(alias = "adw_carousel_set_interactive")]
    pub fn set_interactive(&self, interactive: bool) {
        unsafe {
            ffi::adw_carousel_set_interactive(self.to_glib_none().0, interactive.to_glib());
        }
    }

    #[doc(alias = "adw_carousel_set_reveal_duration")]
    pub fn set_reveal_duration(&self, reveal_duration: u32) {
        unsafe {
            ffi::adw_carousel_set_reveal_duration(self.to_glib_none().0, reveal_duration);
        }
    }

    #[doc(alias = "adw_carousel_set_spacing")]
    pub fn set_spacing(&self, spacing: u32) {
        unsafe {
            ffi::adw_carousel_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    pub fn connect_page_changed<F: Fn(&Carousel, u32) + 'static>(&self, f: F) -> SignalHandlerId {
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

    pub fn connect_property_allow_mouse_drag_notify<F: Fn(&Carousel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
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

    pub fn connect_property_animation_duration_notify<F: Fn(&Carousel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_animation_duration_trampoline<F: Fn(&Carousel) + 'static>(
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
                b"notify::animation-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_animation_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_interactive_notify<F: Fn(&Carousel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
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

    pub fn connect_property_n_pages_notify<F: Fn(&Carousel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
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

    pub fn connect_property_position_notify<F: Fn(&Carousel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
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

    pub fn connect_property_reveal_duration_notify<F: Fn(&Carousel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
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

    pub fn connect_property_spacing_notify<F: Fn(&Carousel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
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

#[derive(Clone, Default)]
pub struct CarouselBuilder {
    allow_mouse_drag: Option<bool>,
    animation_duration: Option<u32>,
    interactive: Option<bool>,
    reveal_duration: Option<u32>,
    spacing: Option<u32>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<gtk::Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<gtk::LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<gtk::Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<gtk::Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<gtk::AccessibleRole>,
    orientation: Option<gtk::Orientation>,
}

impl CarouselBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Carousel {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref allow_mouse_drag) = self.allow_mouse_drag {
            properties.push(("allow-mouse-drag", allow_mouse_drag));
        }
        if let Some(ref animation_duration) = self.animation_duration {
            properties.push(("animation-duration", animation_duration));
        }
        if let Some(ref interactive) = self.interactive {
            properties.push(("interactive", interactive));
        }
        if let Some(ref reveal_duration) = self.reveal_duration {
            properties.push(("reveal-duration", reveal_duration));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new::<Carousel>(&properties).expect("object new");
        ret
    }

    pub fn allow_mouse_drag(mut self, allow_mouse_drag: bool) -> Self {
        self.allow_mouse_drag = Some(allow_mouse_drag);
        self
    }

    pub fn animation_duration(mut self, animation_duration: u32) -> Self {
        self.animation_duration = Some(animation_duration);
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = Some(interactive);
        self
    }

    pub fn reveal_duration(mut self, reveal_duration: u32) -> Self {
        self.reveal_duration = Some(reveal_duration);
        self
    }

    pub fn spacing(mut self, spacing: u32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: gtk::Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<gtk::LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: gtk::Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: gtk::Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: gtk::AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn orientation(mut self, orientation: gtk::Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for Carousel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Carousel")
    }
}