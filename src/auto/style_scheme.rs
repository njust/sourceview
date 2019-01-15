// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Style;
use ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct StyleScheme(Object<ffi::GtkSourceStyleScheme, ffi::GtkSourceStyleSchemeClass, StyleSchemeClass>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_get_type(),
    }
}

pub const NONE_STYLE_SCHEME: Option<&StyleScheme> = None;

pub trait StyleSchemeExt: 'static {
    fn get_authors(&self) -> Vec<GString>;

    fn get_description(&self) -> Option<GString>;

    fn get_filename(&self) -> Option<GString>;

    fn get_id(&self) -> Option<GString>;

    fn get_name(&self) -> Option<GString>;

    fn get_style(&self, style_id: &str) -> Option<Style>;

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleScheme>> StyleSchemeExt for O {
    fn get_authors(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_style_scheme_get_authors(self.as_ref().to_glib_none().0))
        }
    }

    fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_description(self.as_ref().to_glib_none().0))
        }
    }

    fn get_filename(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_filename(self.as_ref().to_glib_none().0))
        }
    }

    fn get_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_id(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_style(&self, style_id: &str) -> Option<Style> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_style(self.as_ref().to_glib_none().0, style_id.to_glib_none().0))
        }
    }

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::description\0".as_ptr() as *const _,
                transmute(notify_description_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::filename\0".as_ptr() as *const _,
                transmute(notify_filename_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_description_trampoline<P>(this: *mut ffi::GtkSourceStyleScheme, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleScheme> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleScheme::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_filename_trampoline<P>(this: *mut ffi::GtkSourceStyleScheme, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleScheme> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleScheme::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::GtkSourceStyleScheme, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleScheme> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleScheme::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for StyleScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleScheme")
    }
}
