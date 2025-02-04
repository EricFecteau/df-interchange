

```
eric@Arch ~/R/df-interchange (main)> just test-all
cargo test --features src_polars_0_32 --features dst_polars_0_46
   Compiling df-interchange v0.1.0 (/home/eric/Rust/df-interchange)
error[E0308]: mismatched types
   --> tests/crates.rs:31:25
    |
31  |     let df: DataFrame = destination.polars().unwrap();
    |             ---------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `polars::prelude::DataFrame`, found `polars_core::frame::DataFrame`
    |             |
    |             expected due to this
    |
    = note: `polars_core::frame::DataFrame` and `polars::prelude::DataFrame` have similar names, but are actually distinct types
```

# TODO 
[ ] Make test come from code instead of Parquet