// This file was generated by gir (127a851) from gir-files (0bcaef9)
// DO NOT EDIT

use CellRenderer;
use CellRendererText;
use TreeIter;
use TreeModel;
use TreePath;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
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
    pub struct CellRendererCombo(Object<ffi::GtkCellRendererCombo>): CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_combo_get_type(),
    }
}

impl CellRendererCombo {
    pub fn new() -> CellRendererCombo {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_combo_new()).downcast_unchecked()
        }
    }
}

pub trait CellRendererComboExt {
    fn get_property_has_entry(&self) -> bool;

    fn set_property_has_entry(&self, has_entry: bool);

    fn get_property_model(&self) -> Option<TreeModel>;

    fn set_property_model<P: IsA<TreeModel> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, model: Option<&P>);

    fn get_property_text_column(&self) -> i32;

    fn set_property_text_column(&self, text_column: i32);

    fn connect_changed<F: Fn(&Self, TreePath, &TreeIter) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<CellRendererCombo> + IsA<glib::object::Object>> CellRendererComboExt for O {
    fn get_property_has_entry(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "has-entry".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_has_entry(&self, has_entry: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "has-entry".to_glib_none().0, Value::from(&has_entry).to_glib_none().0);
        }
    }

    fn get_property_model(&self) -> Option<TreeModel> {
        let mut value = Value::from(None::<&TreeModel>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "model".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_model<P: IsA<TreeModel> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, model: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "model".to_glib_none().0, Value::from(model).to_glib_none().0);
        }
    }

    fn get_property_text_column(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text-column".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_text_column(&self, text_column: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text-column".to_glib_none().0, Value::from(&text_column).to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self, TreePath, &TreeIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, TreePath, &TreeIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkCellRendererCombo, path_string: *mut libc::c_char, new_iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer)
where P: IsA<CellRendererCombo> {
    callback_guard!();
    let f: &Box_<Fn(&P, TreePath, &TreeIter) + 'static> = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path_string));
    f(&CellRendererCombo::from_glib_none(this).downcast_unchecked(), path, &from_glib_borrow(new_iter))
}
