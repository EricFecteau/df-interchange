mod setup;
use df_interchange::{Interchange, InterchangeError};
use paste::paste;

macro_rules! test_polars_to_polars {
    ($from_ver:literal, $to_ver:literal) => {
        paste! {
            #[test]
            pub fn [<test_polars_ $from_ver _to_polars_ $to_ver>]() -> Result<(), InterchangeError> {
                let src_df = setup::[<polars_data_ $from_ver>]();
                let converted_df = Interchange::[<from_polars_ $from_ver>](src_df.clone())?.[<to_polars_ $to_ver>]()?;
                let dst_df = setup::[<polars_data_ $to_ver>]();

                // Print if it fails
                println!("{:?}", &src_df);
                println!("{:?}", &converted_df);
                println!("{:?}", &dst_df);

                assert!(dst_df.equals_missing(&converted_df));

                Ok(())
            }

        }
    };
}

// ---------- Polars 0_40 ---------- //

#[cfg(all(feature = "polars_0_40", feature = "polars_0_41"))]
test_polars_to_polars!("0_40", "0_41");

#[cfg(all(feature = "polars_0_40", feature = "polars_0_42"))]
test_polars_to_polars!("0_40", "0_42");

#[cfg(all(feature = "polars_0_40", feature = "polars_0_43"))]
test_polars_to_polars!("0_40", "0_43");

#[cfg(all(feature = "polars_0_40", feature = "polars_0_44"))]
test_polars_to_polars!("0_40", "0_44");

#[cfg(all(feature = "polars_0_40", feature = "polars_0_45"))]
test_polars_to_polars!("0_40", "0_45");

#[cfg(all(feature = "polars_0_40", feature = "polars_0_46"))]
test_polars_to_polars!("0_40", "0_46");

// ---------- Polars 0_41 ---------- //

#[cfg(all(feature = "polars_0_41", feature = "polars_0_40"))]
test_polars_to_polars!("0_41", "0_40");

#[cfg(all(feature = "polars_0_41", feature = "polars_0_42"))]
test_polars_to_polars!("0_41", "0_42");

#[cfg(all(feature = "polars_0_41", feature = "polars_0_43"))]
test_polars_to_polars!("0_41", "0_43");

#[cfg(all(feature = "polars_0_41", feature = "polars_0_44"))]
test_polars_to_polars!("0_41", "0_44");

#[cfg(all(feature = "polars_0_41", feature = "polars_0_45"))]
test_polars_to_polars!("0_41", "0_45");

#[cfg(all(feature = "polars_0_41", feature = "polars_0_46"))]
test_polars_to_polars!("0_41", "0_46");

// ---------- Polars 0_42 ---------- //

#[cfg(all(feature = "polars_0_42", feature = "polars_0_40"))]
test_polars_to_polars!("0_42", "0_40");

#[cfg(all(feature = "polars_0_42", feature = "polars_0_41"))]
test_polars_to_polars!("0_42", "0_41");

#[cfg(all(feature = "polars_0_42", feature = "polars_0_43"))]
test_polars_to_polars!("0_42", "0_43");

#[cfg(all(feature = "polars_0_42", feature = "polars_0_44"))]
test_polars_to_polars!("0_42", "0_44");

#[cfg(all(feature = "polars_0_42", feature = "polars_0_45"))]
test_polars_to_polars!("0_42", "0_45");

#[cfg(all(feature = "polars_0_42", feature = "polars_0_46"))]
test_polars_to_polars!("0_42", "0_46");

// ---------- Polars 0_43 ---------- //

#[cfg(all(feature = "polars_0_43", feature = "polars_0_40"))]
test_polars_to_polars!("0_43", "0_40");

#[cfg(all(feature = "polars_0_43", feature = "polars_0_41"))]
test_polars_to_polars!("0_43", "0_41");

#[cfg(all(feature = "polars_0_43", feature = "polars_0_42"))]
test_polars_to_polars!("0_43", "0_42");

#[cfg(all(feature = "polars_0_43", feature = "polars_0_44"))]
test_polars_to_polars!("0_43", "0_44");

#[cfg(all(feature = "polars_0_43", feature = "polars_0_45"))]
test_polars_to_polars!("0_43", "0_45");

#[cfg(all(feature = "polars_0_43", feature = "polars_0_46"))]
test_polars_to_polars!("0_43", "0_46");

// ---------- Polars 0_44 ---------- //

#[cfg(all(feature = "polars_0_44", feature = "polars_0_40"))]
test_polars_to_polars!("0_44", "0_40");

#[cfg(all(feature = "polars_0_44", feature = "polars_0_41"))]
test_polars_to_polars!("0_44", "0_41");

#[cfg(all(feature = "polars_0_44", feature = "polars_0_42"))]
test_polars_to_polars!("0_44", "0_42");

#[cfg(all(feature = "polars_0_44", feature = "polars_0_43"))]
test_polars_to_polars!("0_44", "0_43");

#[cfg(all(feature = "polars_0_44", feature = "polars_0_45"))]
test_polars_to_polars!("0_44", "0_45");

#[cfg(all(feature = "polars_0_44", feature = "polars_0_46"))]
test_polars_to_polars!("0_44", "0_46");

// ---------- Polars 0_45 ---------- //

#[cfg(all(feature = "polars_0_45", feature = "polars_0_40"))]
test_polars_to_polars!("0_45", "0_40");

#[cfg(all(feature = "polars_0_45", feature = "polars_0_41"))]
test_polars_to_polars!("0_45", "0_41");

#[cfg(all(feature = "polars_0_45", feature = "polars_0_42"))]
test_polars_to_polars!("0_45", "0_42");

#[cfg(all(feature = "polars_0_45", feature = "polars_0_43"))]
test_polars_to_polars!("0_45", "0_43");

#[cfg(all(feature = "polars_0_45", feature = "polars_0_44"))]
test_polars_to_polars!("0_45", "0_44");

#[cfg(all(feature = "polars_0_45", feature = "polars_0_46"))]
test_polars_to_polars!("0_45", "0_46");

// ---------- Polars 0_46 ---------- //

#[cfg(all(feature = "polars_0_46", feature = "polars_0_40"))]
test_polars_to_polars!("0_46", "0_40");

#[cfg(all(feature = "polars_0_46", feature = "polars_0_41"))]
test_polars_to_polars!("0_46", "0_41");

#[cfg(all(feature = "polars_0_46", feature = "polars_0_42"))]
test_polars_to_polars!("0_46", "0_42");

#[cfg(all(feature = "polars_0_46", feature = "polars_0_43"))]
test_polars_to_polars!("0_46", "0_43");

#[cfg(all(feature = "polars_0_46", feature = "polars_0_44"))]
test_polars_to_polars!("0_46", "0_44");

#[cfg(all(feature = "polars_0_46", feature = "polars_0_45"))]
test_polars_to_polars!("0_46", "0_45");
