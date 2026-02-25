use df_interchange::{Interchange, InterchangeError};
use std::{
    fs::File,
    io::{Read, Write},
    time::SystemTime,
};

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
    feature = "polars_0_51",
    feature = "polars_0_52",
    feature = "polars_0_53",
    feature = "arrow_54",
    feature = "arrow_55",
    feature = "arrow_56",
    feature = "arrow_57",
    feature = "arrow_58",
))]
#[test]
pub fn test_large_data() -> Result<(), InterchangeError> {
    use polars_crate_0_53::prelude::{col, IntoLazy, SortMultipleOptions};

    let timer = SystemTime::now();

    let lf = load_data();

    let pre_interchange = lf
        .clone()
        .group_by([col("occupation_10a")])
        .agg([col("resident_id_m").count()])
        .sort(["occupation_10a"], SortMultipleOptions::default())
        .collect()
        .unwrap();

    println!("{pre_interchange}");

    let df = lf.collect().unwrap();

    let pre_rows = df.shape();

    println!("Collecting data: {:?}", timer.elapsed().unwrap().as_secs());

    let timer = SystemTime::now();

    let arrow_54 = Interchange::from_polars_0_53(df)?.to_arrow_54()?;
    let arrow_55 = Interchange::from_arrow_54(arrow_54)?.to_arrow_55()?;
    let arrow_56 = Interchange::from_arrow_55(arrow_55)?.to_arrow_56()?;
    let arrow_57 = Interchange::from_arrow_56(arrow_56)?.to_arrow_57()?;
    let arrow_58 = Interchange::from_arrow_57(arrow_57)?.to_arrow_58()?;

    let polars_0_40 = Interchange::from_arrow_58(arrow_58)?.to_polars_0_40()?;
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
    let polars_0_51 = Interchange::from_polars_0_50(polars_0_50)?.to_polars_0_51()?;
    let polars_0_52 = Interchange::from_polars_0_51(polars_0_51)?.to_polars_0_52()?;
    let polars_0_53 = Interchange::from_polars_0_52(polars_0_52)?.to_polars_0_53()?;

    println!(
        "Moving data between versions: {:?}",
        timer.elapsed().unwrap().as_secs()
    );

    let post_rows = polars_0_53.shape();

    // Print if it fails
    println!("{:?}", &pre_rows);
    println!("{:?}", &post_rows);

    assert!(pre_rows == post_rows);

    let lf = polars_0_53.lazy();

    let post_interchange = lf
        .group_by([col("occupation_10a")])
        .agg([col("resident_id_m").count()])
        .sort(["occupation_10a"], SortMultipleOptions::default())
        .collect()
        .unwrap();

    // Delete csv file
    let _ = std::fs::remove_file("./data/census.csv");

    // Print if it fails
    println!("{}", pre_interchange);
    println!("{}", post_interchange);

    assert!(pre_interchange.equals_missing(&post_interchange));

    Ok(())
}

fn load_data() -> polars_crate_0_53::prelude::LazyFrame {
    use polars_crate_0_53::prelude::LazyFileListReader;

    // Unzip
    let zip_file = File::open("./data/data.zip").unwrap();
    let mut csv_buf: Vec<u8> = Vec::new();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();
    let _ = archive
        .by_name("census.csv")
        .unwrap()
        .read_to_end(&mut csv_buf)
        .unwrap();
    let mut file = std::fs::File::create("./data/census.csv").unwrap();
    file.write_all(&csv_buf).unwrap();

    polars_crate_0_53::prelude::LazyCsvReader::new(polars_crate_0_53::prelude::PlRefPath::from(
        "./data/census.csv",
    ))
    .with_has_header(true)
    .finish()
    .unwrap()
}
