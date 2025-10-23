#![allow(clippy::enum_variant_names)]

use thiserror::Error;

pub type Result<T> = std::result::Result<T, InterchangeError>;

#[derive(Error, Debug)]
pub enum InterchangeError {
    #[cfg(feature = "polars_0_40")]
    #[error(transparent)]
    Polars0_40Error(#[from] polars_crate_0_40::error::PolarsError),

    #[cfg(feature = "polars_0_41")]
    #[error(transparent)]
    Polars0_41Error(#[from] polars_crate_0_41::error::PolarsError),

    #[cfg(feature = "polars_0_42")]
    #[error(transparent)]
    Polars0_42Error(#[from] polars_crate_0_42::error::PolarsError),

    #[cfg(feature = "polars_0_43")]
    #[error(transparent)]
    Polars0_43Error(#[from] polars_crate_0_43::error::PolarsError),

    #[cfg(feature = "polars_0_44")]
    #[error(transparent)]
    Polars0_44Error(#[from] polars_crate_0_44::error::PolarsError),

    #[cfg(feature = "polars_0_45")]
    #[error(transparent)]
    Polars0_45Error(#[from] polars_crate_0_45::error::PolarsError),

    #[cfg(feature = "polars_0_46")]
    #[error(transparent)]
    Polars0_46Error(#[from] polars_crate_0_46::error::PolarsError),

    #[cfg(feature = "polars_0_47")]
    #[error(transparent)]
    Polars0_47Error(#[from] polars_crate_0_47::error::PolarsError),

    #[cfg(feature = "polars_0_48")]
    #[error(transparent)]
    Polars0_48Error(#[from] polars_crate_0_48::error::PolarsError),

    #[cfg(feature = "polars_0_49")]
    #[error(transparent)]
    Polars0_49Error(#[from] polars_crate_0_49::error::PolarsError),

    #[cfg(feature = "polars_0_50")]
    #[error(transparent)]
    Polars0_50Error(#[from] polars_crate_0_50::error::PolarsError),

    #[cfg(feature = "polars_0_51")]
    #[error(transparent)]
    Polars0_51Error(#[from] polars_crate_0_51::error::PolarsError),

    #[cfg(feature = "arrow_54")]
    #[error(transparent)]
    Arrow54Error(#[from] arrow_crate_54::error::ArrowError),

    #[cfg(feature = "arrow_55")]
    #[error(transparent)]
    Arrow55Error(#[from] arrow_crate_55::error::ArrowError),

    #[cfg(feature = "arrow_56")]
    #[error(transparent)]
    Arrow56Error(#[from] arrow_crate_56::error::ArrowError),

    #[error("Chunks must be aligned when moving data from Polars to Arrow.")]
    ChunksNotAligned,
}
