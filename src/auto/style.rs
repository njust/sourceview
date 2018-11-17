// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use pango;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Style(Object<ffi::GtkSourceStyle, ffi::GtkSourceStyleClass>);

    match fn {
        get_type => || ffi::gtk_source_style_get_type(),
    }
}

pub trait StyleExt {
    fn get_property_background(&self) -> Option<String>;

    fn get_property_background_set(&self) -> bool;

    fn get_property_bold(&self) -> bool;

    fn get_property_bold_set(&self) -> bool;

    fn get_property_foreground(&self) -> Option<String>;

    fn get_property_foreground_set(&self) -> bool;

    fn get_property_italic(&self) -> bool;

    fn get_property_italic_set(&self) -> bool;

    fn get_property_line_background(&self) -> Option<String>;

    fn get_property_line_background_set(&self) -> bool;

    fn get_property_pango_underline(&self) -> pango::Underline;

    fn get_property_scale(&self) -> Option<String>;

    fn get_property_scale_set(&self) -> bool;

    fn get_property_strikethrough(&self) -> bool;

    fn get_property_strikethrough_set(&self) -> bool;

    #[cfg_attr(feature = "v3_18", deprecated)]
    fn get_property_underline(&self) -> bool;

    fn get_property_underline_color(&self) -> Option<String>;

    fn get_property_underline_color_set(&self) -> bool;

    fn get_property_underline_set(&self) -> bool;
}

impl<O: IsA<Style> + IsA<glib::object::Object>> StyleExt for O {
    fn get_property_background(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_background_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "background-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_bold(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "bold".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_bold_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "bold-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_foreground(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "foreground".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_foreground_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "foreground-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_italic(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "italic".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_italic_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "italic-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_line_background(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "line-background".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_line_background_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "line-background-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_pango_underline(&self) -> pango::Underline {
        unsafe {
            let mut value = Value::from_type(<pango::Underline as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pango-underline".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_scale(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_scale_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scale-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_strikethrough(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "strikethrough".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_strikethrough_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "strikethrough-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_underline(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_underline_color(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-color".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_underline_color_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-color-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_underline_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "underline-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Style")
    }
}
