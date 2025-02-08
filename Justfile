test-all:
    cargo test --features all_polars --test polars_to_polars
    cargo test --features "polars_0_45 polars_0_46" --test polars_to_polars