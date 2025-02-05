mod common;
use df_interchange::polars_to_polars;

#[test]
fn test_polars_to_polars() {
    let src_df = common::src_polars_data();
    let converted_df = polars_to_polars(src_df);
    let dst_df = common::dst_polars_data();

    assert!(dst_df.equals_missing(&converted_df));
}

#[test]
fn test_polars_versions() {
    let src_version = common::src_polars_version();
    let dst_version = common::dst_polars_version();

    // Source
    #[cfg(feature = "src_polars_0_43")]
    assert_eq!(src_version, "43");

    #[cfg(feature = "src_polars_0_44")]
    assert_eq!(src_version, "44");

    #[cfg(feature = "src_polars_0_45")]
    assert_eq!(src_version, "45");

    #[cfg(feature = "src_polars_0_46")]
    assert_eq!(src_version, "46");

    // Destination
    #[cfg(feature = "dst_polars_0_43")]
    assert_eq!(dst_version, "43");

    #[cfg(feature = "dst_polars_0_44")]
    assert_eq!(dst_version, "44");

    #[cfg(feature = "dst_polars_0_45")]
    assert_eq!(dst_version, "45");

    #[cfg(feature = "dst_polars_0_46")]
    assert_eq!(dst_version, "46");
}
