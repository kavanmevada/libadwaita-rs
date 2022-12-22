use glib::object::IsA;
use glib::translate::*;

use crate::prelude::*;
use crate::MessageDialog;

pub trait MessageDialogExtManual: 'static {
    #[doc(alias = "adw_message_dialog_get_response_label")]
    #[doc(alias = "get_response_label")]
    fn response_label(&self, response: &str) -> glib::GString;

    #[doc(alias = "adw_message_dialog_add_responses")]
    fn add_responses(&self, ids_and_labels: &[(&str, &str)]);
}

impl<O: IsA<MessageDialog>> MessageDialogExtManual for O {
    #[doc(alias = "adw_message_dialog_get_response_label")]
    #[doc(alias = "get_response_label")]
    fn response_label(&self, response: &str) -> glib::GString {
        assert!(self.as_ref().has_response(response));

        unsafe {
            from_glib_none(ffi::adw_message_dialog_get_response_label(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_message_dialog_add_responses")]
    fn add_responses(&self, ids_and_labels: &[(&str, &str)]) {
        ids_and_labels.iter().for_each(|(id, label)| {
            self.add_response(id, label);
        });
    }
}
