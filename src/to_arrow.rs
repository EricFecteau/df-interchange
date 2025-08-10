use paste::paste;
use std::mem::transmute;

use crate::{error::InterchangeError, ArrowArray, ArrowSchema, Interchange};

macro_rules! ffi_to_arrow {
    ($to_ver:literal) => {
        paste! {
            impl Interchange {
                #[doc = "Move Arrow data interchange format to Arrow version `" $to_ver "`."]
                pub fn [<to_arrow_ $to_ver>](mut self) -> Result<Vec<[<arrow_crate_ $to_ver>]::record_batch::RecordBatch>, InterchangeError> {

                    if !self.chunks_aligned {
                        return Err(InterchangeError::ChunksNotAligned);
                    }

                    // Get number of columns
                    let num_cols = self.ffi.len();

                    // Get number of chunks
                    let num_chunks = self.ffi[0].1.len();

                    // Get batch vector ready
                    let mut batches = Vec::with_capacity(num_chunks);

                    // For columns in the ffi
                    for _ in 0..num_chunks {

                        // Create arrays for the batch
                        let mut arrays = Vec::with_capacity(num_cols);
                        let mut fields = Vec::with_capacity(num_cols);

                        for col_num in 0..num_cols {

                            // Get the chunk
                            let chunk = self.ffi[col_num].1.pop().unwrap();

                            // Convert ffi array from this crate's version of ArrowArray to arrow-rs
                            let ffi_array = unsafe { transmute::<ArrowArray,  [<arrow_crate_ $to_ver>]::ffi::FFI_ArrowArray>(chunk.0) };

                            // Convert ffi field from this crate's version of ArrowField to arrow-rs
                            let ffi_schema = unsafe { transmute::<ArrowSchema,  [<arrow_crate_ $to_ver>]::ffi::FFI_ArrowSchema>(chunk.1) };

                            // Import into arrow-rs
                            let from_ffi = unsafe { [<arrow_crate_ $to_ver>]::ffi::from_ffi(ffi_array, &ffi_schema) }?;

                            // Make an array out of it
                            let array_ref = [<arrow_crate_ $to_ver>]::array::make_array(from_ffi);

                            // Get the field for the batch schema
                            let field = std::convert::TryInto::<[<arrow_crate_ $to_ver>]::datatypes::Field>::try_into(&ffi_schema)?;

                            arrays.push(array_ref);
                            fields.push(std::sync::Arc::new(field));
                        }

                        // Create batch record from array and schema
                        let schema = [<arrow_crate_ $to_ver>]::datatypes::Schema::new(fields);
                        let record_batch = [<arrow_crate_ $to_ver>]::record_batch::RecordBatch::try_new(std::sync::Arc::new(schema), arrays)?;

                        batches.push(record_batch)

                    }

                    // Reverse the batches
                    let batches = batches.into_iter().rev().collect();

                    Ok(batches)
                }
            }
        }
    };
}

#[cfg(feature = "arrow_54")]
ffi_to_arrow!("54");

#[cfg(feature = "arrow_55")]
ffi_to_arrow!("55");

#[cfg(feature = "arrow_56")]
ffi_to_arrow!("56");
