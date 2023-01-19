use std::ops::Index;
use std::slice::SliceIndex;

mod ffi {
    use super::{ContourPointer, ContoursPointer, Point};

    #[link(name = "rxcv", kind = "static")]
    extern "C" {
        pub(super) fn cv_new_contours() -> *const ContoursPointer;
        pub(super) fn cv_contours_size(contours: *const ContoursPointer) -> i32;
        pub(super) fn cv_release_contours(contours: *const ContoursPointer);
        pub(super) fn cv_contours_at(
            contours: *const ContoursPointer,
            index: i32,
        ) -> *const ContourPointer;
        pub(super) fn cv_new_contour() -> *const ContourPointer;
        pub(super) fn cv_release_contour(contour: *const ContourPointer);
        pub(super) fn cv_contour_size(contour: *const ContourPointer) -> i32;
        pub(super) fn cv_contour_at(contour: *const ContourPointer, index: i32) -> Point;
        pub(super) fn cv_contour_area(contour: *const ContourPointer) -> f64;
        pub(super) fn cv_contour_arc_length(contour: *const ContourPointer, closed: bool) -> f64;
    }
}

#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub(crate) enum ContourPointer {}
pub(crate) enum ContoursPointer {}

pub struct Contour<const UM: bool> {
    pointer: *const ContourPointer,
}

pub struct Contours {
    pub(crate) pointer: *const ContoursPointer,
    pub(crate) inner: Vec<Contour<false>>,
}

pub struct ContourIter<'a, const UM: bool> {
    inner: &'a Contour<UM>,
    index: i32,
    size: i32,
}

impl Default for Contour<true> {
    fn default() -> Self {
        Self {
            pointer: unsafe { ffi::cv_new_contour() },
        }
    }
}

impl Contour<false> {
    fn from_ref(pointer: *const ContourPointer) -> Self {
        Self { pointer }
    }
}

impl<const UM: bool> Drop for Contour<UM> {
    fn drop(&mut self) {
        if UM {
            unsafe { ffi::cv_release_contour(self.pointer) }
        }
    }
}

impl<const UM: bool> Contour<UM> {
    pub fn size(&self) -> i32 {
        unsafe { ffi::cv_contour_size(self.pointer) }
    }

    pub fn iter(&self) -> ContourIter<UM> {
        ContourIter {
            inner: self,
            index: 0,
            size: self.size(),
        }
    }

    pub fn area(&self) -> f64 {
        unsafe { ffi::cv_contour_area(self.pointer) }
    }

    pub fn arc_length(&self, closed: bool) -> f64 {
        unsafe { ffi::cv_contour_arc_length(self.pointer, closed) }
    }

    pub fn closed_arc_length(&self) -> f64 {
        unsafe { ffi::cv_contour_arc_length(self.pointer, true) }
    }
}

impl<const UM: bool> Iterator for ContourIter<'_, UM> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size <= self.index {
            None
        } else {
            let point = unsafe { ffi::cv_contour_at(self.inner.pointer, self.index) };
            self.index += 1;
            Some(point)
        }
    }
}

impl Contours {
    pub(crate) fn new(pointer: *const ContoursPointer) -> Self {
        Self {
            pointer,
            inner: vec![],
        }
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn at(&self, index: i32) -> Contour<false> {
        Contour::from_ref(unsafe { ffi::cv_contours_at(self.pointer, index) })
    }

    pub(crate) fn get_inner(&self) -> Vec<Contour<false>> {
        let size = unsafe { ffi::cv_contours_size(self.pointer) };
        (0..size).map(|i| self.at(i)).collect()
    }
}

impl<T> Index<T> for Contours
where
    T: SliceIndex<[Contour<false>]>,
{
    type Output = T::Output;

    fn index(&self, index: T) -> &Self::Output {
        &self.inner[index]
    }
}

impl Default for Contours {
    fn default() -> Self {
        Self::new(unsafe { ffi::cv_new_contours() })
    }
}

impl Drop for Contours {
    fn drop(&mut self) {
        unsafe { ffi::cv_release_contours(self.pointer) }
    }
}
