use paste::paste;

const TEST_I32: [std::option::Option<i32>; 6] =
    [Some(0i32), Some(1), Some(2), Some(3), Some(4), None];

const TEST_I64: [std::option::Option<i64>; 6] =
    [Some(0i64), Some(1), Some(2), Some(3), Some(4), None];

#[allow(unused_macros)]
macro_rules! create_data {
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
create_data!("0_40");

#[cfg(feature = "polars_0_41")]
create_data!("0_41");

#[cfg(feature = "polars_0_42")]
create_data!("0_42");

#[cfg(feature = "polars_0_43")]
create_data!("0_43");

#[allow(unused_macros)]
macro_rules! create_data {
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
create_data!("0_44");

#[cfg(feature = "polars_0_45")]
create_data!("0_45");

#[cfg(feature = "polars_0_46")]
create_data!("0_46");
