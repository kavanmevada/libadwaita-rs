// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::PreferencesPage;
use crate::Toast;
use crate::Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "AdwPreferencesWindow")]
    pub struct PreferencesWindow(Object<ffi::AdwPreferencesWindow, ffi::AdwPreferencesWindowClass>) @extends Window, gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;

    match fn {
        type_ => || ffi::adw_preferences_window_get_type(),
    }
}

impl PreferencesWindow {
    pub const NONE: Option<&'static PreferencesWindow> = None;

    #[doc(alias = "adw_preferences_window_new")]
    pub fn new() -> PreferencesWindow {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_preferences_window_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PreferencesWindow`] objects.
    ///
    /// This method returns an instance of [`PreferencesWindowBuilder`](crate::builders::PreferencesWindowBuilder) which can be used to create [`PreferencesWindow`] objects.
    pub fn builder() -> PreferencesWindowBuilder {
        PreferencesWindowBuilder::default()
    }
}

impl Default for PreferencesWindow {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PreferencesWindow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct PreferencesWindowBuilder {
    can_navigate_back: Option<bool>,
    search_enabled: Option<bool>,
    visible_page: Option<gtk::Widget>,
    visible_page_name: Option<String>,
    content: Option<gtk::Widget>,
    application: Option<gtk::Application>,
    decorated: Option<bool>,
    default_height: Option<i32>,
    default_widget: Option<gtk::Widget>,
    default_width: Option<i32>,
    deletable: Option<bool>,
    destroy_with_parent: Option<bool>,
    display: Option<gdk::Display>,
    focus_visible: Option<bool>,
    focus_widget: Option<gtk::Widget>,
    fullscreened: Option<bool>,
    #[cfg(any(feature = "gtk_v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v4_2")))]
    handle_menubar_accel: Option<bool>,
    hide_on_close: Option<bool>,
    icon_name: Option<String>,
    maximized: Option<bool>,
    mnemonics_visible: Option<bool>,
    modal: Option<bool>,
    resizable: Option<bool>,
    startup_id: Option<String>,
    title: Option<String>,
    #[cfg(any(feature = "gtk_v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v4_6")))]
    titlebar: Option<gtk::Widget>,
    transient_for: Option<gtk::Window>,
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

impl PreferencesWindowBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PreferencesWindowBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`PreferencesWindow`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> PreferencesWindow {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref can_navigate_back) = self.can_navigate_back {
            properties.push(("can-navigate-back", can_navigate_back));
        }
        if let Some(ref search_enabled) = self.search_enabled {
            properties.push(("search-enabled", search_enabled));
        }
        if let Some(ref visible_page) = self.visible_page {
            properties.push(("visible-page", visible_page));
        }
        if let Some(ref visible_page_name) = self.visible_page_name {
            properties.push(("visible-page-name", visible_page_name));
        }
        if let Some(ref content) = self.content {
            properties.push(("content", content));
        }
        if let Some(ref application) = self.application {
            properties.push(("application", application));
        }
        if let Some(ref decorated) = self.decorated {
            properties.push(("decorated", decorated));
        }
        if let Some(ref default_height) = self.default_height {
            properties.push(("default-height", default_height));
        }
        if let Some(ref default_widget) = self.default_widget {
            properties.push(("default-widget", default_widget));
        }
        if let Some(ref default_width) = self.default_width {
            properties.push(("default-width", default_width));
        }
        if let Some(ref deletable) = self.deletable {
            properties.push(("deletable", deletable));
        }
        if let Some(ref destroy_with_parent) = self.destroy_with_parent {
            properties.push(("destroy-with-parent", destroy_with_parent));
        }
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        if let Some(ref focus_visible) = self.focus_visible {
            properties.push(("focus-visible", focus_visible));
        }
        if let Some(ref focus_widget) = self.focus_widget {
            properties.push(("focus-widget", focus_widget));
        }
        if let Some(ref fullscreened) = self.fullscreened {
            properties.push(("fullscreened", fullscreened));
        }
        #[cfg(any(feature = "gtk_v4_2", feature = "dox"))]
        if let Some(ref handle_menubar_accel) = self.handle_menubar_accel {
            properties.push(("handle-menubar-accel", handle_menubar_accel));
        }
        if let Some(ref hide_on_close) = self.hide_on_close {
            properties.push(("hide-on-close", hide_on_close));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref maximized) = self.maximized {
            properties.push(("maximized", maximized));
        }
        if let Some(ref mnemonics_visible) = self.mnemonics_visible {
            properties.push(("mnemonics-visible", mnemonics_visible));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref startup_id) = self.startup_id {
            properties.push(("startup-id", startup_id));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        #[cfg(any(feature = "gtk_v4_6", feature = "dox"))]
        if let Some(ref titlebar) = self.titlebar {
            properties.push(("titlebar", titlebar));
        }
        if let Some(ref transient_for) = self.transient_for {
            properties.push(("transient-for", transient_for));
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
        glib::Object::new::<PreferencesWindow>(&properties)
            .expect("Failed to create an instance of PreferencesWindow")
    }

    pub fn can_navigate_back(mut self, can_navigate_back: bool) -> Self {
        self.can_navigate_back = Some(can_navigate_back);
        self
    }

    pub fn search_enabled(mut self, search_enabled: bool) -> Self {
        self.search_enabled = Some(search_enabled);
        self
    }

    pub fn visible_page(mut self, visible_page: &impl IsA<gtk::Widget>) -> Self {
        self.visible_page = Some(visible_page.clone().upcast());
        self
    }

    pub fn visible_page_name(mut self, visible_page_name: &str) -> Self {
        self.visible_page_name = Some(visible_page_name.to_string());
        self
    }

    pub fn content(mut self, content: &impl IsA<gtk::Widget>) -> Self {
        self.content = Some(content.clone().upcast());
        self
    }

    pub fn application(mut self, application: &impl IsA<gtk::Application>) -> Self {
        self.application = Some(application.clone().upcast());
        self
    }

    pub fn decorated(mut self, decorated: bool) -> Self {
        self.decorated = Some(decorated);
        self
    }

    pub fn default_height(mut self, default_height: i32) -> Self {
        self.default_height = Some(default_height);
        self
    }

    pub fn default_widget(mut self, default_widget: &impl IsA<gtk::Widget>) -> Self {
        self.default_widget = Some(default_widget.clone().upcast());
        self
    }

    pub fn default_width(mut self, default_width: i32) -> Self {
        self.default_width = Some(default_width);
        self
    }

    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = Some(deletable);
        self
    }

    pub fn destroy_with_parent(mut self, destroy_with_parent: bool) -> Self {
        self.destroy_with_parent = Some(destroy_with_parent);
        self
    }

    pub fn display(mut self, display: &gdk::Display) -> Self {
        self.display = Some(display.clone());
        self
    }

    pub fn focus_visible(mut self, focus_visible: bool) -> Self {
        self.focus_visible = Some(focus_visible);
        self
    }

    pub fn focus_widget(mut self, focus_widget: &impl IsA<gtk::Widget>) -> Self {
        self.focus_widget = Some(focus_widget.clone().upcast());
        self
    }

    pub fn fullscreened(mut self, fullscreened: bool) -> Self {
        self.fullscreened = Some(fullscreened);
        self
    }

    #[cfg(any(feature = "gtk_v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v4_2")))]
    pub fn handle_menubar_accel(mut self, handle_menubar_accel: bool) -> Self {
        self.handle_menubar_accel = Some(handle_menubar_accel);
        self
    }

    pub fn hide_on_close(mut self, hide_on_close: bool) -> Self {
        self.hide_on_close = Some(hide_on_close);
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = Some(maximized);
        self
    }

    pub fn mnemonics_visible(mut self, mnemonics_visible: bool) -> Self {
        self.mnemonics_visible = Some(mnemonics_visible);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn startup_id(mut self, startup_id: &str) -> Self {
        self.startup_id = Some(startup_id.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    #[cfg(any(feature = "gtk_v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gtk_v4_6")))]
    pub fn titlebar(mut self, titlebar: &impl IsA<gtk::Widget>) -> Self {
        self.titlebar = Some(titlebar.clone().upcast());
        self
    }

    pub fn transient_for(mut self, transient_for: &impl IsA<gtk::Window>) -> Self {
        self.transient_for = Some(transient_for.clone().upcast());
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
}

pub trait PreferencesWindowExt: 'static {
    #[doc(alias = "adw_preferences_window_add")]
    fn add(&self, page: &impl IsA<PreferencesPage>);

    #[doc(alias = "adw_preferences_window_add_toast")]
    fn add_toast(&self, toast: &Toast);

    #[doc(alias = "adw_preferences_window_close_subpage")]
    fn close_subpage(&self);

    #[doc(alias = "adw_preferences_window_get_can_navigate_back")]
    #[doc(alias = "get_can_navigate_back")]
    fn can_navigate_back(&self) -> bool;

    #[doc(alias = "adw_preferences_window_get_search_enabled")]
    #[doc(alias = "get_search_enabled")]
    fn is_search_enabled(&self) -> bool;

    #[doc(alias = "adw_preferences_window_get_visible_page")]
    #[doc(alias = "get_visible_page")]
    fn visible_page(&self) -> Option<PreferencesPage>;

    #[doc(alias = "adw_preferences_window_get_visible_page_name")]
    #[doc(alias = "get_visible_page_name")]
    fn visible_page_name(&self) -> Option<glib::GString>;

    #[doc(alias = "adw_preferences_window_present_subpage")]
    fn present_subpage(&self, subpage: &impl IsA<gtk::Widget>);

    #[doc(alias = "adw_preferences_window_remove")]
    fn remove(&self, page: &impl IsA<PreferencesPage>);

    #[doc(alias = "adw_preferences_window_set_can_navigate_back")]
    fn set_can_navigate_back(&self, can_navigate_back: bool);

    #[doc(alias = "adw_preferences_window_set_search_enabled")]
    fn set_search_enabled(&self, search_enabled: bool);

    #[doc(alias = "adw_preferences_window_set_visible_page")]
    fn set_visible_page(&self, page: &impl IsA<PreferencesPage>);

    #[doc(alias = "adw_preferences_window_set_visible_page_name")]
    fn set_visible_page_name(&self, name: &str);

    #[doc(alias = "can-navigate-back")]
    fn connect_can_navigate_back_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "search-enabled")]
    fn connect_search_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible-page")]
    fn connect_visible_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible-page-name")]
    fn connect_visible_page_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PreferencesWindow>> PreferencesWindowExt for O {
    fn add(&self, page: &impl IsA<PreferencesPage>) {
        unsafe {
            ffi::adw_preferences_window_add(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            );
        }
    }

    fn add_toast(&self, toast: &Toast) {
        unsafe {
            ffi::adw_preferences_window_add_toast(
                self.as_ref().to_glib_none().0,
                toast.to_glib_full(),
            );
        }
    }

    fn close_subpage(&self) {
        unsafe {
            ffi::adw_preferences_window_close_subpage(self.as_ref().to_glib_none().0);
        }
    }

    fn can_navigate_back(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_preferences_window_get_can_navigate_back(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_search_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_preferences_window_get_search_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn visible_page(&self) -> Option<PreferencesPage> {
        unsafe {
            from_glib_none(ffi::adw_preferences_window_get_visible_page(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn visible_page_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::adw_preferences_window_get_visible_page_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn present_subpage(&self, subpage: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_preferences_window_present_subpage(
                self.as_ref().to_glib_none().0,
                subpage.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove(&self, page: &impl IsA<PreferencesPage>) {
        unsafe {
            ffi::adw_preferences_window_remove(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_can_navigate_back(&self, can_navigate_back: bool) {
        unsafe {
            ffi::adw_preferences_window_set_can_navigate_back(
                self.as_ref().to_glib_none().0,
                can_navigate_back.into_glib(),
            );
        }
    }

    fn set_search_enabled(&self, search_enabled: bool) {
        unsafe {
            ffi::adw_preferences_window_set_search_enabled(
                self.as_ref().to_glib_none().0,
                search_enabled.into_glib(),
            );
        }
    }

    fn set_visible_page(&self, page: &impl IsA<PreferencesPage>) {
        unsafe {
            ffi::adw_preferences_window_set_visible_page(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_visible_page_name(&self, name: &str) {
        unsafe {
            ffi::adw_preferences_window_set_visible_page_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn connect_can_navigate_back_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_navigate_back_trampoline<
            P: IsA<PreferencesWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwPreferencesWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PreferencesWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-navigate-back\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_navigate_back_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_search_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_enabled_trampoline<
            P: IsA<PreferencesWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwPreferencesWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PreferencesWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_enabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_visible_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_page_trampoline<
            P: IsA<PreferencesWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwPreferencesWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PreferencesWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_page_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_visible_page_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_page_name_trampoline<
            P: IsA<PreferencesWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwPreferencesWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PreferencesWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-page-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_page_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PreferencesWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PreferencesWindow")
    }
}
