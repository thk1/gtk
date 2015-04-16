// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Text buffer for ::Entry

use libc::{c_int, c_uint};
use glib::translate::{FromGlibPtr, ToGlibPtr};
use ffi;

// TODO:
// Implements custom signal : inserted-text + deleted-text

/// EntryBuffer — Text buffer for ::Entry
/*
* # Signals available:
* * `deleted-text` : Run First
* * `inserted-text` : Run First
*
*/
pub struct EntryBuffer {
    pointer: *mut ffi::C_GtkEntryBuffer,
}

impl EntryBuffer {
    pub fn new(initial_chars: Option<&str>) -> Option<EntryBuffer> {
        let tmp_pointer = unsafe {
            ffi::gtk_entry_buffer_new(initial_chars.borrow_to_glib().0, -1)
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(EntryBuffer {
                pointer: tmp_pointer,
            })
        }
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_entry_buffer_get_text(self.pointer))
        }
    }

    pub fn set_text(&self, text: &str) -> () {
        unsafe {
            ffi::gtk_entry_buffer_set_text(self.pointer, text.borrow_to_glib().0, -1);
        }
    }

    pub fn get_bytes(&self) -> i64 {
        unsafe {
            ffi::gtk_entry_buffer_get_bytes(self.pointer) as i64
        }
    }

    pub fn get_length(&self) -> u32 {
        unsafe {
            ffi::gtk_entry_buffer_get_length(self.pointer) as u32
        }
    }

    pub fn get_max_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_buffer_get_max_length(self.pointer) as i32
        }
    }

    pub fn set_max_length(&self, max_length: i32) -> () {
        unsafe {
            ffi::gtk_entry_buffer_set_max_length(self.pointer, max_length as c_int)
        }
    }

    pub fn insert_text(&self, position: u32, text: &str) -> () {
        unsafe {
            ffi::gtk_entry_buffer_insert_text(self.pointer, position as c_uint,
                                              text.borrow_to_glib().0, -1);
        }
    }

    pub fn delete_text(&self, position: u32, n_chars: u32) -> u32 {
        unsafe {
            ffi::gtk_entry_buffer_delete_text(self.pointer, position as c_uint, n_chars as c_uint) as u32
        }
    }

    pub fn emit_deleted_test(&self, position: u32, n_chars: u32) -> () {
        unsafe {
            ffi::gtk_entry_buffer_emit_deleted_text(self.pointer, position as c_uint, n_chars as c_uint)
        }
    }

    pub fn emit_inserted_text(&self, position: u32, text: &str) -> () {
        unsafe {
            ffi::gtk_entry_buffer_emit_inserted_text(self.pointer, position as c_uint,
                                                     text.borrow_to_glib().0, -1);
        }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::C_GtkEntryBuffer {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(pointer: *mut ffi::C_GtkEntryBuffer) -> EntryBuffer {
        EntryBuffer {
            pointer:    pointer
        }
    }
}

impl_drop!(EntryBuffer, GTK_ENTRY_BUFFER);
