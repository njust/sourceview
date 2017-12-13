// This file was generated by gir (13f739b) from gir-files (469db10)
// DO NOT EDIT

use ffi;
use glib;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SearchSettings(Object<ffi::GtkSourceSearchSettings, ffi::GtkSourceSearchSettingsClass>);

    match fn {
        get_type => || ffi::gtk_source_search_settings_get_type(),
    }
}

impl SearchSettings {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new() -> SearchSettings {
        unsafe {
            from_glib_full(ffi::gtk_source_search_settings_new())
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
impl Default for SearchSettings {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SearchSettingsExt {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_at_word_boundaries(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_case_sensitive(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_regex_enabled(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_search_text(&self) -> Option<String>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_wrap_around(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_at_word_boundaries(&self, at_word_boundaries: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_case_sensitive(&self, case_sensitive: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_regex_enabled(&self, regex_enabled: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_search_text<'a, P: Into<Option<&'a str>>>(&self, search_text: P);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_wrap_around(&self, wrap_around: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_at_word_boundaries_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_case_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_regex_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_search_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_wrap_around_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SearchSettings> + IsA<glib::object::Object>> SearchSettingsExt for O {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_at_word_boundaries(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_at_word_boundaries(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_case_sensitive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_case_sensitive(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_regex_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_regex_enabled(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_search_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_search_settings_get_search_text(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_wrap_around(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_wrap_around(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_at_word_boundaries(&self, at_word_boundaries: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_at_word_boundaries(self.to_glib_none().0, at_word_boundaries.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_case_sensitive(&self, case_sensitive: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_case_sensitive(self.to_glib_none().0, case_sensitive.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_regex_enabled(&self, regex_enabled: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_regex_enabled(self.to_glib_none().0, regex_enabled.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_search_text<'a, P: Into<Option<&'a str>>>(&self, search_text: P) {
        let search_text = search_text.into();
        let search_text = search_text.to_glib_none();
        unsafe {
            ffi::gtk_source_search_settings_set_search_text(self.to_glib_none().0, search_text.0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_wrap_around(&self, wrap_around: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_wrap_around(self.to_glib_none().0, wrap_around.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_at_word_boundaries_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::at-word-boundaries",
                transmute(notify_at_word_boundaries_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_case_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::case-sensitive",
                transmute(notify_case_sensitive_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_regex_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::regex-enabled",
                transmute(notify_regex_enabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_search_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::search-text",
                transmute(notify_search_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_wrap_around_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::wrap-around",
                transmute(notify_wrap_around_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_at_word_boundaries_trampoline<P>(this: *mut ffi::GtkSourceSearchSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchSettings> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchSettings::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_case_sensitive_trampoline<P>(this: *mut ffi::GtkSourceSearchSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchSettings> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchSettings::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_regex_enabled_trampoline<P>(this: *mut ffi::GtkSourceSearchSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchSettings> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchSettings::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_search_text_trampoline<P>(this: *mut ffi::GtkSourceSearchSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchSettings> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchSettings::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn notify_wrap_around_trampoline<P>(this: *mut ffi::GtkSourceSearchSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SearchSettings> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SearchSettings::from_glib_borrow(this).downcast_unchecked())
}
