test-quick:
    cargo test --features "polars_0_45 polars_0_52" --test polars_to_polars
    cargo test --features "polars_0_52 arrow_57" --test polars_to_arrow --test arrow_to_polars
    cargo test --features "arrow_55 arrow_57" --test arrow_to_arrow

test-large:
    cargo test --features all -j1 --test large

test-ci:
    cargo test --features "arrow_55 arrow_56 arrow_57 polars_0_49 polars_0_50 polars_0_51 polars_0_52" -j1 --test polars_to_arrow --test arrow_to_polars --test polars_to_polars --test arrow_to_arrow

test-all:
    cargo test --features all -j1

doc:
    cargo doc --lib --open --features all