// This file was generated by gir (83d5a2f) from gir-files (db49619)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Language(Object<ffi::GtkSourceLanguage>);

    match fn {
        get_type => || ffi::gtk_source_language_get_type(),
    }
}

pub trait LanguageExt {
    fn get_globs(&self) -> Vec<String>;

    fn get_hidden(&self) -> bool;

    fn get_id(&self) -> Option<String>;

    fn get_metadata(&self, name: &str) -> Option<String>;

    fn get_mime_types(&self) -> Vec<String>;

    fn get_name(&self) -> Option<String>;

    fn get_section(&self) -> Option<String>;

    #[cfg(feature = "v3_4")]
    fn get_style_fallback(&self, style_id: &str) -> Option<String>;

    fn get_style_ids(&self) -> Vec<String>;

    fn get_style_name(&self, style_id: &str) -> Option<String>;

    fn connect_property_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_section_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Language> + IsA<glib::object::Object>> LanguageExt for O {
    fn get_globs(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_source_language_get_globs(self.to_glib_none().0))
        }
    }

    fn get_hidden(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_language_get_hidden(self.to_glib_none().0))
        }
    }

    fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_get_id(self.to_glib_none().0))
        }
    }

    fn get_metadata(&self, name: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_get_metadata(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_mime_types(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_source_language_get_mime_types(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_get_name(self.to_glib_none().0))
        }
    }

    fn get_section(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_get_section(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_4")]
    fn get_style_fallback(&self, style_id: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_get_style_fallback(self.to_glib_none().0, style_id.to_glib_none().0))
        }
    }

    fn get_style_ids(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_source_language_get_style_ids(self.to_glib_none().0))
        }
    }

    fn get_style_name(&self, style_id: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_get_style_name(self.to_glib_none().0, style_id.to_glib_none().0))
        }
    }

    fn connect_property_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hidden",
                transmute(notify_hidden_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::id",
                transmute(notify_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_section_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::section",
                transmute(notify_section_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_hidden_trampoline<P>(this: *mut ffi::GtkSourceLanguage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Language> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Language::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_id_trampoline<P>(this: *mut ffi::GtkSourceLanguage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Language> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Language::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::GtkSourceLanguage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Language> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Language::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_section_trampoline<P>(this: *mut ffi::GtkSourceLanguage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Language> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Language::from_glib_none(this).downcast_unchecked())
}
