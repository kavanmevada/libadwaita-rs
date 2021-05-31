// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::TabView;
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
    pub struct TabBar(Object<ffi::AdwTabBar, ffi::AdwTabBarClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_tab_bar_get_type(),
    }
}

impl TabBar {
    #[doc(alias = "adw_tab_bar_new")]
    pub fn new() -> TabBar {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::adw_tab_bar_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`TabBar`].
    ///
    /// This method returns an instance of [`TabBarBuilder`] which can be used to create a [`TabBar`].
    pub fn builder() -> TabBarBuilder {
        TabBarBuilder::default()
    }

    #[doc(alias = "adw_tab_bar_get_autohide")]
    #[doc(alias = "get_autohide")]
    pub fn is_autohide(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_bar_get_autohide(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_bar_get_end_action_widget")]
    #[doc(alias = "get_end_action_widget")]
    pub fn end_action_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::adw_tab_bar_get_end_action_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_bar_get_expand_tabs")]
    #[doc(alias = "get_expand_tabs")]
    pub fn expands_tabs(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_bar_get_expand_tabs(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_bar_get_inverted")]
    #[doc(alias = "get_inverted")]
    pub fn is_inverted(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_bar_get_inverted(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_bar_get_is_overflowing")]
    #[doc(alias = "get_is_overflowing")]
    pub fn is_overflowing(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_bar_get_is_overflowing(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_bar_get_start_action_widget")]
    #[doc(alias = "get_start_action_widget")]
    pub fn start_action_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::adw_tab_bar_get_start_action_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_bar_get_tabs_revealed")]
    #[doc(alias = "get_tabs_revealed")]
    pub fn is_tabs_revealed(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_bar_get_tabs_revealed(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_bar_get_view")]
    #[doc(alias = "get_view")]
    pub fn view(&self) -> Option<TabView> {
        unsafe { from_glib_none(ffi::adw_tab_bar_get_view(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_bar_set_autohide")]
    pub fn set_autohide(&self, autohide: bool) {
        unsafe {
            ffi::adw_tab_bar_set_autohide(self.to_glib_none().0, autohide.into_glib());
        }
    }

    #[doc(alias = "adw_tab_bar_set_end_action_widget")]
    pub fn set_end_action_widget<P: IsA<gtk::Widget>>(&self, widget: Option<&P>) {
        unsafe {
            ffi::adw_tab_bar_set_end_action_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_bar_set_expand_tabs")]
    pub fn set_expand_tabs(&self, expand_tabs: bool) {
        unsafe {
            ffi::adw_tab_bar_set_expand_tabs(self.to_glib_none().0, expand_tabs.into_glib());
        }
    }

    #[doc(alias = "adw_tab_bar_set_inverted")]
    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::adw_tab_bar_set_inverted(self.to_glib_none().0, inverted.into_glib());
        }
    }

    #[doc(alias = "adw_tab_bar_set_start_action_widget")]
    pub fn set_start_action_widget<P: IsA<gtk::Widget>>(&self, widget: Option<&P>) {
        unsafe {
            ffi::adw_tab_bar_set_start_action_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_bar_set_view")]
    pub fn set_view(&self, view: Option<&TabView>) {
        unsafe {
            ffi::adw_tab_bar_set_view(self.to_glib_none().0, view.to_glib_none().0);
        }
    }

    #[doc(alias = "autohide")]
    pub fn connect_autohide_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autohide_trampoline<F: Fn(&TabBar) + 'static>(
            this: *mut ffi::AdwTabBar,
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
                b"notify::autohide\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_autohide_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "end-action-widget")]
    pub fn connect_end_action_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_end_action_widget_trampoline<F: Fn(&TabBar) + 'static>(
            this: *mut ffi::AdwTabBar,
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
                b"notify::end-action-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_end_action_widget_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "expand-tabs")]
    pub fn connect_expand_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expand_tabs_trampoline<F: Fn(&TabBar) + 'static>(
            this: *mut ffi::AdwTabBar,
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
                b"notify::expand-tabs\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expand_tabs_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "inverted")]
    pub fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&TabBar) + 'static>(
            this: *mut ffi::AdwTabBar,
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
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-overflowing")]
    pub fn connect_is_overflowing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_overflowing_trampoline<F: Fn(&TabBar) + 'static>(
            this: *mut ffi::AdwTabBar,
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
                b"notify::is-overflowing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_overflowing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "start-action-widget")]
    pub fn connect_start_action_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_action_widget_trampoline<F: Fn(&TabBar) + 'static>(
            this: *mut ffi::AdwTabBar,
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
                b"notify::start-action-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_action_widget_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tabs-revealed")]
    pub fn connect_tabs_revealed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tabs_revealed_trampoline<F: Fn(&TabBar) + 'static>(
            this: *mut ffi::AdwTabBar,
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
                b"notify::tabs-revealed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tabs_revealed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "view")]
    pub fn connect_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_trampoline<F: Fn(&TabBar) + 'static>(
            this: *mut ffi::AdwTabBar,
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
                b"notify::view\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_view_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`TabBar`].
pub struct TabBarBuilder {
    autohide: Option<bool>,
    end_action_widget: Option<gtk::Widget>,
    expand_tabs: Option<bool>,
    inverted: Option<bool>,
    start_action_widget: Option<gtk::Widget>,
    view: Option<TabView>,
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
}

impl TabBarBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`TabBarBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`TabBar`].
    pub fn build(self) -> TabBar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref autohide) = self.autohide {
            properties.push(("autohide", autohide));
        }
        if let Some(ref end_action_widget) = self.end_action_widget {
            properties.push(("end-action-widget", end_action_widget));
        }
        if let Some(ref expand_tabs) = self.expand_tabs {
            properties.push(("expand-tabs", expand_tabs));
        }
        if let Some(ref inverted) = self.inverted {
            properties.push(("inverted", inverted));
        }
        if let Some(ref start_action_widget) = self.start_action_widget {
            properties.push(("start-action-widget", start_action_widget));
        }
        if let Some(ref view) = self.view {
            properties.push(("view", view));
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
        glib::Object::new::<TabBar>(&properties).expect("Failed to create an instance of TabBar")
    }

    pub fn autohide(mut self, autohide: bool) -> Self {
        self.autohide = Some(autohide);
        self
    }

    pub fn end_action_widget<P: IsA<gtk::Widget>>(mut self, end_action_widget: &P) -> Self {
        self.end_action_widget = Some(end_action_widget.clone().upcast());
        self
    }

    pub fn expand_tabs(mut self, expand_tabs: bool) -> Self {
        self.expand_tabs = Some(expand_tabs);
        self
    }

    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = Some(inverted);
        self
    }

    pub fn start_action_widget<P: IsA<gtk::Widget>>(mut self, start_action_widget: &P) -> Self {
        self.start_action_widget = Some(start_action_widget.clone().upcast());
        self
    }

    pub fn view(mut self, view: &TabView) -> Self {
        self.view = Some(view.clone());
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
}

impl fmt::Display for TabBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TabBar")
    }
}
