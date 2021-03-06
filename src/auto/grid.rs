// This file was generated by gir (9bd51ed) from gir-files (db49619)
// DO NOT EDIT

#[cfg(feature = "v3_10")]
use BaselinePosition;
use Container;
use Orientable;
use PositionType;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Grid(Object<ffi::GtkGrid>): Container, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_grid_get_type(),
    }
}

impl Grid {
    pub fn new() -> Grid {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_grid_new()).downcast_unchecked()
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

pub trait GridExt {
    fn attach<P: IsA<Widget>>(&self, child: &P, left: i32, top: i32, width: i32, height: i32);

    fn attach_next_to<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, sibling: R, side: PositionType, width: i32, height: i32);

    #[cfg(feature = "v3_10")]
    fn get_baseline_row(&self) -> i32;

    fn get_child_at(&self, left: i32, top: i32) -> Option<Widget>;

    fn get_column_homogeneous(&self) -> bool;

    fn get_column_spacing(&self) -> u32;

    #[cfg(feature = "v3_10")]
    fn get_row_baseline_position(&self, row: i32) -> BaselinePosition;

    fn get_row_homogeneous(&self) -> bool;

    fn get_row_spacing(&self) -> u32;

    fn insert_column(&self, position: i32);

    fn insert_next_to<P: IsA<Widget>>(&self, sibling: &P, side: PositionType);

    fn insert_row(&self, position: i32);

    #[cfg(feature = "v3_10")]
    fn remove_column(&self, position: i32);

    #[cfg(feature = "v3_10")]
    fn remove_row(&self, position: i32);

    #[cfg(feature = "v3_10")]
    fn set_baseline_row(&self, row: i32);

    fn set_column_homogeneous(&self, homogeneous: bool);

    fn set_column_spacing(&self, spacing: u32);

    #[cfg(feature = "v3_10")]
    fn set_row_baseline_position(&self, row: i32, pos: BaselinePosition);

    fn set_row_homogeneous(&self, homogeneous: bool);

    fn set_row_spacing(&self, spacing: u32);

    fn get_property_baseline_row(&self) -> i32;

    fn set_property_baseline_row(&self, baseline_row: i32);

    fn get_cell_height<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_cell_height<T: IsA<Widget>>(&self, item: &T, height: i32);

    fn get_cell_width<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_cell_width<T: IsA<Widget>>(&self, item: &T, width: i32);

    fn get_cell_left_attach<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_cell_left_attach<T: IsA<Widget>>(&self, item: &T, left_attach: i32);

    fn get_cell_top_attach<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_cell_top_attach<T: IsA<Widget>>(&self, item: &T, top_attach: i32);

    fn connect_property_baseline_row_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_column_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_row_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Grid> + IsA<Container> + IsA<glib::object::Object>> GridExt for O {
    fn attach<P: IsA<Widget>>(&self, child: &P, left: i32, top: i32, width: i32, height: i32) {
        unsafe {
            ffi::gtk_grid_attach(self.to_glib_none().0, child.to_glib_none().0, left, top, width, height);
        }
    }

    fn attach_next_to<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, sibling: R, side: PositionType, width: i32, height: i32) {
        let sibling = sibling.into();
        let sibling = sibling.to_glib_none();
        unsafe {
            ffi::gtk_grid_attach_next_to(self.to_glib_none().0, child.to_glib_none().0, sibling.0, side.to_glib(), width, height);
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_baseline_row(&self) -> i32 {
        unsafe {
            ffi::gtk_grid_get_baseline_row(self.to_glib_none().0)
        }
    }

    fn get_child_at(&self, left: i32, top: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_grid_get_child_at(self.to_glib_none().0, left, top))
        }
    }

    fn get_column_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_get_column_homogeneous(self.to_glib_none().0))
        }
    }

    fn get_column_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_grid_get_column_spacing(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_row_baseline_position(&self, row: i32) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_grid_get_row_baseline_position(self.to_glib_none().0, row))
        }
    }

    fn get_row_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_get_row_homogeneous(self.to_glib_none().0))
        }
    }

    fn get_row_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_grid_get_row_spacing(self.to_glib_none().0)
        }
    }

    fn insert_column(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_insert_column(self.to_glib_none().0, position);
        }
    }

    fn insert_next_to<P: IsA<Widget>>(&self, sibling: &P, side: PositionType) {
        unsafe {
            ffi::gtk_grid_insert_next_to(self.to_glib_none().0, sibling.to_glib_none().0, side.to_glib());
        }
    }

    fn insert_row(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_insert_row(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v3_10")]
    fn remove_column(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_remove_column(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v3_10")]
    fn remove_row(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_remove_row(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_baseline_row(&self, row: i32) {
        unsafe {
            ffi::gtk_grid_set_baseline_row(self.to_glib_none().0, row);
        }
    }

    fn set_column_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_set_column_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_set_column_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_row_baseline_position(&self, row: i32, pos: BaselinePosition) {
        unsafe {
            ffi::gtk_grid_set_row_baseline_position(self.to_glib_none().0, row, pos.to_glib());
        }
    }

    fn set_row_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_set_row_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_set_row_spacing(self.to_glib_none().0, spacing);
        }
    }

    fn get_property_baseline_row(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "baseline-row".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_baseline_row(&self, baseline_row: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "baseline-row".to_glib_none().0, Value::from(&baseline_row).to_glib_none().0);
        }
    }

    fn get_cell_height<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "height".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_cell_height<T: IsA<Widget>>(&self, item: &T, height: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "height".to_glib_none().0, Value::from(&height).to_glib_none().0);
        }
    }

    fn get_cell_width<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "width".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_cell_width<T: IsA<Widget>>(&self, item: &T, width: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "width".to_glib_none().0, Value::from(&width).to_glib_none().0);
        }
    }

    fn get_cell_left_attach<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "left-attach".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_cell_left_attach<T: IsA<Widget>>(&self, item: &T, left_attach: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "left-attach".to_glib_none().0, Value::from(&left_attach).to_glib_none().0);
        }
    }

    fn get_cell_top_attach<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "top-attach".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_cell_top_attach<T: IsA<Widget>>(&self, item: &T, top_attach: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "top-attach".to_glib_none().0, Value::from(&top_attach).to_glib_none().0);
        }
    }

    fn connect_property_baseline_row_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::baseline-row",
                transmute(notify_baseline_row_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_column_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::column-homogeneous",
                transmute(notify_column_homogeneous_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::column-spacing",
                transmute(notify_column_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_row_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::row-homogeneous",
                transmute(notify_row_homogeneous_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::row-spacing",
                transmute(notify_row_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_baseline_row_trampoline<P>(this: *mut ffi::GtkGrid, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Grid> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Grid::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_column_homogeneous_trampoline<P>(this: *mut ffi::GtkGrid, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Grid> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Grid::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_column_spacing_trampoline<P>(this: *mut ffi::GtkGrid, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Grid> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Grid::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_row_homogeneous_trampoline<P>(this: *mut ffi::GtkGrid, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Grid> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Grid::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_row_spacing_trampoline<P>(this: *mut ffi::GtkGrid, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Grid> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Grid::from_glib_none(this).downcast_unchecked())
}
