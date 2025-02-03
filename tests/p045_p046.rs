use df_interchange::polars_to_polars;

#[test]
fn p045_to_p046() {
    let old_df = polars_0_45::df! [
        "names" => ["a", "b", "c"],
        "values" => [1, 2, 3],
        "values_nulls" => [Some(1), None, Some(3)]
    ]
    .unwrap();
    let new_df = polars_to_polars(old_df);
    println!("{}", new_df);
    old_df.equals_missing(&new_df);

    panic!();
}
