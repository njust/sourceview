// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use CompletionProposal;
use ffi;
use gdk_pixbuf;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use gio;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CompletionItem(Object<ffi::GtkSourceCompletionItem, ffi::GtkSourceCompletionItemClass>): CompletionProposal;

    match fn {
        get_type => || ffi::gtk_source_completion_item_get_type(),
    }
}

impl CompletionItem {
    #[cfg_attr(feature = "v3_24", deprecated)]
    pub fn new<'a, 'b, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>, Q: Into<Option<&'b str>>>(label: &str, text: &str, icon: P, info: Q) -> CompletionItem {
        let icon = icon.into();
        let icon = icon.to_glib_none();
        let info = info.into();
        let info = info.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_completion_item_new(label.to_glib_none().0, text.to_glib_none().0, icon.0, info.0))
        }
    }

    #[cfg_attr(feature = "v3_10", deprecated)]
    pub fn new_from_stock<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(label: P, text: &str, stock: &str, info: Q) -> CompletionItem {
        let label = label.into();
        let label = label.to_glib_none();
        let info = info.into();
        let info = info.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_completion_item_new_from_stock(label.0, text.to_glib_none().0, stock.to_glib_none().0, info.0))
        }
    }

    #[cfg_attr(feature = "v3_24", deprecated)]
    pub fn new_with_markup<'a, 'b, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>, Q: Into<Option<&'b str>>>(markup: &str, text: &str, icon: P, info: Q) -> CompletionItem {
        let icon = icon.into();
        let icon = icon.to_glib_none();
        let info = info.into();
        let info = info.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_completion_item_new_with_markup(markup.to_glib_none().0, text.to_glib_none().0, icon.0, info.0))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn new2() -> Option<CompletionItem> {
        unsafe {
            from_glib_full(ffi::gtk_source_completion_item_new2())
        }
    }
}

pub trait CompletionItemExt {
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_gicon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, gicon: Q);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_icon<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, icon: P);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_info<'a, P: Into<Option<&'a str>>>(&self, info: P);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_markup<'a, P: Into<Option<&'a str>>>(&self, markup: P);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_text<'a, P: Into<Option<&'a str>>>(&self, text: P);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_property_gicon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, gicon: Option<&P>);

    fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_property_icon_name(&self, icon_name: Option<&str>);

    fn set_property_info(&self, info: Option<&str>);

    fn set_property_label(&self, label: Option<&str>);

    fn set_property_markup(&self, markup: Option<&str>);

    fn set_property_text(&self, text: Option<&str>);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionItem> + IsA<glib::object::Object>> CompletionItemExt for O {
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_gicon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, gicon: Q) {
        let gicon = gicon.into();
        let gicon = gicon.to_glib_none();
        unsafe {
            ffi::gtk_source_completion_item_set_gicon(self.to_glib_none().0, gicon.0);
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_icon<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, icon: P) {
        let icon = icon.into();
        let icon = icon.to_glib_none();
        unsafe {
            ffi::gtk_source_completion_item_set_icon(self.to_glib_none().0, icon.0);
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gtk_source_completion_item_set_icon_name(self.to_glib_none().0, icon_name.0);
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_info<'a, P: Into<Option<&'a str>>>(&self, info: P) {
        let info = info.into();
        let info = info.to_glib_none();
        unsafe {
            ffi::gtk_source_completion_item_set_info(self.to_glib_none().0, info.0);
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::gtk_source_completion_item_set_label(self.to_glib_none().0, label.0);
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_markup<'a, P: Into<Option<&'a str>>>(&self, markup: P) {
        let markup = markup.into();
        let markup = markup.to_glib_none();
        unsafe {
            ffi::gtk_source_completion_item_set_markup(self.to_glib_none().0, markup.0);
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    fn set_text<'a, P: Into<Option<&'a str>>>(&self, text: P) {
        let text = text.into();
        let text = text.to_glib_none();
        unsafe {
            ffi::gtk_source_completion_item_set_text(self.to_glib_none().0, text.0);
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_property_gicon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, gicon: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "gicon".to_glib_none().0, Value::from(gicon).to_glib_none().0);
        }
    }

    fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon".to_glib_none().0, Value::from(icon).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_property_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-name".to_glib_none().0, Value::from(icon_name).to_glib_none().0);
        }
    }

    fn set_property_info(&self, info: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "info".to_glib_none().0, Value::from(info).to_glib_none().0);
        }
    }

    fn set_property_label(&self, label: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "label".to_glib_none().0, Value::from(label).to_glib_none().0);
        }
    }

    fn set_property_markup(&self, markup: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "markup".to_glib_none().0, Value::from(markup).to_glib_none().0);
        }
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::gicon",
                transmute(notify_gicon_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon",
                transmute(notify_icon_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-name",
                transmute(notify_icon_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::info",
                transmute(notify_info_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label",
                transmute(notify_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::markup",
                transmute(notify_markup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
unsafe extern "C" fn notify_gicon_trampoline<P>(this: *mut ffi::GtkSourceCompletionItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_trampoline<P>(this: *mut ffi::GtkSourceCompletionItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionItem::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
unsafe extern "C" fn notify_icon_name_trampoline<P>(this: *mut ffi::GtkSourceCompletionItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_info_trampoline<P>(this: *mut ffi::GtkSourceCompletionItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_trampoline<P>(this: *mut ffi::GtkSourceCompletionItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_markup_trampoline<P>(this: *mut ffi::GtkSourceCompletionItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::GtkSourceCompletionItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionItem::from_glib_borrow(this).downcast_unchecked())
}
