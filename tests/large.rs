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
    use polars_crate_0_46::prelude::{col, lit, IntoLazy};

    let lf = load_data();

    // let lf = lf.with_column((col("SEX") * lit(2)).alias("test2"));

    // let verify_order = lf
    //     .clone()
    //     .with_column(col("index").shift(lit(1)).alias("index_shift"))
    //     .select([col("index"), col("index_shift")])
    //     .filter((col("index") - col("index_shift")).eq_missing(lit(1)).not())
    //     .filter(col("index").eq(lit(0)).not());

    // assert!(0 == verify_order.collect().unwrap().shape().0);

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

    // let verify_order = polars_0_46
    //     .lazy()
    //     .with_column(col("index").shift(lit(1)).alias("index_shift"))
    //     .select([col("index"), col("index_shift")])
    //     .filter((col("index") - col("index_shift")).eq_missing(lit(1)).not())
    //     .filter(col("index").eq(lit(0)).not());

    // assert!(0 == verify_order.collect().unwrap().shape().0);

    // let converted_df = Interchange::[<from_arrow_ $from_ver>](src_df.clone())?.[<to_polars_ $to_ver>]()?;
    // let dst_df = setup::[<polars_data_ $to_ver>]();

    // // Print if it fails
    // println!("{:?}", &src_df);
    // println!("{:?}", &converted_df);
    // println!("{:?}", &dst_df);

    // assert!(dst_df.equals_missing(&converted_df));

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
    // .with_row_index("index", None)
}
