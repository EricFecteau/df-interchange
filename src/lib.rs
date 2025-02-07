#![allow(clippy::useless_transmute)]
mod from_polars;
mod to_polars;

#[repr(C)]
struct ArrowArray {
    length: i64,
    null_count: i64,
    offset: i64,
    n_buffers: i64,
    n_children: i64,
    buffers: *mut *const ::std::os::raw::c_void,
    children: *mut *mut ArrowArray,
    dictionary: *mut ArrowArray,
    release: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ArrowArray)>,
    private_data: *mut ::std::os::raw::c_void,
}

#[repr(C)]
struct ArrowSchema {
    format: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
    metadata: *const ::std::os::raw::c_char,
    flags: i64,
    n_children: i64,
    children: *mut *mut ArrowSchema,
    dictionary: *mut ArrowSchema,
    release: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ArrowSchema)>,
    private_data: *mut ::std::os::raw::c_void,
}

pub struct Interchange {
    ffi: Vec<(String, Vec<(ArrowArray, ArrowSchema)>)>,
}
