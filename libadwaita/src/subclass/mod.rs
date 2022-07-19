pub mod action_row;
pub mod application;
pub mod application_window;
pub mod bin;
pub mod combo_row;
pub mod expander_row;
pub mod preferences_group;
pub mod preferences_page;
pub mod preferences_row;
pub mod preferences_window;
pub mod swipeable;
pub mod window;

pub mod prelude {
    pub use super::action_row::ActionRowImpl;
    pub use super::application::AdwApplicationImpl;
    pub use super::application_window::AdwApplicationWindowImpl;
    pub use super::bin::BinImpl;
    pub use super::combo_row::ComboRowImpl;
    pub use super::expander_row::ExpanderRowImpl;
    pub use super::preferences_group::PreferencesGroupImpl;
    pub use super::preferences_page::PreferencesPageImpl;
    pub use super::preferences_row::PreferencesRowImpl;
    pub use super::preferences_window::PreferencesWindowImpl;
    pub use super::swipeable::SwipeableImpl;
    pub use super::window::AdwWindowImpl;
    pub use gio::subclass::prelude::*;
    pub use glib::subclass::prelude::*;
    pub use gtk::subclass::prelude::*;
}
