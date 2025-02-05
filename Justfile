test-all:
    cargo test --features src_polars_0_46 --features dst_polars_0_45 --test polars_to_polars
    # cargo test --features src_polars_0_43 --features dst_polars_0_44 --test polars_to_polars
    # cargo test --features src_polars_0_44 --features dst_polars_0_45 --test polars_to_polars
    # cargo test --features src_polars_0_45 --features dst_polars_0_46 --test polars_to_polars