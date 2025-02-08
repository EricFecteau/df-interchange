mod setup;
use df_interchange::Interchange;
use paste::paste;

macro_rules! test_arrow_to_arrow {
    ($from_ver:literal, $to_ver:literal) => {
        paste! {
            #[test]
            pub fn [<test_arrow_ $from_ver _to_arrow_ $to_ver>]() {
                let src_df = setup::[<arrow_data_ $from_ver>]();
                let converted_df = Interchange::[<from_arrow_ $from_ver>](src_df).[<to_arrow_ $to_ver>]();
                let dst_df = setup::[<arrow_data_ $to_ver>]();

                assert!(dst_df.eq(&converted_df));
            }

        }
    };
}

// ---------- Arrow 50 ---------- //

#[cfg(all(feature = "arrow_50", feature = "arrow_51"))]
test_arrow_to_arrow!("50", "51");

#[cfg(all(feature = "arrow_50", feature = "arrow_52"))]
test_arrow_to_arrow!("50", "52");

#[cfg(all(feature = "arrow_50", feature = "arrow_53"))]
test_arrow_to_arrow!("50", "53");

#[cfg(all(feature = "arrow_50", feature = "arrow_54"))]
test_arrow_to_arrow!("50", "54");

// ---------- Arrow 51 ---------- //

#[cfg(all(feature = "arrow_51", feature = "arrow_51"))]
test_arrow_to_arrow!("51", "51");

#[cfg(all(feature = "arrow_51", feature = "arrow_52"))]
test_arrow_to_arrow!("51", "52");

#[cfg(all(feature = "arrow_51", feature = "arrow_53"))]
test_arrow_to_arrow!("51", "53");

#[cfg(all(feature = "arrow_51", feature = "arrow_54"))]
test_arrow_to_arrow!("51", "54");

// ---------- Arrow 52 ---------- //

#[cfg(all(feature = "arrow_52", feature = "arrow_51"))]
test_arrow_to_arrow!("52", "51");

#[cfg(all(feature = "arrow_52", feature = "arrow_52"))]
test_arrow_to_arrow!("52", "52");

#[cfg(all(feature = "arrow_52", feature = "arrow_53"))]
test_arrow_to_arrow!("52", "53");

#[cfg(all(feature = "arrow_52", feature = "arrow_54"))]
test_arrow_to_arrow!("52", "54");

// ---------- Arrow 53 ---------- //

#[cfg(all(feature = "arrow_53", feature = "arrow_51"))]
test_arrow_to_arrow!("53", "51");

#[cfg(all(feature = "arrow_53", feature = "arrow_52"))]
test_arrow_to_arrow!("53", "52");

#[cfg(all(feature = "arrow_53", feature = "arrow_53"))]
test_arrow_to_arrow!("53", "53");

#[cfg(all(feature = "arrow_53", feature = "arrow_54"))]
test_arrow_to_arrow!("53", "54");

// ---------- Arrow 54 ---------- //

#[cfg(all(feature = "arrow_54", feature = "arrow_51"))]
test_arrow_to_arrow!("54", "51");

#[cfg(all(feature = "arrow_54", feature = "arrow_52"))]
test_arrow_to_arrow!("54", "52");

#[cfg(all(feature = "arrow_54", feature = "arrow_53"))]
test_arrow_to_arrow!("54", "53");

#[cfg(all(feature = "arrow_54", feature = "arrow_54"))]
test_arrow_to_arrow!("54", "54");
