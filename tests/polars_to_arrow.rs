mod setup;
use df_interchange::{Interchange, InterchangeError};
use paste::paste;

macro_rules! test_arrow_to_polars {
    ($from_ver:literal, $to_ver:literal) => {
        paste! {
            #[test]
            pub fn [<test_polars_ $from_ver _to_arrow_ $to_ver>]() -> Result<(), InterchangeError> {
                let src_df = setup::[<polars_data_ $from_ver>]();
                let converted_df = Interchange::[<from_polars_ $from_ver>](src_df.clone())?.[<to_arrow_ $to_ver>]()?;
                let dst_df = setup::[<arrow_data_ $to_ver>]();

                // Print if it fails
                println!("{:?}", &src_df);
                println!("{:?}", &converted_df);
                println!("{:?}", &dst_df);

                assert!(dst_df.eq(&converted_df));

                Ok(())
            }

        }
    };
}

// ---------- Polars 0_40 ---------- //

#[cfg(all(feature = "polars_0_40", feature = "arrow_54"))]
test_arrow_to_polars!("0_40", "54");

#[cfg(all(feature = "polars_0_40", feature = "arrow_55"))]
test_arrow_to_polars!("0_40", "55");

// ---------- Polars 0_41 ---------- //

#[cfg(all(feature = "polars_0_41", feature = "arrow_54"))]
test_arrow_to_polars!("0_41", "54");

#[cfg(all(feature = "polars_0_41", feature = "arrow_55"))]
test_arrow_to_polars!("0_41", "55");

// ---------- Polars 0_42 ---------- //

#[cfg(all(feature = "polars_0_42", feature = "arrow_54"))]
test_arrow_to_polars!("0_42", "54");

#[cfg(all(feature = "polars_0_42", feature = "arrow_55"))]
test_arrow_to_polars!("0_42", "55");

// ---------- Polars 0_43 ---------- //

#[cfg(all(feature = "polars_0_43", feature = "arrow_54"))]
test_arrow_to_polars!("0_43", "54");

#[cfg(all(feature = "polars_0_43", feature = "arrow_55"))]
test_arrow_to_polars!("0_43", "55");

// ---------- Polars 0_44 ---------- //

#[cfg(all(feature = "polars_0_44", feature = "arrow_54"))]
test_arrow_to_polars!("0_44", "54");

#[cfg(all(feature = "polars_0_44", feature = "arrow_55"))]
test_arrow_to_polars!("0_44", "55");

// ---------- Polars 0_45 ---------- //

#[cfg(all(feature = "polars_0_45", feature = "arrow_54"))]
test_arrow_to_polars!("0_45", "54");

#[cfg(all(feature = "polars_0_45", feature = "arrow_55"))]
test_arrow_to_polars!("0_45", "55");

// ---------- Polars 0_46 ---------- //

#[cfg(all(feature = "polars_0_46", feature = "arrow_54"))]
test_arrow_to_polars!("0_46", "54");

#[cfg(all(feature = "polars_0_46", feature = "arrow_55"))]
test_arrow_to_polars!("0_46", "55");
