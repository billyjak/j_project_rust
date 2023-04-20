use serde_json::map::IntoIter;
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    






    let path: &Path = Path::new("src/resources/data.json");


    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    untyped_example(&contents);


}

fn untyped_example(anything: &str) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // Parse the string of data into serde_json::Value.
    let v: Vec<HashMap<String, Value>> = serde_json::from_str(&anything)?;
    // let mut iter = serde_json::map::IntoIter::into<IntoIter, Value>;
    //for each key in nouns, pick a key at random and print the roman


    for x in v.iter() {
       print!("{}", ); 
    }

    // Access parts of the data by indexing with square brackets.

    Ok(())

}