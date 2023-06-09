// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "AdwAnimationTarget")]
    pub struct AnimationTarget(Object<ffi::AdwAnimationTarget, ffi::AdwAnimationTargetClass>);

    match fn {
        type_ => || ffi::adw_animation_target_get_type(),
    }
}

impl AnimationTarget {
    pub const NONE: Option<&'static AnimationTarget> = None;
}

impl fmt::Display for AnimationTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AnimationTarget")
    }
}
