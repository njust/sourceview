// This file was generated by gir (d121f7e) from gir-files (2e2a9ca)
// DO NOT EDIT

use GutterRenderer;
use ffi;
use gdk_pixbuf;
use gio;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct GutterRendererPixbuf(Object<ffi::GtkSourceGutterRendererPixbuf>): GutterRenderer;

    match fn {
        get_type => || ffi::gtk_source_gutter_renderer_pixbuf_get_type(),
    }
}

impl GutterRendererPixbuf {
    pub fn new() -> GutterRendererPixbuf {
        unsafe {
            GutterRenderer::from_glib_full(ffi::gtk_source_gutter_renderer_pixbuf_new()).downcast_unchecked()
        }
    }
}

pub trait GutterRendererPixbufExt {
    fn get_gicon(&self) -> Option<gio::Icon>;

    fn get_icon_name(&self) -> Option<String>;

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_stock_id(&self) -> Option<String>;

    fn set_gicon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon: Q);

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P);

    fn set_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P);

    fn set_stock_id<'a, P: Into<Option<&'a str>>>(&self, stock_id: P);
}

impl<O: IsA<GutterRendererPixbuf>> GutterRendererPixbufExt for O {
    fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_gicon(self.to_glib_none().0))
        }
    }

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_pixbuf(self.to_glib_none().0))
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_stock_id(self.to_glib_none().0))
        }
    }

    fn set_gicon<'a, P: IsA<gio::Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon: Q) {
        let icon = icon.into();
        let icon = icon.to_glib_none();
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_gicon(self.to_glib_none().0, icon.0);
        }
    }

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_icon_name(self.to_glib_none().0, icon_name.0);
        }
    }

    fn set_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P) {
        let pixbuf = pixbuf.into();
        let pixbuf = pixbuf.to_glib_none();
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_pixbuf(self.to_glib_none().0, pixbuf.0);
        }
    }

    fn set_stock_id<'a, P: Into<Option<&'a str>>>(&self, stock_id: P) {
        let stock_id = stock_id.into();
        let stock_id = stock_id.to_glib_none();
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_stock_id(self.to_glib_none().0, stock_id.0);
        }
    }
}
