// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::AnimationTarget;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AdwCallbackAnimationTarget")]
    pub struct CallbackAnimationTarget(Object<ffi::AdwCallbackAnimationTarget, ffi::AdwCallbackAnimationTargetClass>) @extends AnimationTarget;

    match fn {
        type_ => || ffi::adw_callback_animation_target_get_type(),
    }
}

impl fmt::Display for CallbackAnimationTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CallbackAnimationTarget")
    }
}
