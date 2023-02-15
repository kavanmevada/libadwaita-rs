// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::AdaptiveCondition;
use crate::AdaptiveConditionType;

impl AdaptiveCondition {
    #[doc(alias = "adw_adaptive_multi_condition_new")]
    #[doc(alias = "adw_adaptive_multi_condition_newv")]
    #[doc(alias = "multi_condition_new")]
    #[doc(alias = "multi_condition_newv")]
    pub fn new_multi(
        type_: AdaptiveConditionType,
        conditions: &[AdaptiveCondition],
    ) -> AdaptiveCondition {
        assert_initialized_main_thread!();
        unsafe {
            let mut conditions = conditions
                .iter()
                .map(|c| c.to_glib_full())
                .collect::<Vec<_>>();
            from_glib_full(ffi::adw_adaptive_multi_condition_newv(
                type_.into_glib(),
                conditions.as_mut_ptr(),
                conditions.len() as i32,
            ))
        }
    }
}
