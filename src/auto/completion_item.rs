// This file was generated by gir (fdeaa47) from gir-files (2e2a9ca)
// DO NOT EDIT

use CompletionProposal;
use ffi;
use gdk_pixbuf;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct CompletionItem(Object<ffi::GtkSourceCompletionItem>): CompletionProposal;

    match fn {
        get_type => || ffi::gtk_source_completion_item_get_type(),
    }
}

impl CompletionItem {
    pub fn new<'a, 'b, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>, Q: Into<Option<&'b str>>>(label: &str, text: &str, icon: P, info: Q) -> CompletionItem {
        let icon = icon.into();
        let icon = icon.to_glib_none();
        let info = info.into();
        let info = info.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_completion_item_new(label.to_glib_none().0, text.to_glib_none().0, icon.0, info.0))
        }
    }

    pub fn new_from_stock<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(label: P, text: &str, stock: &str, info: Q) -> CompletionItem {
        let label = label.into();
        let label = label.to_glib_none();
        let info = info.into();
        let info = info.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_completion_item_new_from_stock(label.0, text.to_glib_none().0, stock.to_glib_none().0, info.0))
        }
    }

    pub fn new_with_markup<'a, 'b, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>, Q: Into<Option<&'b str>>>(markup: &str, text: &str, icon: P, info: Q) -> CompletionItem {
        let icon = icon.into();
        let icon = icon.to_glib_none();
        let info = info.into();
        let info = info.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_source_completion_item_new_with_markup(markup.to_glib_none().0, text.to_glib_none().0, icon.0, info.0))
        }
    }
}

pub trait CompletionItemExt {
    //#[cfg(feature = "v3_18")]
    //fn set_property_gicon<P: IsA</*Ignored*/gio::Icon> + IsA<glib::object::Object>>(&self, gicon: Option<&P>);

    fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>);

    #[cfg(feature = "v3_18")]
    fn set_property_icon_name(&self, icon_name: Option<&str>);

    fn set_property_info(&self, info: Option<&str>);

    fn set_property_label(&self, label: Option<&str>);

    fn set_property_markup(&self, markup: Option<&str>);

    fn set_property_text(&self, text: Option<&str>);
}

impl<O: IsA<CompletionItem> + IsA<glib::object::Object>> CompletionItemExt for O {
    //#[cfg(feature = "v3_18")]
    //fn set_property_gicon<P: IsA</*Ignored*/gio::Icon> + IsA<glib::object::Object>>(&self, gicon: Option<&P>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "gicon".to_glib_none().0, Value::from(gicon).to_glib_none().0);
    //    }
    //}

    fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon".to_glib_none().0, Value::from(icon).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_18")]
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
}
