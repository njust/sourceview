<<<<<<< HEAD
// This file was generated by gir (6bcd52a) from gir-files (1069259)
=======
// This file was generated by gir (d933f9a) from gir-files (469db10)
>>>>>>> origin/master
// DO NOT EDIT

use ffi;
use glib;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::Value;
<<<<<<< HEAD
#[cfg(feature = "v3_20")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_20")]
=======
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_20", feature = "dox"))]
>>>>>>> origin/master
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
<<<<<<< HEAD
#[cfg(feature = "v3_20")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v3_20")]
=======
use gtk;
use gtk_ffi;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_20", feature = "dox"))]
>>>>>>> origin/master
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Tag(Object<ffi::GtkSourceTag, ffi::GtkSourceTagClass>): [
        gtk::TextTag => gtk_ffi::GtkTextTag,
    ];

    match fn {
        get_type => || ffi::gtk_source_tag_get_type(),
    }
}

impl Tag {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P) -> Tag {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            gtk::TextTag::from_glib_full(ffi::gtk_source_tag_new(name.0)).downcast_unchecked()
        }
    }
}

pub trait TagExt {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_draw_spaces(&self) -> bool;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_draw_spaces(&self, draw_spaces: bool);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_draw_spaces_set(&self) -> bool;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_draw_spaces_set(&self, draw_spaces_set: bool);

<<<<<<< HEAD
    #[cfg(feature = "v3_20")]
    fn connect_property_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_20")]
    fn connect_property_draw_spaces_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
=======
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_draw_spaces_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
>>>>>>> origin/master
}

impl<O: IsA<Tag> + IsA<glib::object::Object>> TagExt for O {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_draw_spaces(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "draw-spaces".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_draw_spaces(&self, draw_spaces: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "draw-spaces".to_glib_none().0, Value::from(&draw_spaces).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_draw_spaces_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "draw-spaces-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_draw_spaces_set(&self, draw_spaces_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "draw-spaces-set".to_glib_none().0, Value::from(&draw_spaces_set).to_glib_none().0);
        }
    }

<<<<<<< HEAD
    #[cfg(feature = "v3_20")]
    fn connect_property_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
=======
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_draw_spaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
>>>>>>> origin/master
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::draw-spaces",
                transmute(notify_draw_spaces_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

<<<<<<< HEAD
    #[cfg(feature = "v3_20")]
    fn connect_property_draw_spaces_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
=======
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_draw_spaces_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
>>>>>>> origin/master
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::draw-spaces-set",
                transmute(notify_draw_spaces_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

<<<<<<< HEAD
#[cfg(feature = "v3_20")]
=======
#[cfg(any(feature = "v3_20", feature = "dox"))]
>>>>>>> origin/master
unsafe extern "C" fn notify_draw_spaces_trampoline<P>(this: *mut ffi::GtkSourceTag, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Tag> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
<<<<<<< HEAD
    f(&Tag::from_glib_none(this).downcast_unchecked())
}

#[cfg(feature = "v3_20")]
=======
    f(&Tag::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
>>>>>>> origin/master
unsafe extern "C" fn notify_draw_spaces_set_trampoline<P>(this: *mut ffi::GtkSourceTag, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Tag> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
<<<<<<< HEAD
    f(&Tag::from_glib_none(this).downcast_unchecked())
=======
    f(&Tag::from_glib_borrow(this).downcast_unchecked())
>>>>>>> origin/master
}
