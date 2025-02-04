#[cfg(feature = "src_polars_0_43")]
use polars_core_0_44 as src_polars;

#[cfg(feature = "dst_polars_0_43")]
use polars_core_0_44 as dst_polars;

#[cfg(feature = "src_polars_0_44")]
use polars_core_0_44 as src_polars;

#[cfg(feature = "dst_polars_0_44")]
use polars_core_0_44 as dst_polars;

#[cfg(feature = "src_polars_0_45")]
use polars_core_0_45 as src_polars;

#[cfg(feature = "dst_polars_0_45")]
use polars_core_0_45 as dst_polars;

#[cfg(feature = "src_polars_0_46")]
use polars_core_0_46 as src_polars;

#[cfg(feature = "dst_polars_0_46")]
use polars_core_0_46 as dst_polars;

const TEST_I32: [std::option::Option<i32>; 6] =
    [Some(0i32), Some(1), Some(2), Some(3), Some(4), None];

const TEST_I64: [std::option::Option<i64>; 6] =
    [Some(0i64), Some(1), Some(2), Some(3), Some(4), None];

pub fn src_polars_data() -> src_polars::frame::DataFrame {
    let c1 = src_polars::frame::column::Column::new("test_i32".into(), TEST_I32);
    let c2 = src_polars::frame::column::Column::new("test_i64".into(), TEST_I64);

    src_polars::frame::DataFrame::new(vec![c1, c2]).unwrap()
}

pub fn dst_polars_data() -> dst_polars::frame::DataFrame {
    let c1 = dst_polars::frame::column::Column::new("test_i32".into(), TEST_I32);
    let c2 = dst_polars::frame::column::Column::new("test_i64".into(), TEST_I64);

    dst_polars::frame::DataFrame::new(vec![c1, c2]).unwrap()
}

pub fn src_polars_version() -> String {
    src_polars::VERSION.split(".").nth(1).unwrap().to_owned()
}

pub fn dst_polars_version() -> String {
    dst_polars::VERSION.split(".").nth(1).unwrap().to_owned()
}
