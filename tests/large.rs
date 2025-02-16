use df_interchange::{Interchange, InterchangeError};

#[cfg(all(
    feature = "polars_0_40",
    feature = "polars_0_41",
    feature = "polars_0_42",
    feature = "polars_0_43",
    feature = "polars_0_44",
    feature = "polars_0_45",
    feature = "polars_0_46",
    feature = "arrow_50",
    feature = "arrow_51",
    feature = "arrow_52",
    feature = "arrow_53",
    feature = "arrow_54",
))]
#[test]
pub fn test_large_data() -> Result<(), InterchangeError> {
    use polars_crate_0_46::prelude::{col, lit, IntoLazy, Series};

    let lf = load_data();

    let rows: Vec<u64> = (0..2_000_001).step_by(100_000).collect();
    let pre_rows = lf
        .clone()
        .with_row_index("index", None)
        .filter(col("index").is_in(lit(Series::from_iter(rows.clone()))))
        .collect()
        .unwrap();

    let pre_weight = lf
        .clone()
        .select([col("FINALWT").sum().alias("weight_sum")])
        .collect()
        .unwrap();

    let df = lf.collect().unwrap();

    let arrow_50 = Interchange::from_polars_0_46(df)?.to_arrow_50()?;
    let arrow_51 = Interchange::from_arrow_50(arrow_50)?.to_arrow_51()?;
    let arrow_52 = Interchange::from_arrow_51(arrow_51)?.to_arrow_52()?;
    let arrow_53 = Interchange::from_arrow_52(arrow_52)?.to_arrow_53()?;
    let arrow_54 = Interchange::from_arrow_53(arrow_53)?.to_arrow_54()?;

    let polars_0_40 = Interchange::from_arrow_54(arrow_54)?.to_polars_0_40()?;
    let polars_0_41 = Interchange::from_polars_0_40(polars_0_40)?.to_polars_0_41()?;
    let polars_0_42 = Interchange::from_polars_0_41(polars_0_41)?.to_polars_0_42()?;
    let polars_0_43 = Interchange::from_polars_0_42(polars_0_42)?.to_polars_0_43()?;
    let polars_0_44 = Interchange::from_polars_0_43(polars_0_43)?.to_polars_0_44()?;
    let polars_0_45 = Interchange::from_polars_0_44(polars_0_44)?.to_polars_0_45()?;
    let polars_0_46 = Interchange::from_polars_0_45(polars_0_45)?.to_polars_0_46()?;

    let lf = polars_0_46.lazy();

    let post_rows = lf
        .clone()
        .with_row_index("index", None)
        .filter(col("index").is_in(lit(Series::from_iter(rows))))
        .collect()
        .unwrap();

    // Print if it fails
    println!("{}", &pre_rows);
    println!("{}", &post_rows);

    assert!(pre_rows.equals_missing(&post_rows));

    let post_weight = lf
        .lazy()
        .clone()
        .select([col("FINALWT").sum().alias("weight_sum")])
        .collect()
        .unwrap();

    // Print if it fails
    println!("{}", &pre_weight);
    println!("{}", &post_weight);

    assert!(pre_weight.equals_missing(&post_weight));

    Ok(())
}

fn load_data() -> polars_crate_0_46::prelude::LazyFrame {
    use polars_crate_0_46::prelude::LazyFileListReader;

    // Get schema
    let schema = polars_crate_0_46::prelude::LazyCsvReader::new("./data/csv/pub0106.csv")
        .with_has_header(true)
        .with_infer_schema_length(None)
        .finish()
        .unwrap()
        .collect_schema()
        .unwrap();

    // Get all files in folder (all CSVs)
    let paths = std::fs::read_dir("./data/csv").unwrap();

    let mut lf_vec = vec![];

    for path in paths {
        let csv = path.unwrap().path();

        let lf = polars_crate_0_46::prelude::LazyCsvReader::new(csv)
            .with_has_header(true)
            .with_schema(Some(schema.clone()))
            .finish()
            .unwrap();

        lf_vec.push(lf);
    }

    polars_crate_0_46::prelude::concat(lf_vec, polars_crate_0_46::prelude::UnionArgs::default())
        .unwrap()
}
