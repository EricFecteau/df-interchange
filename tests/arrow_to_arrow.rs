mod setup;
use df_interchange::{Interchange, InterchangeError};
use paste::paste;

macro_rules! test_arrow_to_arrow {
    ($from_ver:literal, $to_ver:literal) => {
        paste! {
            #[test]
            pub fn [<test_arrow_ $from_ver _to_arrow_ $to_ver>]() -> Result<(), InterchangeError> {
                let src_df = setup::[<arrow_data_ $from_ver>]();
                let converted_df = Interchange::[<from_arrow_ $from_ver>](src_df.clone())?.[<to_arrow_ $to_ver>]()?;
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

// ---------- Arrow 54 ---------- //

#[cfg(all(feature = "arrow_54", feature = "arrow_55"))]
test_arrow_to_arrow!("54", "55");

#[cfg(all(feature = "arrow_54", feature = "arrow_56"))]
test_arrow_to_arrow!("54", "56");

// ---------- Arrow 55 ---------- //

#[cfg(all(feature = "arrow_55", feature = "arrow_54"))]
test_arrow_to_arrow!("55", "54");

#[cfg(all(feature = "arrow_55", feature = "arrow_56"))]
test_arrow_to_arrow!("55", "56");

// ---------- Arrow 56 ---------- //

#[cfg(all(feature = "arrow_56", feature = "arrow_54"))]
test_arrow_to_arrow!("56", "54");

#[cfg(all(feature = "arrow_56", feature = "arrow_55"))]
test_arrow_to_arrow!("56", "55");
