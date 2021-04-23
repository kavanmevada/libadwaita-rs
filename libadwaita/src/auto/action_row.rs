// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::PreferencesRow;
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
    pub struct ActionRow(Object<ffi::AdwActionRow, ffi::AdwActionRowClass>) @extends PreferencesRow, gtk::ListBoxRow, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;

    match fn {
        type_ => || ffi::adw_action_row_get_type(),
    }
}

impl ActionRow {
    #[doc(alias = "adw_action_row_new")]
    pub fn new() -> ActionRow {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_action_row_new()).unsafe_cast() }
    }
}

impl Default for ActionRow {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct ActionRowBuilder {
    activatable_widget: Option<gtk::Widget>,
    icon_name: Option<String>,
    subtitle: Option<String>,
    subtitle_lines: Option<i32>,
    title_lines: Option<i32>,
    use_underline: Option<bool>,
    title: Option<String>,
    activatable: Option<bool>,
    child: Option<gtk::Widget>,
    selectable: Option<bool>,
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
    action_name: Option<String>,
    action_target: Option<glib::Variant>,
}

impl ActionRowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ActionRow {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref activatable_widget) = self.activatable_widget {
            properties.push(("activatable-widget", activatable_widget));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref subtitle) = self.subtitle {
            properties.push(("subtitle", subtitle));
        }
        if let Some(ref subtitle_lines) = self.subtitle_lines {
            properties.push(("subtitle-lines", subtitle_lines));
        }
        if let Some(ref title_lines) = self.title_lines {
            properties.push(("title-lines", title_lines));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref activatable) = self.activatable {
            properties.push(("activatable", activatable));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref selectable) = self.selectable {
            properties.push(("selectable", selectable));
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
        if let Some(ref action_name) = self.action_name {
            properties.push(("action-name", action_name));
        }
        if let Some(ref action_target) = self.action_target {
            properties.push(("action-target", action_target));
        }
        let ret = glib::Object::new::<ActionRow>(&properties).expect("object new");
        ret
    }

    pub fn activatable_widget<P: IsA<gtk::Widget>>(mut self, activatable_widget: &P) -> Self {
        self.activatable_widget = Some(activatable_widget.clone().upcast());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn subtitle_lines(mut self, subtitle_lines: i32) -> Self {
        self.subtitle_lines = Some(subtitle_lines);
        self
    }

    pub fn title_lines(mut self, title_lines: i32) -> Self {
        self.title_lines = Some(title_lines);
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn activatable(mut self, activatable: bool) -> Self {
        self.activatable = Some(activatable);
        self
    }

    pub fn child<P: IsA<gtk::Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
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

    pub fn action_name(mut self, action_name: &str) -> Self {
        self.action_name = Some(action_name.to_string());
        self
    }

    pub fn action_target(mut self, action_target: &glib::Variant) -> Self {
        self.action_target = Some(action_target.clone());
        self
    }
}

pub const NONE_ACTION_ROW: Option<&ActionRow> = None;

pub trait ActionRowExt: 'static {
    #[doc(alias = "adw_action_row_activate")]
    fn activate(&self);

    #[doc(alias = "adw_action_row_add_prefix")]
    fn add_prefix<P: IsA<gtk::Widget>>(&self, widget: &P);

    #[doc(alias = "adw_action_row_add_suffix")]
    fn add_suffix<P: IsA<gtk::Widget>>(&self, widget: &P);

    #[doc(alias = "adw_action_row_get_activatable_widget")]
    fn activatable_widget(&self) -> Option<gtk::Widget>;

    #[doc(alias = "adw_action_row_get_icon_name")]
    fn icon_name(&self) -> Option<glib::GString>;

    #[doc(alias = "adw_action_row_get_subtitle")]
    fn subtitle(&self) -> Option<glib::GString>;

    #[doc(alias = "adw_action_row_get_subtitle_lines")]
    fn subtitle_lines(&self) -> i32;

    #[doc(alias = "adw_action_row_get_title_lines")]
    fn title_lines(&self) -> i32;

    #[doc(alias = "adw_action_row_remove")]
    fn remove<P: IsA<gtk::Widget>>(&self, widget: &P);

    #[doc(alias = "adw_action_row_set_activatable_widget")]
    fn set_activatable_widget<P: IsA<gtk::Widget>>(&self, widget: Option<&P>);

    #[doc(alias = "adw_action_row_set_icon_name")]
    fn set_icon_name(&self, icon_name: &str);

    #[doc(alias = "adw_action_row_set_subtitle")]
    fn set_subtitle(&self, subtitle: Option<&str>);

    #[doc(alias = "adw_action_row_set_subtitle_lines")]
    fn set_subtitle_lines(&self, subtitle_lines: i32);

    #[doc(alias = "adw_action_row_set_title_lines")]
    fn set_title_lines(&self, title_lines: i32);

    fn connect_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_activatable_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subtitle_lines_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_title_lines_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<ActionRow>> ActionRowExt for O {
    fn activate(&self) {
        unsafe {
            ffi::adw_action_row_activate(self.as_ref().to_glib_none().0);
        }
    }

    fn add_prefix<P: IsA<gtk::Widget>>(&self, widget: &P) {
        unsafe {
            ffi::adw_action_row_add_prefix(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn add_suffix<P: IsA<gtk::Widget>>(&self, widget: &P) {
        unsafe {
            ffi::adw_action_row_add_suffix(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn activatable_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::adw_action_row_get_activatable_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::adw_action_row_get_icon_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn subtitle(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::adw_action_row_get_subtitle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn subtitle_lines(&self) -> i32 {
        unsafe { ffi::adw_action_row_get_subtitle_lines(self.as_ref().to_glib_none().0) }
    }

    fn title_lines(&self) -> i32 {
        unsafe { ffi::adw_action_row_get_title_lines(self.as_ref().to_glib_none().0) }
    }

    fn remove<P: IsA<gtk::Widget>>(&self, widget: &P) {
        unsafe {
            ffi::adw_action_row_remove(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_activatable_widget<P: IsA<gtk::Widget>>(&self, widget: Option<&P>) {
        unsafe {
            ffi::adw_action_row_set_activatable_widget(
                self.as_ref().to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::adw_action_row_set_icon_name(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    fn set_subtitle(&self, subtitle: Option<&str>) {
        unsafe {
            ffi::adw_action_row_set_subtitle(
                self.as_ref().to_glib_none().0,
                subtitle.to_glib_none().0,
            );
        }
    }

    fn set_subtitle_lines(&self, subtitle_lines: i32) {
        unsafe {
            ffi::adw_action_row_set_subtitle_lines(self.as_ref().to_glib_none().0, subtitle_lines);
        }
    }

    fn set_title_lines(&self, title_lines: i32) {
        unsafe {
            ffi::adw_action_row_set_title_lines(self.as_ref().to_glib_none().0, title_lines);
        }
    }

    fn connect_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activated_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwActionRow,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ActionRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_activatable_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwActionRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ActionRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::activatable-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activatable_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwActionRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ActionRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwActionRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ActionRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_subtitle_lines_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_lines_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwActionRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ActionRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle-lines\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_lines_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_title_lines_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_lines_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwActionRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ActionRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title-lines\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_lines_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwActionRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ActionRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ActionRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ActionRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ActionRow")
    }
}