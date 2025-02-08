use paste::paste;
use std::mem::transmute;

use crate::{ArrowArray, ArrowSchema, Interchange};

#[allow(unused_macros)]
macro_rules! polars_to_ffi {
    ($from_ver:literal) => {
        paste! {
            impl Interchange {
                #[doc = "Move Polars version `" $from_ver "` to the Arrow data interchange format."]
                pub fn [<from_polars_ $from_ver>](df: [<polars_crate_ $from_ver>]::frame::DataFrame) -> Self {
                    Self {
                        ffi: {
                            // Number of columns
                            let num_cols = df.width();

                            // Prepare ffi series vec
                            let mut ffi = Vec::with_capacity(num_cols);

                            // Get the columns from the df, as series
                            let series: Vec<[<polars_crate_ $from_ver>]::series::Series> = df
                                .take_columns()
                                .into_iter()
                                .map(|s| s.take_materialized_series())
                                .collect();

                            for s in series {
                                // Get arrow-type fields
                                let field = &s
                                    .field()
                                    .to_arrow([<polars_crate_ $from_ver>]::datatypes::CompatLevel::newest());

                                // Get name of column
                                let name = s.name().to_string();

                                // Get number of chunks in each column
                                let n_chunks = s.n_chunks();

                                // Prepare chunk vec
                                let mut ffi_chunk = Vec::with_capacity(num_cols);

                                for c in 0..n_chunks {

                                    // Get ffi array
                                    let ffi_array = [<polars_arrow_ $from_ver>]::ffi::export_array_to_c(
                                        s.to_arrow(c, [<polars_crate_ $from_ver>]::datatypes::CompatLevel::newest()),
                                    );

                                    // Get ffi field
                                    let ffi_field = [<polars_arrow_ $from_ver>]::ffi::export_field_to_c(field);

                                    // Convert ffi array from polars-arrow to this crate's version of ArrowArray
                                    let ffi_array = unsafe { transmute::<
                                        [<polars_arrow_ $from_ver>]::ffi::ArrowArray,
                                        ArrowArray,
                                    >(ffi_array)};

                                    // Convert ffi field from polars-arrow to this crate's version of ArrowField
                                    let ffi_field = unsafe{transmute::<
                                        [<polars_arrow_ $from_ver>]::ffi::ArrowSchema,
                                        ArrowSchema,
                                    >(ffi_field)};

                                    // Create series
                                    ffi_chunk.push((ffi_array, ffi_field));
                                }

                                ffi.push((name, ffi_chunk));
                            }

                            ffi
                        }
                    }
                }
            }
        }
    };
}
#[cfg(feature = "polars_0_44")]
polars_to_ffi!("0_44");

#[cfg(feature = "polars_0_45")]
polars_to_ffi!("0_45");

#[cfg(feature = "polars_0_46")]
polars_to_ffi!("0_46");

#[allow(unused_macros)]
macro_rules! polars_to_ffi {
    ($from_ver:literal) => {
        paste! {
            impl Interchange {
                #[doc = "Move Polars version `" $from_ver "` to the Arrow data interchange format."]
                pub fn [<from_polars_ $from_ver>](df: [<polars_crate_ $from_ver>]::frame::DataFrame) -> Self {
                    Self {
                        ffi: {
                            // Number of columns
                            let num_cols = df.width();

                            // Prepare ffi series vec
                            let mut ffi = Vec::with_capacity(num_cols);

                            // Get the series from the df
                            let series: Vec<[<polars_crate_ $from_ver>]::series::Series> = df.take_columns();

                            for s in series {
                                // Get arrow-type fields
                                let field = &s
                                    .field()
                                    .to_arrow([<polars_crate_ $from_ver>]::datatypes::CompatLevel::newest());

                                // Get name of column
                                let name = s.name().to_string();

                                // Get number of chunks in each column
                                let n_chunks = s.n_chunks();

                                // Prepare chunk vec
                                let mut ffi_chunk = Vec::with_capacity(num_cols);

                                for c in 0..n_chunks {

                                    // Get ffi array
                                    let ffi_array = [<polars_arrow_ $from_ver>]::ffi::export_array_to_c(
                                        s.to_arrow(c, [<polars_crate_ $from_ver>]::datatypes::CompatLevel::newest()),
                                    );

                                    // Get ffi field
                                    let ffi_field = [<polars_arrow_ $from_ver>]::ffi::export_field_to_c(field);

                                    // Convert ffi array from polars-arrow to this crate's version of ArrowArray
                                    let ffi_array = unsafe { transmute::<
                                        [<polars_arrow_ $from_ver>]::ffi::ArrowArray,
                                        ArrowArray,
                                    >(ffi_array)};

                                    // Convert ffi field from polars-arrow to this crate's version of ArrowField
                                    let ffi_field = unsafe{transmute::<
                                        [<polars_arrow_ $from_ver>]::ffi::ArrowSchema,
                                        ArrowSchema,
                                    >(ffi_field)};

                                    // Create series
                                    ffi_chunk.push((ffi_array, ffi_field));
                                }

                                ffi.push((name, ffi_chunk));
                            }

                            ffi
                        }
                    }
                }
            }
        }
    };
}

#[cfg(feature = "polars_0_42")]
polars_to_ffi!("0_42");

#[cfg(feature = "polars_0_43")]
polars_to_ffi!("0_43");

#[allow(unused_macros)]
macro_rules! polars_to_ffi {
    ($from_ver:literal) => {
        paste! {
            impl Interchange {
                #[doc = "Move Polars version `" $from_ver "` to the Arrow data interchange format."]
                pub fn [<from_polars_ $from_ver>](df: [<polars_crate_ $from_ver>]::frame::DataFrame) -> Self {
                    Self {
                        ffi: {
                            // Number of columns
                            let num_cols = df.width();

                            // Prepare ffi series vec
                            let mut ffi = Vec::with_capacity(num_cols);

                            // Get column names to remove them one by one
                            let names = df.get_column_names();

                            // Get the series from the df
                            let series: Vec<[<polars_crate_ $from_ver>]::series::Series> = df.select_series(names).unwrap();

                            for s in series {
                                // Get arrow-type fields
                                let field = &s
                                    .field()
                                    .to_arrow(false);

                                // Get name of column
                                let name = s.name().to_string();

                                // Get number of chunks in each column
                                let n_chunks = s.n_chunks();

                                // Prepare chunk vec
                                let mut ffi_chunk = Vec::with_capacity(num_cols);

                                for c in 0..n_chunks {

                                    // Get ffi array
                                    let ffi_array = [<polars_arrow_ $from_ver>]::ffi::export_array_to_c(
                                        s.to_arrow(c, false),
                                    );

                                    // Get ffi field
                                    let ffi_field = [<polars_arrow_ $from_ver>]::ffi::export_field_to_c(field);

                                    // Convert ffi array from polars-arrow to this crate's version of ArrowArray
                                    let ffi_array = unsafe { transmute::<
                                        [<polars_arrow_ $from_ver>]::ffi::ArrowArray,
                                        ArrowArray,
                                    >(ffi_array)};

                                    // Convert ffi field from polars-arrow to this crate's version of ArrowField
                                    let ffi_field = unsafe{transmute::<
                                        [<polars_arrow_ $from_ver>]::ffi::ArrowSchema,
                                        ArrowSchema,
                                    >(ffi_field)};

                                    // Create series
                                    ffi_chunk.push((ffi_array, ffi_field));
                                }

                                ffi.push((name, ffi_chunk));
                            }

                            ffi
                        }
                    }
                }
            }
        }
    };
}

#[cfg(feature = "polars_0_40")]
polars_to_ffi!("0_40");

#[cfg(feature = "polars_0_41")]
polars_to_ffi!("0_41");
