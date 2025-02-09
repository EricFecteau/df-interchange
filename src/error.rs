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

    #[cfg(feature = "arrow_50")]
    #[error(transparent)]
    Arrow50Error(#[from] arrow_crate_50::error::ArrowError),

    #[cfg(feature = "arrow_51")]
    #[error(transparent)]
    Arrow51Error(#[from] arrow_crate_51::error::ArrowError),

    #[cfg(feature = "arrow_52")]
    #[error(transparent)]
    Arrow52Error(#[from] arrow_crate_52::error::ArrowError),

    #[cfg(feature = "arrow_53")]
    #[error(transparent)]
    Arrow53Error(#[from] arrow_crate_53::error::ArrowError),

    #[cfg(feature = "arrow_54")]
    #[error(transparent)]
    Arrow54Error(#[from] arrow_crate_54::error::ArrowError),
}
