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

// ---------- Poars 0_40 ---------- //

#[cfg(all(feature = "polars_0_40", feature = "arrow_50"))]
test_arrow_to_polars!("0_40", "50");

#[cfg(all(feature = "polars_0_40", feature = "arrow_51"))]
test_arrow_to_polars!("0_40", "51");

#[cfg(all(feature = "polars_0_40", feature = "arrow_52"))]
test_arrow_to_polars!("0_40", "52");

#[cfg(all(feature = "polars_0_40", feature = "arrow_53"))]
test_arrow_to_polars!("0_40", "53");

#[cfg(all(feature = "polars_0_40", feature = "arrow_54"))]
test_arrow_to_polars!("0_40", "54");

// ---------- Poars 0_41 ---------- //

#[cfg(all(feature = "polars_0_41", feature = "arrow_50"))]
test_arrow_to_polars!("0_41", "50");

#[cfg(all(feature = "polars_0_41", feature = "arrow_51"))]
test_arrow_to_polars!("0_41", "51");

#[cfg(all(feature = "polars_0_41", feature = "arrow_52"))]
test_arrow_to_polars!("0_41", "52");

#[cfg(all(feature = "polars_0_41", feature = "arrow_53"))]
test_arrow_to_polars!("0_41", "53");

#[cfg(all(feature = "polars_0_41", feature = "arrow_54"))]
test_arrow_to_polars!("0_41", "54");

// ---------- Poars 0_42 ---------- //

#[cfg(all(feature = "polars_0_42", feature = "arrow_50"))]
test_arrow_to_polars!("0_42", "50");

#[cfg(all(feature = "polars_0_42", feature = "arrow_51"))]
test_arrow_to_polars!("0_42", "51");

#[cfg(all(feature = "polars_0_42", feature = "arrow_52"))]
test_arrow_to_polars!("0_42", "52");

#[cfg(all(feature = "polars_0_42", feature = "arrow_53"))]
test_arrow_to_polars!("0_42", "53");

#[cfg(all(feature = "polars_0_42", feature = "arrow_54"))]
test_arrow_to_polars!("0_42", "54");

// ---------- Poars 0_43 ---------- //

#[cfg(all(feature = "polars_0_43", feature = "arrow_50"))]
test_arrow_to_polars!("0_43", "50");

#[cfg(all(feature = "polars_0_43", feature = "arrow_51"))]
test_arrow_to_polars!("0_43", "51");

#[cfg(all(feature = "polars_0_43", feature = "arrow_52"))]
test_arrow_to_polars!("0_43", "52");

#[cfg(all(feature = "polars_0_43", feature = "arrow_53"))]
test_arrow_to_polars!("0_43", "53");

#[cfg(all(feature = "polars_0_43", feature = "arrow_54"))]
test_arrow_to_polars!("0_43", "54");

// ---------- Poars 0_44 ---------- //

#[cfg(all(feature = "polars_0_44", feature = "arrow_50"))]
test_arrow_to_polars!("0_44", "50");

#[cfg(all(feature = "polars_0_44", feature = "arrow_51"))]
test_arrow_to_polars!("0_44", "51");

#[cfg(all(feature = "polars_0_44", feature = "arrow_52"))]
test_arrow_to_polars!("0_44", "52");

#[cfg(all(feature = "polars_0_44", feature = "arrow_53"))]
test_arrow_to_polars!("0_44", "53");

#[cfg(all(feature = "polars_0_44", feature = "arrow_54"))]
test_arrow_to_polars!("0_44", "54");

// ---------- Poars 0_45 ---------- //

#[cfg(all(feature = "polars_0_45", feature = "arrow_50"))]
test_arrow_to_polars!("0_45", "50");

#[cfg(all(feature = "polars_0_45", feature = "arrow_51"))]
test_arrow_to_polars!("0_45", "51");

#[cfg(all(feature = "polars_0_45", feature = "arrow_52"))]
test_arrow_to_polars!("0_45", "52");

#[cfg(all(feature = "polars_0_45", feature = "arrow_53"))]
test_arrow_to_polars!("0_45", "53");

#[cfg(all(feature = "polars_0_45", feature = "arrow_54"))]
test_arrow_to_polars!("0_45", "54");

// ---------- Poars 0_46 ---------- //

#[cfg(all(feature = "polars_0_46", feature = "arrow_50"))]
test_arrow_to_polars!("0_46", "50");

#[cfg(all(feature = "polars_0_46", feature = "arrow_51"))]
test_arrow_to_polars!("0_46", "51");

#[cfg(all(feature = "polars_0_46", feature = "arrow_52"))]
test_arrow_to_polars!("0_46", "52");

#[cfg(all(feature = "polars_0_46", feature = "arrow_53"))]
test_arrow_to_polars!("0_46", "53");

#[cfg(all(feature = "polars_0_46", feature = "arrow_54"))]
test_arrow_to_polars!("0_46", "54");
