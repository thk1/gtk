// This file was generated by gir (e6cb5d0) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
#[cfg(feature = "v3_10")]
use PlacesOpenFlags;
use ScrolledWindow;
use Widget;
use ffi;
use glib::object::Downcast;
#[cfg(feature = "v3_10")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_10")]
use glib_ffi;
#[cfg(feature = "v3_10")]
use libc;
#[cfg(feature = "v3_10")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_10")]
use std::mem::transmute;

glib_wrapper! {
    pub struct PlacesSidebar(Object<ffi::GtkPlacesSidebar>): ScrolledWindow, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_places_sidebar_get_type(),
    }
}

impl PlacesSidebar {
    #[cfg(feature = "v3_10")]
    pub fn new() -> PlacesSidebar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_places_sidebar_new()).downcast_unchecked()
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn add_shortcut<T: IsA</*Ignored*/gio::File>>(&self, location: &T) {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_add_shortcut() }
    //}

    #[cfg(feature = "v3_12")]
    pub fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_local_only(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn get_location(&self) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_get_location() }
    //}

    //#[cfg(feature = "v3_10")]
    //pub fn get_nth_bookmark(&self, n: i32) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_get_nth_bookmark() }
    //}

    #[cfg(feature = "v3_10")]
    pub fn get_open_flags(&self) -> PlacesOpenFlags {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_open_flags(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_show_connect_to_server(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_connect_to_server(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_show_desktop(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_desktop(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_show_enter_location(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_enter_location(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn list_shortcuts(&self) -> /*Ignored*/Vec<gio::File> {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_list_shortcuts() }
    //}

    //#[cfg(feature = "v3_10")]
    //pub fn remove_shortcut<T: IsA</*Ignored*/gio::File>>(&self, location: &T) {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_remove_shortcut() }
    //}

    #[cfg(feature = "v3_12")]
    pub fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_local_only(self.to_glib_none().0, local_only.to_glib());
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn set_location<T: IsA</*Ignored*/gio::File>>(&self, location: Option<&T>) {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_set_location() }
    //}

    #[cfg(feature = "v3_10")]
    pub fn set_open_flags(&self, flags: PlacesOpenFlags) {
        unsafe {
            ffi::gtk_places_sidebar_set_open_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_show_connect_to_server(&self, show_connect_to_server: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_connect_to_server(self.to_glib_none().0, show_connect_to_server.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_show_desktop(&self, show_desktop: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_desktop(self.to_glib_none().0, show_desktop.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_show_enter_location(&self, show_enter_location: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_enter_location(self.to_glib_none().0, show_enter_location.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn connect_drag_action_ask<F: Fn(&PlacesSidebar, i32) -> i32 + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&PlacesSidebar, i32) -> i32 + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "drag-action-ask",
                transmute(drag_action_ask_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //#[cfg(feature = "v3_10")]
    //pub fn connect_drag_action_requested<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored context: Gdk.DragContext
    //    Ignored dest_file: Gio.File
    //    Ignored source_file_list: *.List TypeId { ns_id: 6, id: 149 }
    //}

    //#[cfg(feature = "v3_10")]
    //pub fn connect_drag_perform_drop<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored dest_file: Gio.File
    //    Ignored source_file_list: *.List TypeId { ns_id: 6, id: 149 }
    //}

    //#[cfg(feature = "v3_10")]
    //pub fn connect_open_location<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored location: Gio.File
    //}

    //#[cfg(feature = "v3_10")]
    //pub fn connect_populate_popup<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored selected_item: Gio.File
    //    Ignored selected_volume: Gio.Volume
    //}

    #[cfg(feature = "v3_10")]
    pub fn connect_show_connect_to_server<F: Fn(&PlacesSidebar) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&PlacesSidebar) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-connect-to-server",
                transmute(show_connect_to_server_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_show_enter_location<F: Fn(&PlacesSidebar) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&PlacesSidebar) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-enter-location",
                transmute(show_enter_location_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn connect_show_error_message<F: Fn(&PlacesSidebar, &str, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&PlacesSidebar, &str, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-error-message",
                transmute(show_error_message_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn drag_action_ask_trampoline(this: *mut ffi::GtkPlacesSidebar, actions: libc::c_int, f: glib_ffi::gpointer) -> libc::c_int {
    callback_guard!();
    let f: &Box_<Fn(&PlacesSidebar, i32) -> i32 + 'static> = transmute(f);
    f(&from_glib_none(this), actions)
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn show_connect_to_server_trampoline(this: *mut ffi::GtkPlacesSidebar, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&PlacesSidebar) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn show_enter_location_trampoline(this: *mut ffi::GtkPlacesSidebar, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&PlacesSidebar) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn show_error_message_trampoline(this: *mut ffi::GtkPlacesSidebar, primary: *mut libc::c_char, secondary: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&PlacesSidebar, &str, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(primary), &String::from_glib_none(secondary))
}
