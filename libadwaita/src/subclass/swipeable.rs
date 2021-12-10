use crate::{NavigationDirection, Swipeable};
use glib::translate::*;
use glib::Cast;
use gtk::subclass::prelude::*;

pub trait SwipeableImpl: WidgetImpl {
    fn cancel_progress(&self, swipeable: &Self::Type) -> f64 {
        self.parent_cancel_progress(swipeable)
    }

    fn distance(&self, swipeable: &Self::Type) -> f64 {
        self.parent_distance(swipeable)
    }

    fn progress(&self, swipeable: &Self::Type) -> f64 {
        self.parent_progress(swipeable)
    }

    fn snap_points(&self, swipeable: &Self::Type) -> Vec<f64> {
        self.parent_snap_points(swipeable)
    }

    fn swipe_area(
        &self,
        swipeable: &Self::Type,
        navigation_direction: NavigationDirection,
        is_drag: bool,
    ) -> gdk::Rectangle {
        self.parent_swipe_area(swipeable, navigation_direction, is_drag)
    }
}

pub trait SwipeableImplExt: ObjectSubclass {
    fn parent_cancel_progress(&self, swipeable: &Self::Type) -> f64;
    fn parent_distance(&self, swipeable: &Self::Type) -> f64;
    fn parent_progress(&self, swipeable: &Self::Type) -> f64;
    fn parent_snap_points(&self, swipeable: &Self::Type) -> Vec<f64>;
    fn parent_swipe_area(
        &self,
        swipeable: &Self::Type,
        navigation_direction: NavigationDirection,
        is_drag: bool,
    ) -> gdk::Rectangle;
}

impl<T: SwipeableImpl> SwipeableImplExt for T {
    fn parent_cancel_progress(&self, swipeable: &Self::Type) -> f64 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Swipeable>()
                as *const ffi::AdwSwipeableInterface;

            let func = (*parent_iface)
                .get_cancel_progress
                .expect("no parent \"get_cancel_progress\" implementation");

            func(swipeable.unsafe_cast_ref::<Swipeable>().to_glib_none().0)
        }
    }

    fn parent_distance(&self, swipeable: &Self::Type) -> f64 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Swipeable>()
                as *const ffi::AdwSwipeableInterface;

            let func = (*parent_iface)
                .get_distance
                .expect("no parent \"get_distance\" implementation");

            func(swipeable.unsafe_cast_ref::<Swipeable>().to_glib_none().0)
        }
    }

    fn parent_progress(&self, swipeable: &Self::Type) -> f64 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Swipeable>()
                as *const ffi::AdwSwipeableInterface;

            let func = (*parent_iface)
                .get_progress
                .expect("no parent \"get_progress\" implementation");

            func(swipeable.unsafe_cast_ref::<Swipeable>().to_glib_none().0)
        }
    }

    fn parent_snap_points(&self, swipeable: &Self::Type) -> Vec<f64> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Swipeable>()
                as *const ffi::AdwSwipeableInterface;

            // gtk::Builder falls back to using ObjectExt::set_property if the method is not implemented
            let func = (*parent_iface)
                .get_snap_points
                .expect("no parent \"get_snap_points\" implementation");

            let mut n_points = std::mem::MaybeUninit::uninit();

            let points = func(
                swipeable.unsafe_cast_ref::<Swipeable>().to_glib_none().0,
                n_points.as_mut_ptr(),
            );

            FromGlibContainer::from_glib_none_num(points, n_points.assume_init() as usize)
        }
    }

    fn parent_swipe_area(
        &self,
        swipeable: &Self::Type,
        navigation_direction: NavigationDirection,
        is_drag: bool,
    ) -> gdk::Rectangle {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Swipeable>()
                as *const ffi::AdwSwipeableInterface;

            let func = (*parent_iface)
                .get_swipe_area
                .expect("no parent \"get_swipe_area\" implementation");

            let mut rect = gdk::Rectangle::uninitialized();
            func(
                swipeable.unsafe_cast_ref::<Swipeable>().to_glib_none().0,
                navigation_direction.into_glib(),
                is_drag.into_glib(),
                rect.to_glib_none_mut().0,
            );

            rect
        }
    }
}

unsafe impl<T: SwipeableImpl> IsImplementable<T> for Swipeable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_cancel_progress = Some(swipeable_get_cancel_progress::<T>);
        iface.get_distance = Some(swipeable_get_distance::<T>);
        iface.get_progress = Some(swipeable_get_progress::<T>);
        iface.get_snap_points = Some(swipeable_get_snap_points::<T>);
        iface.get_swipe_area = Some(swipeable_get_swipe_area::<T>);
    }
}

unsafe extern "C" fn swipeable_get_cancel_progress<T: SwipeableImpl>(
    swipeable: *mut ffi::AdwSwipeable,
) -> f64 {
    let instance = &*(swipeable as *mut T::Instance);
    let imp = instance.impl_();

    imp.cancel_progress(from_glib_borrow::<_, Swipeable>(swipeable).unsafe_cast_ref())
}

unsafe extern "C" fn swipeable_get_distance<T: SwipeableImpl>(
    swipeable: *mut ffi::AdwSwipeable,
) -> f64 {
    let instance = &*(swipeable as *mut T::Instance);
    let imp = instance.impl_();

    imp.distance(from_glib_borrow::<_, Swipeable>(swipeable).unsafe_cast_ref())
}

unsafe extern "C" fn swipeable_get_progress<T: SwipeableImpl>(
    swipeable: *mut ffi::AdwSwipeable,
) -> f64 {
    let instance = &*(swipeable as *mut T::Instance);
    let imp = instance.impl_();

    imp.progress(from_glib_borrow::<_, Swipeable>(swipeable).unsafe_cast_ref())
}

unsafe extern "C" fn swipeable_get_snap_points<T: SwipeableImpl>(
    swipeable: *mut ffi::AdwSwipeable,
    n_pointsptr: *mut libc::c_int,
) -> *mut f64 {
    let instance = &*(swipeable as *mut T::Instance);
    let imp = instance.impl_();

    let points = imp.snap_points(from_glib_borrow::<_, Swipeable>(swipeable).unsafe_cast_ref());

    n_pointsptr.write(points.len() as libc::c_int);
    ToGlibContainerFromSlice::to_glib_full_from_slice(&points)
}

unsafe extern "C" fn swipeable_get_swipe_area<T: SwipeableImpl>(
    swipeable: *mut ffi::AdwSwipeable,
    navigation_direction: ffi::AdwNavigationDirection,
    is_drag: i32,
    area: *mut gdk::ffi::GdkRectangle,
) {
    let instance = &*(swipeable as *mut T::Instance);
    let imp = instance.impl_();

    let swipe_area = imp.swipe_area(
        from_glib_borrow::<_, Swipeable>(swipeable).unsafe_cast_ref(),
        from_glib(navigation_direction),
        from_glib(is_drag),
    );

    *area = *swipe_area.to_glib_full();
}
