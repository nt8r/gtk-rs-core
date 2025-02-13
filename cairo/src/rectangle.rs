// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "use_glib")]
use glib::translate::*;
use std::fmt;
#[cfg(feature = "use_glib")]
use std::mem;

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
#[doc(alias = "cairo_rectangle_t")]
pub struct Rectangle(ffi::cairo_rectangle_t);

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self(ffi::cairo_rectangle_t {
            x,
            y,
            width,
            height,
        })
    }
    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn set_x(&mut self, x: f64) {
        self.0.x = x;
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
    pub fn set_y(&mut self, y: f64) {
        self.0.y = y;
    }
    pub fn width(&self) -> f64 {
        self.0.width
    }
    pub fn set_width(&mut self, width: f64) {
        self.0.width = width;
    }
    pub fn height(&self) -> f64 {
        self.0.height
    }
    pub fn set_height(&mut self, height: f64) {
        self.0.height = height;
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rectangle")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl Uninitialized for Rectangle {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::cairo_rectangle_t> for Rectangle {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::cairo_rectangle_t, Self> {
        let ptr: *const Rectangle = self;
        Stash(ptr as *const ffi::cairo_rectangle_t, self)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::cairo_rectangle_t> for Rectangle {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::cairo_rectangle_t, Self> {
        let ptr: *mut Rectangle = &mut *self;
        StashMut(ptr as *mut ffi::cairo_rectangle_t, self)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::cairo_rectangle_t> for Rectangle {
    unsafe fn from_glib_none(ptr: *const ffi::cairo_rectangle_t) -> Self {
        *(ptr as *const Rectangle)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::cairo_rectangle_t> for Rectangle {
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_rectangle_t) -> crate::Borrowed<Self> {
        crate::Borrowed::new(*(ptr as *mut Rectangle))
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::cairo_rectangle_t> for Rectangle {
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_rectangle_t) -> Self {
        *(ptr as *mut Rectangle)
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl_inline!(
    Rectangle,
    ffi::cairo_rectangle_t,
    ffi::gobject::cairo_gobject_rectangle_get_type
);

impl Rectangle {
    pub fn to_raw_none(&self) -> *mut ffi::cairo_rectangle_t {
        let ptr = self as *const Rectangle as usize;
        ptr as *mut ffi::cairo_rectangle_t
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "use_glib")]
    #[test]
    fn rectangle_gvalues() {
        use glib::ToValue;
        let rect = Rectangle::new(1., 2., 3., 4.);
        let value = rect.to_value();
        assert_eq!(value.get::<Rectangle>().unwrap().width(), 3.);
        let _ = rect.to_value();
        let rect = Some(rect);
        let value = rect.to_value();
        assert_eq!(
            value.get::<Option<Rectangle>>().unwrap().map(|s| s.width()),
            Some(3.)
        );
        let _ = rect.as_ref().to_value();
        assert_eq!(
            value
                .get::<Option<&Rectangle>>()
                .unwrap()
                .map(|s| s.width()),
            Some(3.)
        );
    }
}
