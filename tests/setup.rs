#![allow(dead_code)]
#![allow(unused_macros)]

use paste::paste;

const TEST_I32: [std::option::Option<i32>; 6] =
    [Some(0i32), Some(1), Some(2), Some(3), Some(4), None];

const TEST_I64: [std::option::Option<i64>; 6] =
    [Some(0i64), Some(1), Some(2), Some(3), Some(4), None];

macro_rules! create_polars_data {
    ($version:literal) => {
        paste! {
            pub fn [<polars_data_ $version>]() -> [<polars_crate_ $version>]::frame::DataFrame {
                use [<polars_crate_ $version>]::prelude::NamedFrom;

                let c1 = [<polars_crate_ $version>]::series::Series::new("test_i32".into(), TEST_I32);
                let c2 = [<polars_crate_ $version>]::series::Series::new("test_i64".into(), TEST_I64);

                [<polars_crate_ $version>]::frame::DataFrame::new(vec![c1, c2]).unwrap()
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
                let c1 = [<polars_crate_ $version>]::frame::column::Column::new("test_i32".into(), TEST_I32);
                let c2 = [<polars_crate_ $version>]::frame::column::Column::new("test_i64".into(), TEST_I64);

                [<polars_crate_ $version>]::frame::DataFrame::new(vec![c1, c2]).unwrap()
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

                let test_i32: [<arrow_crate_ $version>]::array::ArrayRef = std::sync::Arc::new([<arrow_crate_ $version>]::array::Int32Array::from(TEST_I32.to_vec()));
                let test_i64: [<arrow_crate_ $version>]::array::ArrayRef = std::sync::Arc::new([<arrow_crate_ $version>]::array::Int64Array::from(TEST_I64.to_vec()));

                let record_batch = [<arrow_crate_ $version>]::record_batch::RecordBatch::try_from_iter(vec![
                  ("test_i32", test_i32),
                  ("test_i64", test_i64),
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
