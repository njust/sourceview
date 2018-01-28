// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v3_14", feature = "dox"))]
use Buffer;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use CompressionType;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Encoding;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use File;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use NewlineType;
use ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use gio;
use glib;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FileLoader(Object<ffi::GtkSourceFileLoader, ffi::GtkSourceFileLoaderClass>);

    match fn {
        get_type => || ffi::gtk_source_file_loader_get_type(),
    }
}

impl FileLoader {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new(buffer: &Buffer, file: &File) -> FileLoader {
        unsafe {
            from_glib_full(ffi::gtk_source_file_loader_new(buffer.to_glib_none().0, file.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //pub fn new_from_stream<P: IsA</*Ignored*/gio::InputStream>>(buffer: &Buffer, file: &File, stream: &P) -> FileLoader {
    //    unsafe { TODO: call ffi::gtk_source_file_loader_new_from_stream() }
    //}
}

pub trait FileLoaderExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_file(&self) -> Option<File>;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_input_stream(&self) -> /*Ignored*/Option<gio::InputStream>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType;

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn load_async<'a, 'b, 'c, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S, callback: T);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_candidate_encodings(&self, candidate_encodings: &[&Encoding]);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileLoader> + IsA<glib::object::Object>> FileLoaderExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::gtk_source_file_loader_get_compression_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_encoding(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_file(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_file(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn get_input_stream(&self) -> /*Ignored*/Option<gio::InputStream> {
    //    unsafe { TODO: call ffi::gtk_source_file_loader_get_input_stream() }
    //}

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_loader_get_location(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::gtk_source_file_loader_get_newline_type(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn load_async<'a, 'b, 'c, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S, callback: T) {
    //    unsafe { TODO: call ffi::gtk_source_file_loader_load_async() }
    //}

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_candidate_encodings(&self, candidate_encodings: &[&Encoding]) {
        unsafe {
            ffi::gtk_source_file_loader_set_candidate_encodings(self.to_glib_none().0, candidate_encodings.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer",
                transmute(notify_buffer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::file",
                transmute(notify_file_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::input-stream",
                transmute(notify_input_stream_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::location",
                transmute(notify_location_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_buffer_trampoline<P>(this: *mut ffi::GtkSourceFileLoader, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileLoader> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileLoader::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_file_trampoline<P>(this: *mut ffi::GtkSourceFileLoader, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileLoader> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileLoader::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_input_stream_trampoline<P>(this: *mut ffi::GtkSourceFileLoader, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileLoader> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileLoader::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_location_trampoline<P>(this: *mut ffi::GtkSourceFileLoader, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileLoader> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileLoader::from_glib_borrow(this).downcast_unchecked())
}
