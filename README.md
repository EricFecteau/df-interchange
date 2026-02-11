# DataFrame Interchange

[![crates.io](https://img.shields.io/crates/v/df-interchange)](https://crates.io/crates/df-interchange) [![docs.rs](https://img.shields.io/docsrs/df-interchange)](https://docs.rs/df-interchange/latest/df_interchange/index.html)

This crate allows for seamless interoperability between any version of [Polars (>=0.40)](https://docs.rs/polars/latest/polars/) and any version of [Arrow (>=54)](https://docs.rs/arrow/latest/arrow/), including between versions of the same crate (e.g. `Polars 0.40` to `Polars 0.46`), using the [Arrow C Data Interchange](https://arrow.apache.org/docs/format/CDataInterface.html) format.

Supported versions:
* Arrow: "54", "55", "56", "57"
* Polars: "0.40", "0.41", "0.42", "0.43", "0.44", "0.45", "0.46", "0.47", "0.48", "0.49", "0.50", "0.51", "0.52", "0.53"

## Polars and Arrow Rust ecosystem

Since both `Polars` (pre-1.0) and `Arrow` have SemVer breaking API updates in Rust every few months, the Rust ecosystem that depend on these crates update at a different rates and are consistently incompatible with each other (e.g. one crate outputs `Polars 0.45` and another crate takes `Polars 0.43` as input). For crates who take these as input or provide these as output, updating should be considered an API break, and require a major bump in version. This has a cascading effect over the whole ecosystem.

For example, attempting to pass `Polars 0.45` to a crate that uses `Polars 0.43`, or vice versa, will give a [error[E0308]: mismatched types](https://doc.rust-lang.org/error_codes/E0308.html) error with the note "'DataFrame' and 'DataFrame' have similar names, but are actually distinct types". 

This crate is meant to solve the interoperability issue and prevent the need for the entirety of the ecosystem to update at the same speed.

## Usage

Enable the correct feature (e.g. `Polars 0.43`, `Polars 0.46` and `Arrow 54`):

```toml
[dependencies]
polars = "0.43"
arrow = "54"
df-interchange = { version = "0.2", features = ["polars_0_43", "polars_0_46", "arrow_54"] }
```

Then use the `from_polars_0_43` & `from_arrow_54` and `to_polars_0_46` implementation of `Interchange` to change types:

```Rust
use std::sync::Arc; 
use arrow::{array::{ArrayRef, Int32Array, Int64Array}, record_batch::RecordBatch}; // Arrow 54
use polars::prelude::*; // Polars 0.43
use df_interchange::Interchange;

// Create Polars 0.43 data
let polars_0_43 = DataFrame::new(vec![
    Series::new("test_i32".into(), [-1i32, 0, 1]),
    Series::new("test_i64".into(), [-1i64, 0, 1]),
])
.unwrap();

// Create arrow 54 data
let arrow_54: Vec<_> = vec![RecordBatch::try_from_iter(vec![
    ("test_i32", Arc::new(Int32Array::from(vec![-1i32, 0, 1])) as ArrayRef),
    ("test_i64", Arc::new(Int64Array::from(vec![-1i64, 0, 1])) as ArrayRef),
])
.unwrap()];

// Convert Polars 0.43 to Polars 0.46
let df_polars = Interchange::from_polars_0_43(polars_0_43)?.to_polars_0_46()?;

// Convert Arrow 54 to Polars 0.46
let df_arrow = Interchange::from_arrow_54(arrow_54)?.to_polars_0_46()?;

// Compare the two DataFrames (not possible prior to conversion to Polars 0.46)
assert!(df_polars.equals_missing(&df_arrow));

```

## Technical info

### Features

Since Rust features are [additive](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification), you can enable features on Arrow or Polars crates by adding them to your own `Cargo.toml`.

For example, you can enable the `lazy` feature on the Polars version you receive from `df-interchange`.

```toml
[dependencies]
polars = { version = "0.46", features = ["lazy"] }
polars_old = { package = "polars", version = "0.45", features = ["lazy"] }
df-interchange = { path = "/home/eric/Rust/df-interchange", version = "0.1.0", features = ["polars_0_45", "polars_0_46"] }
```

To use this, since the `.lazy()` uses the `IntoLazy` trait for `DataFrame`, you have to [disambiguate the trait](https://doc.rust-lang.org/rust-by-example/trait/disambiguating.html) with `<polars_old::prelude::DataFrame as polars_old::prelude::IntoLazy>::lazy(df)`:

```Rust
use df_interchange::Interchange;
use polars::prelude::*;

let df_0_46 = DataFrame::new(vec![
    Column::new("test_i32".into(), [1i32, 2, 3, 4]),
    Column::new("test_i64".into(), [1i64, 2, 3, 4]),
])
.unwrap()
.lazy();

let df_0_45 = Interchange::from_polars_0_46(df_0_46.collect().unwrap())?.to_polars_0_45()?;

let lf = <polars_old::prelude::DataFrame as polars_old::prelude::IntoLazy>::lazy(df_0_45);
```

During conversion, you may encounter errors based on data type conversions enabled by features. For example, if you convert a column of `i8` from a `Polars 0.46` that enables the `dtype-i8` feature, to `Polars 0.43` that does not enable `dtype-i8`, you will get a `Error(ComputeError(ErrString("cannot create series from Int8")))`. You can enable this feature on both versions of the crate to solve the issue.

```toml
polars = { version = "0.46", features = ["dtype-i8"] }
polars_0_43 = { package = "polars", version = "0.43", features = ["dtype-i8"] }
df-interchange = { version = "0.1.0", features = ["polars_0_43", "polars_0_46"] }
```

```Rust
use polars::prelude::*; // Polars 0.46
use df_interchange::Interchange; 

let df_0_46 = DataFrame::new(vec![
    Column::new("test_i8".into(), [1i8, 2, 3, 4]),
    Column::new("test_i64".into(), [1i64, 2, 3, 4]),
])
.unwrap();

let df_0_43 = Interchange::from_polars_0_46(df_0_46)?.to_polars_0_43()?;
```
