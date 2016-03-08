// This file was generated by gir (dc20488) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use RecentFilterFlags;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct RecentFilter(Object<ffi::GtkRecentFilter>): Buildable;

    match fn {
        get_type => || ffi::gtk_recent_filter_get_type(),
    }
}

impl RecentFilter {
    pub fn new() -> RecentFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_recent_filter_new())
        }
    }

    pub fn add_age(&self, days: i32) {
        unsafe {
            ffi::gtk_recent_filter_add_age(self.to_glib_none().0, days);
        }
    }

    pub fn add_application(&self, application: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_application(self.to_glib_none().0, application.to_glib_none().0);
        }
    }

    //pub fn add_custom(&self, needed: RecentFilterFlags, func: /*Unknown conversion*//*Unimplemented*/RecentFilterFunc, data: /*Unimplemented*/Fundamental: Pointer, data_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_recent_filter_add_custom() }
    //}

    pub fn add_group(&self, group: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_group(self.to_glib_none().0, group.to_glib_none().0);
        }
    }

    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    pub fn add_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_recent_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    pub fn add_pixbuf_formats(&self) {
        unsafe {
            ffi::gtk_recent_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    //pub fn filter(&self, filter_info: /*Ignored*/&RecentFilterInfo) -> bool {
    //    unsafe { TODO: call ffi::gtk_recent_filter_filter() }
    //}

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_filter_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_needed(&self) -> RecentFilterFlags {
        unsafe {
            ffi::gtk_recent_filter_get_needed(self.to_glib_none().0)
        }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_recent_filter_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }
}
