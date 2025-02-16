test-quick:
    cargo test --features "polars_0_45 polars_0_46" --test polars_to_polars
    cargo test --features "polars_0_45 arrow_54" --test polars_to_arrow --test arrow_to_polars
    cargo test --features "arrow_53 arrow_54" --test arrow_to_arrow

test-large:
    cargo test --features all -j1 --test large

test-all:
    cargo test --features all -j1

test-ci:
    cargo test --features all -j1 --test polars_to_arrow --test arrow_to_polars --test polars_to_polars --test arrow_to_arrow

doc:
    cargo doc --lib --open --features all

get-data:
    bash ./data/download.sh