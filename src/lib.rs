use std::mem::transmute;

#[cfg(feature = "src_polars_0_43")]
use {polars_0_43 as src_polars, polars_arrow_0_43 as src_polars_arrow};

#[cfg(feature = "dst_polars_0_43")]
use {polars_0_43 as dst_polars, polars_arrow_0_43 as dst_polars_arrow};

#[cfg(feature = "src_polars_0_44")]
use {polars_0_44 as src_polars, polars_arrow_0_44 as src_polars_arrow};

#[cfg(feature = "dst_polars_0_44")]
use {polars_0_44 as dst_polars, polars_arrow_0_44 as dst_polars_arrow};

#[cfg(feature = "src_polars_0_45")]
use {polars_0_45 as src_polars, polars_arrow_0_45 as src_polars_arrow};

#[cfg(feature = "dst_polars_0_45")]
use {polars_0_45 as dst_polars, polars_arrow_0_45 as dst_polars_arrow};

#[cfg(feature = "src_polars_0_46")]
use {polars_0_46 as src_polars, polars_arrow_0_46 as src_polars_arrow};

#[cfg(feature = "dst_polars_0_46")]
use {polars_0_46 as dst_polars, polars_arrow_0_46 as dst_polars_arrow};

pub fn polars_to_polars(src_df: src_polars::frame::DataFrame) -> dst_polars::frame::DataFrame {
    let src_ffi = polars_to_ffi(src_df);

    ffi_to_polars(src_ffi)
}

fn polars_to_ffi(
    src_df: src_polars::frame::DataFrame,
) -> Vec<(
    String,
    Vec<(
        src_polars_arrow::ffi::ArrowArray,
        src_polars_arrow::ffi::ArrowSchema,
    )>,
)> {
    // Number of columns
    let num_cols = src_df.width();

    // Prepare dst series vec
    let mut src_ffi = Vec::with_capacity(num_cols);

    // Vectors of src series
    #[cfg(any(
        feature = "src_polars_0_44",
        feature = "src_polars_0_45",
        feature = "src_polars_0_46"
    ))]
    let src_series: Vec<src_polars::series::Series> = src_df
        .take_columns()
        .into_iter()
        .map(|s| s.take_materialized_series())
        .collect();

    // Vectors of src series
    #[cfg(feature = "src_polars_0_43")]
    let src_series: Vec<src_polars::series::Series> = src_df.take_columns();

    for s in src_series {
        let field = &s
            .field()
            .to_arrow(src_polars::datatypes::CompatLevel::newest());
        let name = s.name().to_string();
        let n_chunks = s.n_chunks();

        let mut ffi_chunk = Vec::with_capacity(num_cols);

        for c in 0..n_chunks {
            // Get ffi data from src polars
            let ffi_array_out = src_polars_arrow::ffi::export_array_to_c(
                s.to_arrow(c, src_polars::datatypes::CompatLevel::newest()),
            );
            let ffi_field_out = src_polars_arrow::ffi::export_field_to_c(field);

            // Create series
            ffi_chunk.push((ffi_array_out, ffi_field_out));
        }

        src_ffi.push((name, ffi_chunk));
    }

    src_ffi
}

fn ffi_to_polars(
    src_ffi: Vec<(
        String,
        Vec<(
            src_polars_arrow::ffi::ArrowArray,
            src_polars_arrow::ffi::ArrowSchema,
        )>,
    )>,
) -> dst_polars::frame::DataFrame {
    let num_cols = src_ffi.len();
    let mut dst_series = Vec::with_capacity(num_cols);

    for s in src_ffi {
        let mut dst_chunks = Vec::with_capacity(num_cols);
        let name = s.0;
        let src_chunks = s.1;

        for c in src_chunks {
            // Import ffi data to dst polars
            let ffi_field_in = unsafe {
                dst_polars_arrow::ffi::import_field_from_c(transmute::<
                    &src_polars_arrow::ffi::ArrowSchema,
                    &dst_polars_arrow::ffi::ArrowSchema,
                >(&c.1))
            }
            .unwrap();
            let ffi_array_in =
                unsafe {
                    dst_polars_arrow::ffi::import_array_from_c(
                        transmute::<
                            src_polars_arrow::ffi::ArrowArray,
                            dst_polars_arrow::ffi::ArrowArray,
                        >(c.0),
                        ffi_field_in.dtype().clone(),
                    )
                }
                .unwrap();

            // Create series
            dst_chunks.push(ffi_array_in);
        }

        dst_series
            .push(dst_polars::series::Series::from_arrow_chunks(name.into(), dst_chunks).unwrap());
    }

    dst_polars::frame::DataFrame::from_iter(dst_series)
}
