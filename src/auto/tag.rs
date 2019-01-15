// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use gobject_ffi;
use gtk;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct Tag(Object<ffi::GtkSourceTag, ffi::GtkSourceTagClass, TagClass>) @extends gtk::TextTag;

    match fn {
        get_type => || ffi::gtk_source_tag_get_type(),
    }
}

impl Tag {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P) -> Tag {
        assert_initialized_main_thread!();
        let name = name.into();
        unsafe {
            gtk::TextTag::from_glib_full(ffi::gtk_source_tag_new(name.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_TAG: Option<&Tag> = None;

pub trait TagExt: 'static {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_draw_spaces(&self) -> bool;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_draw_spaces(&self, draw_spaces: bool);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_draw_spaces_set(&self) -> bool;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_draw_spaces_set(&self, draw_spaces_set: bool);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_draw_spaces_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Tag>> TagExt for O {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_draw_spaces(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"draw-spaces\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_draw_spaces(&self, draw_spaces: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"draw-spaces\0".as_ptr() as *const _, Value::from(&draw_spaces).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_draw_spaces_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"draw-spaces-set\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_draw_spaces_set(&self, draw_spaces_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"draw-spaces-set\0".as_ptr() as *const _, Value::from(&draw_spaces_set).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::draw-spaces\0".as_ptr() as *const _,
                transmute(notify_draw_spaces_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_draw_spaces_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::draw-spaces-set\0".as_ptr() as *const _,
                transmute(notify_draw_spaces_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_draw_spaces_trampoline<P>(this: *mut ffi::GtkSourceTag, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Tag> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Tag::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_draw_spaces_set_trampoline<P>(this: *mut ffi::GtkSourceTag, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Tag> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Tag::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tag")
    }
}
