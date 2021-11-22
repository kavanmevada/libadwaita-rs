// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::FoldThresholdPolicy;
use crate::SqueezerPage;
use crate::SqueezerTransitionType;
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
    #[doc(alias = "AdwSqueezer")]
    pub struct Squeezer(Object<ffi::AdwSqueezer, ffi::AdwSqueezerClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

    match fn {
        type_ => || ffi::adw_squeezer_get_type(),
    }
}

impl Squeezer {
    #[doc(alias = "adw_squeezer_new")]
    pub fn new() -> Squeezer {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_squeezer_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Squeezer`] objects.
    ///
    /// This method returns an instance of [`SqueezerBuilder`](crate::builders::SqueezerBuilder) which can be used to create [`Squeezer`] objects.
    pub fn builder() -> SqueezerBuilder {
        SqueezerBuilder::default()
    }

    #[doc(alias = "adw_squeezer_add")]
    pub fn add(&self, child: &impl IsA<gtk::Widget>) -> Option<SqueezerPage> {
        unsafe {
            from_glib_none(ffi::adw_squeezer_add(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_squeezer_get_allow_none")]
    #[doc(alias = "get_allow_none")]
    pub fn allows_none(&self) -> bool {
        unsafe { from_glib(ffi::adw_squeezer_get_allow_none(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_squeezer_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    pub fn is_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::adw_squeezer_get_homogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_squeezer_get_interpolate_size")]
    #[doc(alias = "get_interpolate_size")]
    pub fn is_interpolate_size(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_squeezer_get_interpolate_size(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_squeezer_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self, child: &impl IsA<gtk::Widget>) -> Option<SqueezerPage> {
        unsafe {
            from_glib_none(ffi::adw_squeezer_get_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_squeezer_get_pages")]
    #[doc(alias = "get_pages")]
    pub fn pages(&self) -> Option<gtk::SelectionModel> {
        unsafe { from_glib_full(ffi::adw_squeezer_get_pages(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_squeezer_get_switch_threshold_policy")]
    #[doc(alias = "get_switch_threshold_policy")]
    pub fn switch_threshold_policy(&self) -> FoldThresholdPolicy {
        unsafe {
            from_glib(ffi::adw_squeezer_get_switch_threshold_policy(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_squeezer_get_transition_duration")]
    #[doc(alias = "get_transition_duration")]
    pub fn transition_duration(&self) -> u32 {
        unsafe { ffi::adw_squeezer_get_transition_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_squeezer_get_transition_running")]
    #[doc(alias = "get_transition_running")]
    pub fn is_transition_running(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_squeezer_get_transition_running(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_squeezer_get_transition_type")]
    #[doc(alias = "get_transition_type")]
    pub fn transition_type(&self) -> SqueezerTransitionType {
        unsafe { from_glib(ffi::adw_squeezer_get_transition_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_squeezer_get_visible_child")]
    #[doc(alias = "get_visible_child")]
    pub fn visible_child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_squeezer_get_visible_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_squeezer_get_xalign")]
    #[doc(alias = "get_xalign")]
    pub fn xalign(&self) -> f32 {
        unsafe { ffi::adw_squeezer_get_xalign(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_squeezer_get_yalign")]
    #[doc(alias = "get_yalign")]
    pub fn yalign(&self) -> f32 {
        unsafe { ffi::adw_squeezer_get_yalign(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_squeezer_remove")]
    pub fn remove(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_squeezer_remove(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_squeezer_set_allow_none")]
    pub fn set_allow_none(&self, allow_none: bool) {
        unsafe {
            ffi::adw_squeezer_set_allow_none(self.to_glib_none().0, allow_none.into_glib());
        }
    }

    #[doc(alias = "adw_squeezer_set_homogeneous")]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::adw_squeezer_set_homogeneous(self.to_glib_none().0, homogeneous.into_glib());
        }
    }

    #[doc(alias = "adw_squeezer_set_interpolate_size")]
    pub fn set_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            ffi::adw_squeezer_set_interpolate_size(
                self.to_glib_none().0,
                interpolate_size.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_squeezer_set_switch_threshold_policy")]
    pub fn set_switch_threshold_policy(&self, policy: FoldThresholdPolicy) {
        unsafe {
            ffi::adw_squeezer_set_switch_threshold_policy(
                self.to_glib_none().0,
                policy.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_squeezer_set_transition_duration")]
    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::adw_squeezer_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[doc(alias = "adw_squeezer_set_transition_type")]
    pub fn set_transition_type(&self, transition: SqueezerTransitionType) {
        unsafe {
            ffi::adw_squeezer_set_transition_type(self.to_glib_none().0, transition.into_glib());
        }
    }

    #[doc(alias = "adw_squeezer_set_xalign")]
    pub fn set_xalign(&self, xalign: f32) {
        unsafe {
            ffi::adw_squeezer_set_xalign(self.to_glib_none().0, xalign);
        }
    }

    #[doc(alias = "adw_squeezer_set_yalign")]
    pub fn set_yalign(&self, yalign: f32) {
        unsafe {
            ffi::adw_squeezer_set_yalign(self.to_glib_none().0, yalign);
        }
    }

    #[doc(alias = "allow-none")]
    pub fn connect_allow_none_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_allow_none_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::allow-none\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_allow_none_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "homogeneous")]
    pub fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "interpolate-size")]
    pub fn connect_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interpolate_size_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::interpolate-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interpolate_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pages")]
    pub fn connect_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::pages\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "switch-threshold-policy")]
    pub fn connect_switch_threshold_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_switch_threshold_policy_trampoline<
            F: Fn(&Squeezer) + 'static,
        >(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::switch-threshold-policy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_switch_threshold_policy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-duration")]
    pub fn connect_transition_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_duration_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::transition-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-running")]
    pub fn connect_transition_running_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_running_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::transition-running\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_running_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-type")]
    pub fn connect_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_type_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::transition-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-child")]
    pub fn connect_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::visible-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "xalign")]
    pub fn connect_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xalign_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::xalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xalign_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "yalign")]
    pub fn connect_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_yalign_trampoline<F: Fn(&Squeezer) + 'static>(
            this: *mut ffi::AdwSqueezer,
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
                b"notify::yalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_yalign_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Squeezer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Squeezer`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct SqueezerBuilder {
    allow_none: Option<bool>,
    homogeneous: Option<bool>,
    interpolate_size: Option<bool>,
    switch_threshold_policy: Option<FoldThresholdPolicy>,
    transition_duration: Option<u32>,
    transition_type: Option<SqueezerTransitionType>,
    xalign: Option<f32>,
    yalign: Option<f32>,
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

impl SqueezerBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`SqueezerBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Squeezer`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> Squeezer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref allow_none) = self.allow_none {
            properties.push(("allow-none", allow_none));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref interpolate_size) = self.interpolate_size {
            properties.push(("interpolate-size", interpolate_size));
        }
        if let Some(ref switch_threshold_policy) = self.switch_threshold_policy {
            properties.push(("switch-threshold-policy", switch_threshold_policy));
        }
        if let Some(ref transition_duration) = self.transition_duration {
            properties.push(("transition-duration", transition_duration));
        }
        if let Some(ref transition_type) = self.transition_type {
            properties.push(("transition-type", transition_type));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
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
        glib::Object::new::<Squeezer>(&properties)
            .expect("Failed to create an instance of Squeezer")
    }

    pub fn allow_none(mut self, allow_none: bool) -> Self {
        self.allow_none = Some(allow_none);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn interpolate_size(mut self, interpolate_size: bool) -> Self {
        self.interpolate_size = Some(interpolate_size);
        self
    }

    pub fn switch_threshold_policy(mut self, switch_threshold_policy: FoldThresholdPolicy) -> Self {
        self.switch_threshold_policy = Some(switch_threshold_policy);
        self
    }

    pub fn transition_duration(mut self, transition_duration: u32) -> Self {
        self.transition_duration = Some(transition_duration);
        self
    }

    pub fn transition_type(mut self, transition_type: SqueezerTransitionType) -> Self {
        self.transition_type = Some(transition_type);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
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

    pub fn layout_manager(mut self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
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

impl fmt::Display for Squeezer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Squeezer")
    }
}
