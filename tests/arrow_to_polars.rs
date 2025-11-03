mod setup;
use df_interchange::{Interchange, InterchangeError};
use paste::paste;

macro_rules! test_arrow_to_polars {
    ($from_ver:literal, $to_ver:literal) => {
        paste! {
            #[test]
            pub fn [<test_arrow_ $from_ver _to_polars_ $to_ver>]() -> Result<(), InterchangeError>  {
                let src_df = setup::[<arrow_data_ $from_ver>]();
                let converted_df = Interchange::[<from_arrow_ $from_ver>](src_df.clone())?.[<to_polars_ $to_ver>]()?;
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

// ---------- Arrow 0_54 ---------- //

#[cfg(all(feature = "arrow_54", feature = "polars_0_40"))]
test_arrow_to_polars!("54", "0_40");

#[cfg(all(feature = "arrow_54", feature = "polars_0_41"))]
test_arrow_to_polars!("54", "0_41");

#[cfg(all(feature = "arrow_54", feature = "polars_0_42"))]
test_arrow_to_polars!("54", "0_42");

#[cfg(all(feature = "arrow_54", feature = "polars_0_43"))]
test_arrow_to_polars!("54", "0_43");

#[cfg(all(feature = "arrow_54", feature = "polars_0_44"))]
test_arrow_to_polars!("54", "0_44");

#[cfg(all(feature = "arrow_54", feature = "polars_0_45"))]
test_arrow_to_polars!("54", "0_45");

#[cfg(all(feature = "arrow_54", feature = "polars_0_46"))]
test_arrow_to_polars!("54", "0_46");

#[cfg(all(feature = "arrow_54", feature = "polars_0_47"))]
test_arrow_to_polars!("54", "0_47");

#[cfg(all(feature = "arrow_54", feature = "polars_0_48"))]
test_arrow_to_polars!("54", "0_48");

#[cfg(all(feature = "arrow_54", feature = "polars_0_49"))]
test_arrow_to_polars!("54", "0_49");

#[cfg(all(feature = "arrow_54", feature = "polars_0_50"))]
test_arrow_to_polars!("54", "0_50");

#[cfg(all(feature = "arrow_54", feature = "polars_0_51"))]
test_arrow_to_polars!("54", "0_51");

#[cfg(all(feature = "arrow_54", feature = "polars_0_52"))]
test_arrow_to_polars!("54", "0_52");

// ---------- Arrow 0_55 ---------- //

#[cfg(all(feature = "arrow_55", feature = "polars_0_40"))]
test_arrow_to_polars!("55", "0_40");

#[cfg(all(feature = "arrow_55", feature = "polars_0_41"))]
test_arrow_to_polars!("55", "0_41");

#[cfg(all(feature = "arrow_55", feature = "polars_0_42"))]
test_arrow_to_polars!("55", "0_42");

#[cfg(all(feature = "arrow_55", feature = "polars_0_43"))]
test_arrow_to_polars!("55", "0_43");

#[cfg(all(feature = "arrow_55", feature = "polars_0_44"))]
test_arrow_to_polars!("55", "0_44");

#[cfg(all(feature = "arrow_55", feature = "polars_0_45"))]
test_arrow_to_polars!("55", "0_45");

#[cfg(all(feature = "arrow_55", feature = "polars_0_46"))]
test_arrow_to_polars!("55", "0_46");

#[cfg(all(feature = "arrow_55", feature = "polars_0_47"))]
test_arrow_to_polars!("55", "0_47");

#[cfg(all(feature = "arrow_55", feature = "polars_0_48"))]
test_arrow_to_polars!("55", "0_48");

#[cfg(all(feature = "arrow_55", feature = "polars_0_49"))]
test_arrow_to_polars!("55", "0_49");

#[cfg(all(feature = "arrow_55", feature = "polars_0_50"))]
test_arrow_to_polars!("55", "0_50");

#[cfg(all(feature = "arrow_55", feature = "polars_0_51"))]
test_arrow_to_polars!("55", "0_51");

#[cfg(all(feature = "arrow_55", feature = "polars_0_52"))]
test_arrow_to_polars!("55", "0_52");

// ---------- Arrow 0_56 ---------- //

#[cfg(all(feature = "arrow_56", feature = "polars_0_40"))]
test_arrow_to_polars!("56", "0_40");

#[cfg(all(feature = "arrow_56", feature = "polars_0_41"))]
test_arrow_to_polars!("56", "0_41");

#[cfg(all(feature = "arrow_56", feature = "polars_0_42"))]
test_arrow_to_polars!("56", "0_42");

#[cfg(all(feature = "arrow_56", feature = "polars_0_43"))]
test_arrow_to_polars!("56", "0_43");

#[cfg(all(feature = "arrow_56", feature = "polars_0_44"))]
test_arrow_to_polars!("56", "0_44");

#[cfg(all(feature = "arrow_56", feature = "polars_0_45"))]
test_arrow_to_polars!("56", "0_45");

#[cfg(all(feature = "arrow_56", feature = "polars_0_46"))]
test_arrow_to_polars!("56", "0_46");

#[cfg(all(feature = "arrow_56", feature = "polars_0_47"))]
test_arrow_to_polars!("56", "0_47");

#[cfg(all(feature = "arrow_56", feature = "polars_0_48"))]
test_arrow_to_polars!("56", "0_48");

#[cfg(all(feature = "arrow_56", feature = "polars_0_49"))]
test_arrow_to_polars!("56", "0_49");

#[cfg(all(feature = "arrow_56", feature = "polars_0_50"))]
test_arrow_to_polars!("56", "0_50");

#[cfg(all(feature = "arrow_56", feature = "polars_0_51"))]
test_arrow_to_polars!("56", "0_51");

#[cfg(all(feature = "arrow_56", feature = "polars_0_52"))]
test_arrow_to_polars!("56", "0_52");

// ---------- Arrow 0_57 ---------- //

#[cfg(all(feature = "arrow_57", feature = "polars_0_40"))]
test_arrow_to_polars!("57", "0_40");

#[cfg(all(feature = "arrow_57", feature = "polars_0_41"))]
test_arrow_to_polars!("57", "0_41");

#[cfg(all(feature = "arrow_57", feature = "polars_0_42"))]
test_arrow_to_polars!("57", "0_42");

#[cfg(all(feature = "arrow_57", feature = "polars_0_43"))]
test_arrow_to_polars!("57", "0_43");

#[cfg(all(feature = "arrow_57", feature = "polars_0_44"))]
test_arrow_to_polars!("57", "0_44");

#[cfg(all(feature = "arrow_57", feature = "polars_0_45"))]
test_arrow_to_polars!("57", "0_45");

#[cfg(all(feature = "arrow_57", feature = "polars_0_46"))]
test_arrow_to_polars!("57", "0_46");

#[cfg(all(feature = "arrow_57", feature = "polars_0_47"))]
test_arrow_to_polars!("57", "0_47");

#[cfg(all(feature = "arrow_57", feature = "polars_0_48"))]
test_arrow_to_polars!("57", "0_48");

#[cfg(all(feature = "arrow_57", feature = "polars_0_49"))]
test_arrow_to_polars!("57", "0_49");

#[cfg(all(feature = "arrow_57", feature = "polars_0_50"))]
test_arrow_to_polars!("57", "0_50");

#[cfg(all(feature = "arrow_57", feature = "polars_0_51"))]
test_arrow_to_polars!("57", "0_51");

#[cfg(all(feature = "arrow_57", feature = "polars_0_52"))]
test_arrow_to_polars!("57", "0_52");
