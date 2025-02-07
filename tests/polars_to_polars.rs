mod common;
use df_interchange::Interchange;
use paste::paste;

macro_rules! test_polars_to_polars {
    ($from_ver:literal, $to_ver:literal) => {
        paste! {
            #[test]
            pub fn [<test_polars_ $from_ver _to_ $to_ver>]() {
                let src_df = common::[<polars_data_ $from_ver>]();
                let converted_df = Interchange::[<from_polars_ $from_ver>](src_df).[<to_polars_ $to_ver>]();
                let dst_df = common::[<polars_data_ $to_ver>]();

                assert!(dst_df.equals_missing(&converted_df));
            }

        }
    };
}

#[cfg(all(feature = "polars_0_42", feature = "polars_0_43"))]
test_polars_to_polars!("0_42", "0_43");

#[cfg(all(feature = "polars_0_42", feature = "polars_0_43"))]
test_polars_to_polars!("0_43", "0_42");

#[cfg(all(feature = "polars_0_43", feature = "polars_0_44"))]
test_polars_to_polars!("0_43", "0_44");

#[cfg(all(feature = "polars_0_43", feature = "polars_0_44"))]
test_polars_to_polars!("0_44", "0_43");

#[cfg(all(feature = "polars_0_44", feature = "polars_0_45"))]
test_polars_to_polars!("0_44", "0_45");

#[cfg(all(feature = "polars_0_44", feature = "polars_0_45"))]
test_polars_to_polars!("0_45", "0_44");

#[cfg(all(feature = "polars_0_45", feature = "polars_0_46"))]
test_polars_to_polars!("0_45", "0_46");

#[cfg(all(feature = "polars_0_45", feature = "polars_0_46"))]
test_polars_to_polars!("0_46", "0_45");

// #[test]
// #[cfg(all(
//     feature = "polars_0_44",
//     feature = "polars_0_45",
//     feature = "polars_0_46"
// ))]
// fn test_polars_0_44_0_45_0_46() {
//     let src1_df = common::polars_data_0_44();
//     let src2_df = common::polars_data_0_45();
//     let c1_df = Interchange::from_polars_0_44(src1_df).to_polars_0_46();
//     let c2_df = Interchange::from_polars_0_45(src2_df).to_polars_0_46();

//     assert!(c1_df.equals_missing(&c2_df));
// }
