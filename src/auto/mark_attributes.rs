<<<<<<< HEAD
// This file was generated by gir (6bcd52a) from gir-files (1069259)
=======
// This file was generated by gir (d933f9a) from gir-files (469db10)
>>>>>>> origin/master
// DO NOT EDIT

use Mark;
use ffi;
use gdk;
use gdk_pixbuf;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct MarkAttributes(Object<ffi::GtkSourceMarkAttributes, ffi::GtkSourceMarkAttributesClass>);

    match fn {
        get_type => || ffi::gtk_source_mark_attributes_get_type(),
    }
}

impl MarkAttributes {
    pub fn new() -> MarkAttributes {
        unsafe {
            from_glib_full(ffi::gtk_source_mark_attributes_new())
        }
    }
}

impl Default for MarkAttributes {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MarkAttributesExt {
    fn get_background(&self) -> Option<gdk::RGBA>;

    //fn get_gicon(&self) -> /*Ignored*/Option<gio::Icon>;

    fn get_icon_name(&self) -> Option<String>;

    fn get_stock_id(&self) -> Option<String>;

    fn get_tooltip_markup(&self, mark: &Mark) -> Option<String>;

    fn get_tooltip_text(&self, mark: &Mark) -> Option<String>;

    fn set_background(&self, background: &gdk::RGBA);

    //fn set_gicon<P: IsA</*Ignored*/gio::Icon>>(&self, gicon: &P);

    fn set_icon_name(&self, icon_name: &str);

    fn set_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf);

    fn set_stock_id(&self, stock_id: &str);

    fn get_property_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn connect_query_tooltip_markup<F: Fn(&Self, &Mark) -> String + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_query_tooltip_text<F: Fn(&Self, &Mark) -> String + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

<<<<<<< HEAD
    fn connect_query_tooltip_text<F: Fn(&Self, &Mark) -> String + 'static>(&self, f: F) -> u64;

    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
=======
    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
>>>>>>> origin/master
}

impl<O: IsA<MarkAttributes> + IsA<glib::object::Object>> MarkAttributesExt for O {
    fn get_background(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut background = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_source_mark_attributes_get_background(self.to_glib_none().0, background.to_glib_none_mut().0));
            if ret { Some(background) } else { None }
        }
    }

    //fn get_gicon(&self) -> /*Ignored*/Option<gio::Icon> {
    //    unsafe { TODO: call ffi::gtk_source_mark_attributes_get_gicon() }
    //}

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_get_stock_id(self.to_glib_none().0))
        }
    }

    fn get_tooltip_markup(&self, mark: &Mark) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_mark_attributes_get_tooltip_markup(self.to_glib_none().0, mark.to_glib_none().0))
        }
    }

    fn get_tooltip_text(&self, mark: &Mark) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_mark_attributes_get_tooltip_text(self.to_glib_none().0, mark.to_glib_none().0))
        }
    }

    fn set_background(&self, background: &gdk::RGBA) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_background(self.to_glib_none().0, background.to_glib_none().0);
        }
    }

    //fn set_gicon<P: IsA</*Ignored*/gio::Icon>>(&self, gicon: &P) {
    //    unsafe { TODO: call ffi::gtk_source_mark_attributes_set_gicon() }
    //}

    fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    fn set_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    fn set_stock_id(&self, stock_id: &str) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_stock_id(self.to_glib_none().0, stock_id.to_glib_none().0);
        }
    }

    fn get_property_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        let mut value = Value::from(None::<&gdk_pixbuf::Pixbuf>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pixbuf".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_query_tooltip_markup<F: Fn(&Self, &Mark) -> String + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Mark) -> String + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "query-tooltip-markup",
                transmute(query_tooltip_markup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_query_tooltip_text<F: Fn(&Self, &Mark) -> String + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Mark) -> String + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "query-tooltip-text",
                transmute(query_tooltip_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

<<<<<<< HEAD
    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
=======
    fn connect_property_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
>>>>>>> origin/master
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::background",
                transmute(notify_background_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

<<<<<<< HEAD
    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
=======
    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
>>>>>>> origin/master
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::gicon",
                transmute(notify_gicon_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

<<<<<<< HEAD
    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
=======
    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
>>>>>>> origin/master
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-name",
                transmute(notify_icon_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

<<<<<<< HEAD
    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
=======
    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
>>>>>>> origin/master
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixbuf",
                transmute(notify_pixbuf_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

<<<<<<< HEAD
    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
=======
    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
>>>>>>> origin/master
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stock-id",
                transmute(notify_stock_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn query_tooltip_markup_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, mark: *mut ffi::GtkSourceMark, f: glib_ffi::gpointer) -> *mut libc::c_char
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P, &Mark) -> String + 'static) = transmute(f);
<<<<<<< HEAD
    f(&MarkAttributes::from_glib_none(this).downcast_unchecked(), &from_glib_none(mark)).to_glib_full()
=======
    f(&MarkAttributes::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(mark)).to_glib_full()
>>>>>>> origin/master
}

unsafe extern "C" fn query_tooltip_text_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, mark: *mut ffi::GtkSourceMark, f: glib_ffi::gpointer) -> *mut libc::c_char
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P, &Mark) -> String + 'static) = transmute(f);
<<<<<<< HEAD
    f(&MarkAttributes::from_glib_none(this).downcast_unchecked(), &from_glib_none(mark)).to_glib_full()
=======
    f(&MarkAttributes::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(mark)).to_glib_full()
}

unsafe extern "C" fn notify_background_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_gicon_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_name_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stock_id_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_borrow(this).downcast_unchecked())
>>>>>>> origin/master
}

unsafe extern "C" fn notify_background_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_gicon_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_name_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stock_id_trampoline<P>(this: *mut ffi::GtkSourceMarkAttributes, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MarkAttributes> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MarkAttributes::from_glib_none(this).downcast_unchecked())
}
