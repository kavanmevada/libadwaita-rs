// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::TabPage;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use crate::TabViewShortcuts;
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
    #[doc(alias = "AdwTabView")]
    pub struct TabView(Object<ffi::AdwTabView, ffi::AdwTabViewClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_tab_view_get_type(),
    }
}

impl TabView {
    #[doc(alias = "adw_tab_view_new")]
    pub fn new() -> TabView {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::adw_tab_view_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`TabView`] objects.
    ///
    /// This method returns an instance of [`TabViewBuilder`](crate::builders::TabViewBuilder) which can be used to create [`TabView`] objects.
    pub fn builder() -> TabViewBuilder {
        TabViewBuilder::default()
    }

    #[doc(alias = "adw_tab_view_add_page")]
    pub fn add_page(&self, child: &impl IsA<gtk::Widget>, parent: Option<&TabPage>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_add_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                parent.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_tab_view_add_shortcuts")]
    pub fn add_shortcuts(&self, shortcuts: TabViewShortcuts) {
        unsafe {
            ffi::adw_tab_view_add_shortcuts(self.to_glib_none().0, shortcuts.into_glib());
        }
    }

    #[doc(alias = "adw_tab_view_append")]
    pub fn append(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_append(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_append_pinned")]
    pub fn append_pinned(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_append_pinned(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_close_other_pages")]
    pub fn close_other_pages(&self, page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_close_other_pages(self.to_glib_none().0, page.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_view_close_page")]
    pub fn close_page(&self, page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_close_page(self.to_glib_none().0, page.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_view_close_page_finish")]
    pub fn close_page_finish(&self, page: &TabPage, confirm: bool) {
        unsafe {
            ffi::adw_tab_view_close_page_finish(
                self.to_glib_none().0,
                page.to_glib_none().0,
                confirm.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_view_close_pages_after")]
    pub fn close_pages_after(&self, page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_close_pages_after(self.to_glib_none().0, page.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_view_close_pages_before")]
    pub fn close_pages_before(&self, page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_close_pages_before(self.to_glib_none().0, page.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_view_get_default_icon")]
    #[doc(alias = "get_default_icon")]
    pub fn default_icon(&self) -> gio::Icon {
        unsafe { from_glib_none(ffi::adw_tab_view_get_default_icon(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_get_is_transferring_page")]
    #[doc(alias = "get_is_transferring_page")]
    pub fn is_transferring_page(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_get_is_transferring_page(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_get_menu_model")]
    #[doc(alias = "get_menu_model")]
    pub fn menu_model(&self) -> Option<gio::MenuModel> {
        unsafe { from_glib_none(ffi::adw_tab_view_get_menu_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_get_n_pages")]
    #[doc(alias = "get_n_pages")]
    pub fn n_pages(&self) -> i32 {
        unsafe { ffi::adw_tab_view_get_n_pages(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_tab_view_get_n_pinned_pages")]
    #[doc(alias = "get_n_pinned_pages")]
    pub fn n_pinned_pages(&self) -> i32 {
        unsafe { ffi::adw_tab_view_get_n_pinned_pages(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_tab_view_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_get_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_get_page_position")]
    #[doc(alias = "get_page_position")]
    pub fn page_position(&self, page: &TabPage) -> i32 {
        unsafe { ffi::adw_tab_view_get_page_position(self.to_glib_none().0, page.to_glib_none().0) }
    }

    #[doc(alias = "adw_tab_view_get_pages")]
    #[doc(alias = "get_pages")]
    pub fn pages(&self) -> gtk::SelectionModel {
        unsafe { from_glib_full(ffi::adw_tab_view_get_pages(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_get_selected_page")]
    #[doc(alias = "get_selected_page")]
    pub fn selected_page(&self) -> Option<TabPage> {
        unsafe { from_glib_none(ffi::adw_tab_view_get_selected_page(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_tab_view_get_shortcuts")]
    #[doc(alias = "get_shortcuts")]
    pub fn shortcuts(&self) -> TabViewShortcuts {
        unsafe { from_glib(ffi::adw_tab_view_get_shortcuts(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_insert")]
    pub fn insert(&self, child: &impl IsA<gtk::Widget>, position: i32) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_insert(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_insert_pinned")]
    pub fn insert_pinned(&self, child: &impl IsA<gtk::Widget>, position: i32) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_insert_pinned(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_prepend")]
    pub fn prepend(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_prepend(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_prepend_pinned")]
    pub fn prepend_pinned(&self, child: &impl IsA<gtk::Widget>) -> TabPage {
        unsafe {
            from_glib_none(ffi::adw_tab_view_prepend_pinned(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_tab_view_remove_shortcuts")]
    pub fn remove_shortcuts(&self, shortcuts: TabViewShortcuts) {
        unsafe {
            ffi::adw_tab_view_remove_shortcuts(self.to_glib_none().0, shortcuts.into_glib());
        }
    }

    #[doc(alias = "adw_tab_view_reorder_backward")]
    pub fn reorder_backward(&self, page: &TabPage) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_backward(
                self.to_glib_none().0,
                page.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_reorder_first")]
    pub fn reorder_first(&self, page: &TabPage) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_first(
                self.to_glib_none().0,
                page.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_reorder_forward")]
    pub fn reorder_forward(&self, page: &TabPage) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_forward(
                self.to_glib_none().0,
                page.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_reorder_last")]
    pub fn reorder_last(&self, page: &TabPage) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_last(
                self.to_glib_none().0,
                page.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_reorder_page")]
    pub fn reorder_page(&self, page: &TabPage, position: i32) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_reorder_page(
                self.to_glib_none().0,
                page.to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_select_next_page")]
    pub fn select_next_page(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_view_select_next_page(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_view_select_previous_page")]
    pub fn select_previous_page(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_view_select_previous_page(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_view_set_default_icon")]
    pub fn set_default_icon(&self, default_icon: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::adw_tab_view_set_default_icon(
                self.to_glib_none().0,
                default_icon.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_view_set_menu_model")]
    pub fn set_menu_model(&self, menu_model: Option<&impl IsA<gio::MenuModel>>) {
        unsafe {
            ffi::adw_tab_view_set_menu_model(
                self.to_glib_none().0,
                menu_model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_view_set_page_pinned")]
    pub fn set_page_pinned(&self, page: &TabPage, pinned: bool) {
        unsafe {
            ffi::adw_tab_view_set_page_pinned(
                self.to_glib_none().0,
                page.to_glib_none().0,
                pinned.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_view_set_selected_page")]
    pub fn set_selected_page(&self, selected_page: &TabPage) {
        unsafe {
            ffi::adw_tab_view_set_selected_page(
                self.to_glib_none().0,
                selected_page.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_tab_view_set_shortcuts")]
    pub fn set_shortcuts(&self, shortcuts: TabViewShortcuts) {
        unsafe {
            ffi::adw_tab_view_set_shortcuts(self.to_glib_none().0, shortcuts.into_glib());
        }
    }

    #[doc(alias = "adw_tab_view_transfer_page")]
    pub fn transfer_page(&self, page: &TabPage, other_view: &TabView, position: i32) {
        unsafe {
            ffi::adw_tab_view_transfer_page(
                self.to_glib_none().0,
                page.to_glib_none().0,
                other_view.to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "close-page")]
    pub fn connect_close_page<F: Fn(&Self, &TabPage) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn close_page_trampoline<F: Fn(&TabView, &TabPage) -> bool + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close-page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    close_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "create-window")]
    pub fn connect_create_window<F: Fn(&Self) -> Option<TabView> + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_window_trampoline<
            F: Fn(&TabView) -> Option<TabView> + 'static,
        >(
            this: *mut ffi::AdwTabView,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::AdwTabView {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)) /*Not checked*/
                .to_glib_none()
                .0
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-window\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    create_window_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "indicator-activated")]
    pub fn connect_indicator_activated<F: Fn(&Self, &TabPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn indicator_activated_trampoline<F: Fn(&TabView, &TabPage) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"indicator-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    indicator_activated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-attached")]
    pub fn connect_page_attached<F: Fn(&Self, &TabPage, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_attached_trampoline<F: Fn(&TabView, &TabPage, i32) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            position: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page), position)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-attached\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_attached_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-detached")]
    pub fn connect_page_detached<F: Fn(&Self, &TabPage, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_detached_trampoline<F: Fn(&TabView, &TabPage, i32) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            position: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page), position)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-detached\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_detached_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-reordered")]
    pub fn connect_page_reordered<F: Fn(&Self, &TabPage, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn page_reordered_trampoline<F: Fn(&TabView, &TabPage, i32) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            position: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(page), position)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-reordered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_reordered_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "setup-menu")]
    pub fn connect_setup_menu<F: Fn(&Self, Option<&TabPage>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn setup_menu_trampoline<F: Fn(&TabView, Option<&TabPage>) + 'static>(
            this: *mut ffi::AdwTabView,
            page: *mut ffi::AdwTabPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<TabPage>::from_glib_borrow(page).as_ref().as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"setup-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    setup_menu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "default-icon")]
    pub fn connect_default_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_icon_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::default-icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-transferring-page")]
    pub fn connect_is_transferring_page_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_transferring_page_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::is-transferring-page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_transferring_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menu-model")]
    pub fn connect_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_model_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::menu-model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_menu_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "n-pages")]
    pub fn connect_n_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_pages_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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

    #[doc(alias = "n-pinned-pages")]
    pub fn connect_n_pinned_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_pinned_pages_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::n-pinned-pages\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_pinned_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pages")]
    pub fn connect_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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

    #[doc(alias = "selected-page")]
    pub fn connect_selected_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_page_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::selected-page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_page_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "shortcuts")]
    pub fn connect_shortcuts_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shortcuts_trampoline<F: Fn(&TabView) + 'static>(
            this: *mut ffi::AdwTabView,
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
                b"notify::shortcuts\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shortcuts_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for TabView {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`TabView`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TabViewBuilder {
    default_icon: Option<gio::Icon>,
    menu_model: Option<gio::MenuModel>,
    selected_page: Option<TabPage>,
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    shortcuts: Option<TabViewShortcuts>,
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

impl TabViewBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`TabViewBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`TabView`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> TabView {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref default_icon) = self.default_icon {
            properties.push(("default-icon", default_icon));
        }
        if let Some(ref menu_model) = self.menu_model {
            properties.push(("menu-model", menu_model));
        }
        if let Some(ref selected_page) = self.selected_page {
            properties.push(("selected-page", selected_page));
        }
        #[cfg(any(feature = "v1_2", feature = "dox"))]
        if let Some(ref shortcuts) = self.shortcuts {
            properties.push(("shortcuts", shortcuts));
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
        glib::Object::new::<TabView>(&properties).expect("Failed to create an instance of TabView")
    }

    pub fn default_icon(mut self, default_icon: &impl IsA<gio::Icon>) -> Self {
        self.default_icon = Some(default_icon.clone().upcast());
        self
    }

    pub fn menu_model(mut self, menu_model: &impl IsA<gio::MenuModel>) -> Self {
        self.menu_model = Some(menu_model.clone().upcast());
        self
    }

    pub fn selected_page(mut self, selected_page: &TabPage) -> Self {
        self.selected_page = Some(selected_page.clone());
        self
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    pub fn shortcuts(mut self, shortcuts: TabViewShortcuts) -> Self {
        self.shortcuts = Some(shortcuts);
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

impl fmt::Display for TabView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TabView")
    }
}
