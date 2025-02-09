use paste::paste;
use std::mem::transmute;

use crate::{error::InterchangeError, ArrowArray, ArrowSchema, Interchange};

macro_rules! arrow_to_ffi {
    ($from_ver:literal) => {
        paste! {
            impl Interchange {
                #[doc = "Move Arrow version `" $from_ver "` to the Arrow data interchange format."]
                pub fn [<from_arrow_ $from_ver>](df: Vec<[<arrow_crate_ $from_ver>]::record_batch::RecordBatch>) -> Result<Self, InterchangeError> {
                    Ok(Self {
                        ffi: {
                            // Number of chunks
                            let num_chunks = df.len();

                            // Number of columns
                            let num_cols = df[0].num_columns();

                            // Prepare ffi series vec
                            let mut ffi: Vec<(String, Vec<(ArrowArray, ArrowSchema)>)> = Vec::with_capacity(num_cols);

                            for c in 0..num_cols {
                                ffi.push((df[0].schema().field(c).name().clone(), Vec::with_capacity(num_chunks)));
                            }

                            for chunk in df {

                                for (col_num, col) in chunk.columns().iter().enumerate() {

                                    // Convert to ArrayData
                                    let array = col.to_data();

                                    // Convert to ffi
                                    let ffi_schema = [<arrow_crate_ $from_ver>]::ffi::FFI_ArrowSchema::try_from(chunk.schema().field(col_num))?;
                                    let (ffi_array, _) = [<arrow_crate_ $from_ver>]::ffi::to_ffi(&array)?;

                                    // Convert ffi array from arrow-rs to this crate's version of ArrowArray
                                    let ffi_array = unsafe { transmute::<[<arrow_crate_ $from_ver>]::ffi::FFI_ArrowArray, ArrowArray>(ffi_array) };

                                    // Convert ffi schema from arrow-rs to this crate's version of ArrowSchema
                                    let ffi_schema = unsafe { transmute::<[<arrow_crate_ $from_ver>]::ffi::FFI_ArrowSchema, ArrowSchema>(ffi_schema) };

                                    ffi[col_num].1.push((ffi_array, ffi_schema));
                                }
                            }

                            ffi
                        }
                    })
                }
            }
        }
    };
}

#[cfg(feature = "arrow_50")]
arrow_to_ffi!("50");

#[cfg(feature = "arrow_51")]
arrow_to_ffi!("51");

#[cfg(feature = "arrow_52")]
arrow_to_ffi!("52");

#[cfg(feature = "arrow_53")]
arrow_to_ffi!("53");

#[cfg(feature = "arrow_54")]
arrow_to_ffi!("54");
