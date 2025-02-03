use std::mem::transmute;

use polars_0_45::datatypes::CompatLevel as src_CompatLevel;
use polars_0_45::frame::DataFrame as src_DataFrame;
use polars_0_45::series::Series as src_Series;
use polars_arrow_0_45 as src_polars_arrow;
use polars_arrow_0_45::ffi::{export_array_to_c, export_field_to_c};

use polars_0_46::frame::DataFrame as dst_DataFrame;
use polars_0_46::series::Series as dst_Series;
use polars_arrow_0_46 as dst_polars_arrow;
use polars_arrow_0_46::ffi::{import_array_from_c, import_field_from_c};

pub fn polars_to_polars(src_df: src_DataFrame) -> dst_DataFrame {
    // NUmber of columns
    let num_cols = src_df.width();

    // Prepare dst series vec
    let mut dst_series = Vec::with_capacity(num_cols);

    // Vectors of src series
    let src_series: Vec<src_Series> = src_df
        .take_columns()
        .into_iter()
        .map(|s| s.take_materialized_series())
        .collect();

    for s in src_series {
        let field = &s.field().to_arrow(src_CompatLevel::newest());
        let name = &s.name().to_string();
        let src_chunks = s.into_chunks();

        let mut dst_chunks = Vec::with_capacity(num_cols);

        for c in src_chunks {
            // Get ffi data from src polars
            let ffi_array_out = export_array_to_c(c);
            let ffi_field_out = export_field_to_c(field);

            // Import ffi data to dst polars

            let ffi_field_in = unsafe {
                import_field_from_c(transmute::<
                    &src_polars_arrow::ffi::ArrowSchema,
                    &dst_polars_arrow::ffi::ArrowSchema,
                >(&ffi_field_out))
            }
            .unwrap();
            let ffi_array_in =
                unsafe {
                    import_array_from_c(
                        transmute::<
                            src_polars_arrow::ffi::ArrowArray,
                            dst_polars_arrow::ffi::ArrowArray,
                        >(ffi_array_out),
                        ffi_field_in.dtype().clone(),
                    )
                }
                .unwrap();

            // Create series
            dst_chunks.push(ffi_array_in);
        }

        dst_series.push(dst_Series::from_arrow_chunks(name.into(), dst_chunks).unwrap());
    }

    // Create DataFrame from the dst Series
    dst_DataFrame::from_iter(dst_series)
}
