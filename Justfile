test-quick:
    cargo test --features "polars_0_45 polars_0_51" --test polars_to_polars
    cargo test --features "polars_0_51 arrow_56" --test polars_to_arrow --test arrow_to_polars
    cargo test --features "arrow_55 arrow_56" --test arrow_to_arrow

test-large:
    cargo test --features all -j1 --test large

test-ci:
    cargo test --features "arrow_54 arrow_55 arrow_56 polars_0_46 polars_0_47 polars_0_48 polars_0_49 polars_0_50 polars_0_51" -j1 --test polars_to_arrow --test arrow_to_polars --test polars_to_polars --test arrow_to_arrow

test-all:
    cargo test --features all -j1

doc:
    cargo doc --lib --open --features all

get-data:
    bash ./data/download.sh