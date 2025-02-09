#![allow(dead_code)]
#![allow(unused_macros)]

use paste::paste;

const TEST_I32: [std::option::Option<i32>; 6] = [
    Some(-1i32),
    Some(0),
    Some(1),
    Some(i32::MIN),
    Some(i32::MAX),
    None,
];

const TEST_I64: [std::option::Option<i64>; 6] = [
    Some(-1i64),
    Some(0),
    Some(1),
    Some(i64::MIN),
    Some(i64::MAX),
    None,
];

const TEST_U32: [std::option::Option<u32>; 6] = [
    Some(0u32),
    Some(1),
    Some(2),
    Some(u32::MIN),
    Some(u32::MAX),
    None,
];

const TEST_U64: [std::option::Option<u64>; 6] = [
    Some(0u64),
    Some(1),
    Some(2),
    Some(u64::MIN),
    Some(u64::MAX),
    None,
];

const TEST_F32: [std::option::Option<f32>; 6] = [
    Some(-10.00001f32),
    Some(0.0),
    Some(10.00001),
    Some(f32::MIN),
    Some(f32::MAX),
    None,
];

const TEST_F64: [std::option::Option<f64>; 6] = [
    Some(-10.000000001f64),
    Some(0.0),
    Some(10.000000001),
    Some(f64::MIN),
    Some(f64::MAX),
    None,
];

const TEST_BOOL: [std::option::Option<bool>; 6] = [Some(true), Some(false), None, None, None, None];

macro_rules! create_polars_data {
    ($version:literal) => {
        paste! {
            pub fn [<polars_data_ $version>]() -> [<polars_crate_ $version>]::frame::DataFrame {
                use [<polars_crate_ $version>]::prelude::NamedFrom;

                let df_vec = vec![
                    [<polars_crate_ $version>]::series::Series::new("test_i32".into(), TEST_I32),
                    [<polars_crate_ $version>]::series::Series::new("test_i64".into(), TEST_I64),
                    [<polars_crate_ $version>]::series::Series::new("test_u32".into(), TEST_U32),
                    [<polars_crate_ $version>]::series::Series::new("test_u64".into(), TEST_U64),
                    [<polars_crate_ $version>]::series::Series::new("test_f32".into(), TEST_F32),
                    [<polars_crate_ $version>]::series::Series::new("test_f64".into(), TEST_F64),
                    [<polars_crate_ $version>]::series::Series::new("test_bool".into(), TEST_BOOL),
                ];

                [<polars_crate_ $version>]::frame::DataFrame::new(df_vec).unwrap()
            }

        }
    };
}
#[cfg(feature = "polars_0_40")]
create_polars_data!("0_40");

#[cfg(feature = "polars_0_41")]
create_polars_data!("0_41");

#[cfg(feature = "polars_0_42")]
create_polars_data!("0_42");

#[cfg(feature = "polars_0_43")]
create_polars_data!("0_43");

macro_rules! create_polars_data {
    ($version:literal) => {
        paste! {
            pub fn [<polars_data_ $version>]() -> [<polars_crate_ $version>]::frame::DataFrame {

                let df_vec = vec![
                    [<polars_crate_ $version>]::frame::column::Column::new("test_i32".into(), TEST_I32),
                    [<polars_crate_ $version>]::frame::column::Column::new("test_i64".into(), TEST_I64),
                    [<polars_crate_ $version>]::frame::column::Column::new("test_u32".into(), TEST_U32),
                    [<polars_crate_ $version>]::frame::column::Column::new("test_u64".into(), TEST_U64),
                    [<polars_crate_ $version>]::frame::column::Column::new("test_f32".into(), TEST_F32),
                    [<polars_crate_ $version>]::frame::column::Column::new("test_f64".into(), TEST_F64),
                    [<polars_crate_ $version>]::frame::column::Column::new("test_bool".into(), TEST_BOOL),
                ];

                [<polars_crate_ $version>]::frame::DataFrame::new(df_vec).unwrap()
            }

        }
    };
}
#[cfg(feature = "polars_0_44")]
create_polars_data!("0_44");

#[cfg(feature = "polars_0_45")]
create_polars_data!("0_45");

#[cfg(feature = "polars_0_46")]
create_polars_data!("0_46");

macro_rules! create_arrow_data {
    ($version:literal) => {
        paste! {
            pub fn [<arrow_data_ $version>]() -> Vec<[<arrow_crate_ $version>]::record_batch::RecordBatch> {

                let record_batch = [<arrow_crate_ $version>]::record_batch::RecordBatch::try_from_iter(vec![
                  ("test_i32", std::sync::Arc::new([<arrow_crate_ $version>]::array::Int32Array::from(TEST_I32.to_vec())) as [<arrow_crate_ $version>]::array::ArrayRef),
                  ("test_i64", std::sync::Arc::new([<arrow_crate_ $version>]::array::Int64Array::from(TEST_I64.to_vec())) as [<arrow_crate_ $version>]::array::ArrayRef),
                  ("test_u32", std::sync::Arc::new([<arrow_crate_ $version>]::array::UInt32Array::from(TEST_U32.to_vec())) as [<arrow_crate_ $version>]::array::ArrayRef),
                  ("test_u64", std::sync::Arc::new([<arrow_crate_ $version>]::array::UInt64Array::from(TEST_U64.to_vec())) as [<arrow_crate_ $version>]::array::ArrayRef),
                  ("test_f32", std::sync::Arc::new([<arrow_crate_ $version>]::array::Float32Array::from(TEST_F32.to_vec())) as [<arrow_crate_ $version>]::array::ArrayRef),
                  ("test_f64", std::sync::Arc::new([<arrow_crate_ $version>]::array::Float64Array::from(TEST_F64.to_vec())) as [<arrow_crate_ $version>]::array::ArrayRef),
                  ("test_bool", std::sync::Arc::new([<arrow_crate_ $version>]::array::BooleanArray::from(TEST_BOOL.to_vec())) as [<arrow_crate_ $version>]::array::ArrayRef),
                ]).unwrap();

                vec![record_batch]
            }

        }
    };
}

#[cfg(feature = "arrow_50")]
create_arrow_data!("50");

#[cfg(feature = "arrow_51")]
create_arrow_data!("51");

#[cfg(feature = "arrow_52")]
create_arrow_data!("52");

#[cfg(feature = "arrow_53")]
create_arrow_data!("53");

#[cfg(feature = "arrow_54")]
create_arrow_data!("54");
