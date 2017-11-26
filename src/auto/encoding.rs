<<<<<<< HEAD
// This file was generated by gir (6bcd52a) from gir-files (1069259)
=======
// This file was generated by gir (d933f9a) from gir-files (469db10)
>>>>>>> origin/master
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Encoding(Boxed<ffi::GtkSourceEncoding>);

    match fn {
        copy => |ptr| ffi::gtk_source_encoding_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_source_encoding_free(ptr),
        get_type => || ffi::gtk_source_encoding_get_type(),
    }
}

impl Encoding {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn get_charset(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_encoding_get_charset(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_encoding_get_name(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gtk_source_encoding_to_string(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn get_all() -> Vec<Encoding> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_source_encoding_get_all())
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn get_current() -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_encoding_get_current())
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    pub fn get_default_candidates() -> Vec<Encoding> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_source_encoding_get_default_candidates())
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn get_from_charset(charset: &str) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_encoding_get_from_charset(charset.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn get_utf8() -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_encoding_get_utf8())
        }
    }
}

impl fmt::Display for Encoding {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
