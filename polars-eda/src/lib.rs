/*
polars-eda utility functions
*/

use polars::prelude::*;
use std::fs;
use std::io::Cursor;

//read in a csv file
pub fn read_csv(path: &str, header: bool) -> Result<DataFrame, PolarsError> {
    let df = CsvReader::from_path(path).unwrap().has_header(header).finish().unwrap();
    Ok(df)
}

//read in a json file
pub fn load_json(path: &str) -> Result<DataFrame, PolarsError> {
    // Read json file to string
    let json_str= fs::read_to_string(path).expect("Unable to read JSON");
    let df = JsonReader::new(Cursor::new(json_str)).finish().unwrap();
    Ok(df)
}

//summarise dataframe
pub fn df_summary(df: DataFrame) {
    println!("Dataframe Summary...");
    println!("{:?}", df.head(Some(5)));
    println!("{:?}", df.schema());
    println!("{:?}", df.describe(None));
}
