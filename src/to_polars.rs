use paste::paste;
use std::mem::transmute;

use crate::{error::InterchangeError, ArrowArray, ArrowSchema, Interchange};

macro_rules! ffi_to_polars {
    ($to_ver:literal) => {
        paste! {
            impl Interchange {
                #[doc = "Move Arrow data interchange format to Polars version `" $to_ver "`."]
                pub fn [<to_polars_ $to_ver>](self) -> Result<[<polars_crate_ $to_ver>]::frame::DataFrame, InterchangeError> {

                    // Prepare series vec
                    let num_cols = self.ffi.len();
                    let mut series = Vec::with_capacity(num_cols);

                    // For columns in the ffi
                    for s in self.ffi {

                        // Prepare chunks vec
                        let num_chunks = s.1.len();
                        let mut chunks = Vec::with_capacity(num_chunks);

                        // Get name of column
                        let name = s.0;

                        // For chunk in column
                        for c in s.1 {

                            // Convert ffi array from this crate's version of ArrowArray to polars-arrow
                            let ffi_array = unsafe { transmute::<ArrowArray,  [<polars_arrow_ $to_ver>]::ffi::ArrowArray>(c.0) };

                            // Convert ffi field from this crate's version of ArrowField to polars-arrow
                            let ffi_field = unsafe { transmute::<ArrowSchema,  [<polars_arrow_ $to_ver>]::ffi::ArrowSchema>(c.1) };

                            // Convert ffi to field
                            let field = unsafe {
                                [<polars_arrow_ $to_ver>]::ffi::import_field_from_c(&ffi_field)
                            }?;

                            // Convert ffi to chunk
                            let array = unsafe {
                                [<polars_arrow_ $to_ver>]::ffi::import_array_from_c(ffi_array, field.dtype().clone(),
                                )
                            }?;

                            // Add the chunks to the vec of chunks
                            chunks.push(array);
                        }

                        // Create series out of vec of chunks
                        series.push(
                            [<polars_crate_ $to_ver>]::series::Series::from_arrow_chunks(
                                name.into(),
                                chunks,
                            )?,
                        );
                    }

                    // Create DataFrame out of Vec of series
                    Ok([<polars_crate_ $to_ver>]::frame::DataFrame::from_iter(series))
                }
            }
        }
    };
}

#[cfg(feature = "polars_0_43")]
ffi_to_polars!("0_43");

#[cfg(feature = "polars_0_44")]
ffi_to_polars!("0_44");

#[cfg(feature = "polars_0_45")]
ffi_to_polars!("0_45");

#[cfg(feature = "polars_0_46")]
ffi_to_polars!("0_46");

#[cfg(feature = "polars_0_47")]
ffi_to_polars!("0_47");

#[cfg(feature = "polars_0_48")]
ffi_to_polars!("0_48");

#[cfg(feature = "polars_0_49")]
ffi_to_polars!("0_49");

#[cfg(feature = "polars_0_50")]
ffi_to_polars!("0_50");

macro_rules! ffi_to_polars {
    ($to_ver:literal) => {
        paste! {
            impl Interchange {
                #[doc = "Move Arrow data interchange format to Polars version `" $to_ver "`."]
                pub fn [<to_polars_ $to_ver>](self) -> Result<[<polars_crate_ $to_ver>]::frame::DataFrame, InterchangeError> {

                    // Prepare series vec
                    let num_cols = self.ffi.len();
                    let mut series = Vec::with_capacity(num_cols);

                    // For columns in the ffi
                    for s in self.ffi {

                        // Prepare chunks vec
                        let num_chunks = s.1.len();
                        let mut chunks = Vec::with_capacity(num_chunks);

                        // Get name of column
                        let name = s.0;

                        // For chunk in column
                        for c in s.1 {

                            // Convert ffi array from this crate's version of ArrowArray to polars-arrow
                            let ffi_array = unsafe { transmute::<ArrowArray,  [<polars_arrow_ $to_ver>]::ffi::ArrowArray>(c.0) };

                            // Convert ffi field from this crate's version of ArrowField to polars-arrow
                            let ffi_field = unsafe { transmute::<ArrowSchema,  [<polars_arrow_ $to_ver>]::ffi::ArrowSchema>(c.1) };

                            // Convert ffi to field
                            let field = unsafe {
                                [<polars_arrow_ $to_ver>]::ffi::import_field_from_c(&ffi_field)
                            }?;

                            // Convert ffi to chunk
                            let array = unsafe {
                                [<polars_arrow_ $to_ver>]::ffi::import_array_from_c(ffi_array, field.data_type().clone(),
                                )
                            }?;

                            // Add the chunks to the vec of chunks
                            chunks.push(array);
                        }

                        // Create series out of vec of chunks
                        series.push(
                            [<polars_crate_ $to_ver>]::series::Series::from_arrow_chunks(
                                &name,
                                chunks,
                            )?,
                        );
                    }

                    // Create DataFrame out of Vec of series
                    Ok([<polars_crate_ $to_ver>]::frame::DataFrame::from_iter(series))
                }
            }
        }
    };
}

#[cfg(feature = "polars_0_41")]
ffi_to_polars!("0_41");

#[cfg(feature = "polars_0_42")]
ffi_to_polars!("0_42");

macro_rules! ffi_to_polars {
    ($to_ver:literal) => {
        paste! {
            impl Interchange {
                #[doc = "Move Arrow data interchange format to Polars version `" $to_ver "`."]
                pub fn [<to_polars_ $to_ver>](self) -> Result<[<polars_crate_ $to_ver>]::frame::DataFrame, InterchangeError> {

                    // Prepare series vec
                    let num_cols = self.ffi.len();
                    let mut series = Vec::with_capacity(num_cols);

                    // For columns in the ffi
                    for s in self.ffi {

                        // Prepare chunks vec
                        let num_chunks = s.1.len();
                        let mut chunks = Vec::with_capacity(num_chunks);

                        // Get name of column
                        let name = s.0;

                        // For chunk in column
                        for c in s.1 {

                            // Convert ffi array from this crate's version of ArrowArray to polars-arrow
                            let ffi_array = unsafe { transmute::<ArrowArray,  [<polars_arrow_ $to_ver>]::ffi::ArrowArray>(c.0) };

                            // Convert ffi field from this crate's version of ArrowField to polars-arrow
                            let ffi_field = unsafe { transmute::<ArrowSchema,  [<polars_arrow_ $to_ver>]::ffi::ArrowSchema>(c.1) };

                            // Convert ffi to field
                            let field = unsafe {
                                [<polars_arrow_ $to_ver>]::ffi::import_field_from_c(&ffi_field)
                            }?;

                            // Convert ffi to chunk
                            let array = unsafe {
                                [<polars_arrow_ $to_ver>]::ffi::import_array_from_c(ffi_array, field.data_type().clone(),
                                )
                            }?;

                            let series_chunk = [<polars_crate_ $to_ver>]::series::Series::from_arrow(
                                &name,
                                array,
                            )?;

                            // Add the chunks to the vec of chunks
                            chunks.push(series_chunk);
                        }

                        // Create one series from multiple series chunks
                        let mut chunk_iter = chunks.into_iter();
                        let mut full_series = chunk_iter.next().unwrap();
                        for c in chunk_iter {
                            full_series.append(&c)?;
                        }

                        series.push(
                            full_series,
                        );
                    }

                    // Create DataFrame out of Vec of series
                    Ok([<polars_crate_ $to_ver>]::frame::DataFrame::from_iter(series))
                }
            }
        }
    };
}

#[cfg(feature = "polars_0_40")]
ffi_to_polars!("0_40");
