// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct UndoManager(Interface<gtk_source_sys::GtkSourceUndoManager>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_undo_manager_get_type(),
    }
}

pub const NONE_UNDO_MANAGER: Option<&UndoManager> = None;

pub trait UndoManagerExt: 'static {
    fn begin_not_undoable_action(&self);

    fn can_redo(&self) -> bool;

    fn can_redo_changed(&self);

    fn can_undo(&self) -> bool;

    fn can_undo_changed(&self);

    fn end_not_undoable_action(&self);

    fn redo(&self);

    fn undo(&self);

    fn connect_can_redo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_can_redo_changed(&self);

    fn connect_can_undo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_can_undo_changed(&self);
}

impl<O: IsA<UndoManager>> UndoManagerExt for O {
    fn begin_not_undoable_action(&self) {
        unsafe {
            gtk_source_sys::gtk_source_undo_manager_begin_not_undoable_action(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    fn can_redo(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_undo_manager_can_redo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_redo_changed(&self) {
        unsafe {
            gtk_source_sys::gtk_source_undo_manager_can_redo_changed(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    fn can_undo(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_undo_manager_can_undo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_undo_changed(&self) {
        unsafe {
            gtk_source_sys::gtk_source_undo_manager_can_undo_changed(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    fn end_not_undoable_action(&self) {
        unsafe {
            gtk_source_sys::gtk_source_undo_manager_end_not_undoable_action(
                self.as_ref().to_glib_none().0,
            );
        }
    }

    fn redo(&self) {
        unsafe {
            gtk_source_sys::gtk_source_undo_manager_redo(self.as_ref().to_glib_none().0);
        }
    }

    fn undo(&self) {
        unsafe {
            gtk_source_sys::gtk_source_undo_manager_undo(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_can_redo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn can_redo_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceUndoManager,
            f: glib_sys::gpointer,
        ) where
            P: IsA<UndoManager>,
        {
            let f: &F = &*(f as *const F);
            f(&UndoManager::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"can-redo-changed\0".as_ptr() as *const _,
                Some(transmute(can_redo_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_can_redo_changed(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("can-redo-changed", &[])
                .unwrap()
        };
    }

    fn connect_can_undo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn can_undo_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceUndoManager,
            f: glib_sys::gpointer,
        ) where
            P: IsA<UndoManager>,
        {
            let f: &F = &*(f as *const F);
            f(&UndoManager::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"can-undo-changed\0".as_ptr() as *const _,
                Some(transmute(can_undo_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_can_undo_changed(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("can-undo-changed", &[])
                .unwrap()
        };
    }
}

impl fmt::Display for UndoManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UndoManager")
    }
}
