// This file was generated by gir (dc20488) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Spinner(Object<ffi::GtkSpinner>): Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_spinner_get_type(),
    }
}

impl Spinner {
    pub fn new() -> Spinner {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_spinner_new()).downcast_unchecked()
        }
    }

    pub fn start(&self) {
        unsafe {
            ffi::gtk_spinner_start(self.to_glib_none().0);
        }
    }

    pub fn stop(&self) {
        unsafe {
            ffi::gtk_spinner_stop(self.to_glib_none().0);
        }
    }
}
