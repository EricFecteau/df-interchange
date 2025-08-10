use df_interchange::{Interchange, InterchangeError};
use std::time::SystemTime;

#[cfg(all(
    feature = "polars_0_40",
    feature = "polars_0_41",
    feature = "polars_0_42",
    feature = "polars_0_43",
    feature = "polars_0_44",
    feature = "polars_0_45",
    feature = "polars_0_46",
    feature = "polars_0_47",
    feature = "polars_0_48",
    feature = "polars_0_49",
    feature = "polars_0_50",
    feature = "arrow_54",
    feature = "arrow_55",
    feature = "arrow_56",
))]
#[test]
pub fn test_large_data() -> Result<(), InterchangeError> {
    use polars_crate_0_50::prelude::{col, lit, IntoLazy, Series};

    let timer = SystemTime::now();

    let lf = load_data();

    let rows: Vec<u64> = (0..2_000_001).step_by(100_000).collect();
    let pre_rows = lf
        .clone()
        .with_row_index("index", None)
        .filter(col("index").is_in(lit(Series::from_iter(rows.clone())).implode(), false))
        .collect()
        .unwrap();

    let pre_weight = lf
        .clone()
        .select([col("FINALWT").sum().alias("weight_sum")])
        .collect()
        .unwrap();

    let df = lf.collect().unwrap();

    println!("Collecting data: {:?}", timer.elapsed().unwrap().as_secs());

    let timer = SystemTime::now();

    let arrow_54 = Interchange::from_polars_0_50(df)?.to_arrow_54()?;
    let arrow_55 = Interchange::from_arrow_54(arrow_54)?.to_arrow_55()?;
    let arrow_56 = Interchange::from_arrow_55(arrow_55)?.to_arrow_56()?;

    let polars_0_40 = Interchange::from_arrow_56(arrow_56)?.to_polars_0_40()?;
    let polars_0_41 = Interchange::from_polars_0_40(polars_0_40)?.to_polars_0_41()?;
    let polars_0_42 = Interchange::from_polars_0_41(polars_0_41)?.to_polars_0_42()?;
    let polars_0_43 = Interchange::from_polars_0_42(polars_0_42)?.to_polars_0_43()?;
    let polars_0_44 = Interchange::from_polars_0_43(polars_0_43)?.to_polars_0_44()?;
    let polars_0_45 = Interchange::from_polars_0_44(polars_0_44)?.to_polars_0_45()?;
    let polars_0_46 = Interchange::from_polars_0_45(polars_0_45)?.to_polars_0_46()?;
    let polars_0_47 = Interchange::from_polars_0_46(polars_0_46)?.to_polars_0_47()?;
    let polars_0_48 = Interchange::from_polars_0_47(polars_0_47)?.to_polars_0_48()?;
    let polars_0_49 = Interchange::from_polars_0_48(polars_0_48)?.to_polars_0_49()?;
    let polars_0_50 = Interchange::from_polars_0_49(polars_0_49)?.to_polars_0_50()?;

    println!(
        "Moving data between versions: {:?}",
        timer.elapsed().unwrap().as_secs()
    );

    let lf = polars_0_50.lazy();

    let post_rows = lf
        .clone()
        .with_row_index("index", None)
        .filter(col("index").is_in(lit(Series::from_iter(rows)).implode(), false))
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

fn load_data() -> polars_crate_0_50::prelude::LazyFrame {
    use polars_crate_0_50::prelude::LazyFileListReader;

    // Get schema
    let schema = polars_crate_0_50::prelude::LazyCsvReader::new(
        polars_crate_0_50::prelude::PlPath::from_str("./data/csv/pub0120.csv"),
    )
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
        let csv = path.unwrap().path().into_os_string().into_string().unwrap();

        let lf = polars_crate_0_50::prelude::LazyCsvReader::new(
            polars_crate_0_50::prelude::PlPath::from_string(csv),
        )
        .with_has_header(true)
        .with_schema(Some(schema.clone()))
        .finish()
        .unwrap();

        lf_vec.push(lf);
    }

    polars_crate_0_50::prelude::concat(lf_vec, polars_crate_0_50::prelude::UnionArgs::default())
        .unwrap()
}
