// This file was generated by gir (e6cb5d0) from gir-files (11e0e6d)
// DO NOT EDIT

use CellRenderer;
use CellRendererText;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CellRendererSpin(Object<ffi::GtkCellRendererSpin>): CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_spin_get_type(),
    }
}

impl CellRendererSpin {
    pub fn new() -> CellRendererSpin {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_spin_new()).downcast_unchecked()
        }
    }
}
