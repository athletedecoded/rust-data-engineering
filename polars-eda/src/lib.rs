/*
polars-eda utility functions
*/

use polars::prelude::*;

//read in a csv file
pub fn read_csv(path: &str, header: bool) -> Result<DataFrame, PolarsError> {
    let df = CsvReader::from_path(path).unwrap().has_header(header).finish().unwrap();
    Ok(df)
}

//read in a json file
// pub fn load_json(path: &str) -> Result<DataFrame, PolarsError> {
//     let df = JsonReader::from_path(path).unwrap().finish().unwrap();
//     Ok(df)
// }
