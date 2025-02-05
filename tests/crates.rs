// use polars_0_45::prelude::*;

// use connectorx::prelude::*;
// use connectorx::sources::dummy::DummyTypeSystem;
// use connectorx::transports::DummyArrow2Transport;
// use df_interchange::polars_to_polars;
// use polars_0_46::prelude::*;

// #[test]
// fn connectorx_033_polars() {
//     // ConnectorX Dummy data
//     let schema = [
//         DummyTypeSystem::I64(true),
//         DummyTypeSystem::F64(true),
//         DummyTypeSystem::Bool(false),
//         DummyTypeSystem::String(true),
//         DummyTypeSystem::F64(false),
//     ];
//     let nrows = [4, 7];
//     let ncols = schema.len();
//     let queries: Vec<CXQuery> = nrows
//         .iter()
//         .map(|v| CXQuery::naked(format!("{},{}", v, ncols)))
//         .collect();
//     let mut destination = Arrow2Destination::new();

//     let dispatcher = Dispatcher::<_, _, DummyArrow2Transport>::new(
//         DummySource::new(&["a", "b", "c", "d", "e"], &schema),
//         &mut destination,
//         &queries,
//         None,
//     );
//     dispatcher.run().expect("run dispatcher");

//     // Get polars 0.32
//     let df = destination.polars().unwrap();

//     // Convert to polars 0.46
//     let df = polars_to_polars(df);

//     // Create polars 0.46 data
//     let expected = df!(
//         "a" => &[0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6],
//         "b" => &[0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
//         "c" => &[true, false, true, false, true, false, true, false, true, false, true],
//         "d" => &["0", "1", "2", "3", "0", "1", "2", "3", "4", "5", "6"],
//         "e" => &[0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
//     )
//     .unwrap();

//     // order of each batch is not guaranteed
//     let expected2 = df!(
//         "a" => &[0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3],
//         "b" => &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 1.0, 2.0, 3.0],
//         "c" => &[true, false, true, false, true, false, true, true, false, true, false],
//         "d" => &["0", "1", "2", "3", "4", "5", "6", "0", "1", "2", "3"],
//         "e" => &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 1.0, 2.0, 3.0]
//     )
//     .unwrap();

//     // Compare 0.46 (converted) to 0.46
//     assert!(df.frame_equal_missing(&expected) || df.frame_equal_missing(&expected2));
// }
