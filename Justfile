test-quick:
    cargo test --features "polars_0_45 polars_0_46" --test polars_to_polars
    cargo test --features "polars_0_45 arrow_54" --test polars_to_arrow --test arrow_to_polars
    cargo test --features "arrow_53 arrow_54" --test arrow_to_arrow

test-all:
    cargo test --features all -j1
    
doc:
    cargo doc --lib --open --features all