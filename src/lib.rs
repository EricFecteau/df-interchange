#![allow(clippy::useless_transmute)]
#![allow(dead_code)]
#![allow(unused_macros)]

mod error;
pub use error::InterchangeError;

#[cfg(any(
    feature = "arrow_50",
    feature = "arrow_51",
    feature = "arrow_52",
    feature = "arrow_53",
    feature = "arrow_54"
))]
mod from_arrow;

#[cfg(any(
    feature = "polars_0_40",
    feature = "polars_0_41",
    feature = "polars_0_42",
    feature = "polars_0_43",
    feature = "polars_0_44",
    feature = "polars_0_45",
    feature = "polars_0_46"
))]
mod from_polars;

#[cfg(any(
    feature = "arrow_50",
    feature = "arrow_51",
    feature = "arrow_52",
    feature = "arrow_53",
    feature = "arrow_54"
))]
mod to_arrow;

#[cfg(any(
    feature = "polars_0_40",
    feature = "polars_0_41",
    feature = "polars_0_42",
    feature = "polars_0_43",
    feature = "polars_0_44",
    feature = "polars_0_45",
    feature = "polars_0_46"
))]
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
