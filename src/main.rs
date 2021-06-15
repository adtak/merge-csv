extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
struct CsvRecord {
    tweet_id: u64,
    tweet: String,
    raw_tweet: String,
    reply_tweet_id: u64,
    reply_tweet: String,
    raw_reply_tweet: String,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_args()?;
    let mut rdr = csv::Reader::from_path(file_path)?;

    let mut csv_records = vec!();
    for result in rdr.deserialize() {
        let record: CsvRecord = result?;
        csv_records.push(record);
    }
    println!("{}", csv_records.len());
    Ok(())
}

fn get_first_args() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

// use std::collections::HashSet;
// use std::iter::FromIterator;

// #[derive(PartialEq, Hash, Clone)]
// struct Text {
//     pub id: u32,
//     pub tweet: String,
// }

// impl Eq for Text {}

// fn merge_conversation(
//         tweet: Vec<Text>,
//         replay: Vec<Text>
// ) -> Result<Vec<Text>, Box<dyn Error>> {
//     let th: HashSet<Text> = HashSet::from_iter(tweet);
//     let rh: HashSet<Text> = HashSet::from_iter(replay);

//     Ok(
//         th.union(&rh)
//             .into_iter()
//             .map(|x| (*x).clone())
//             .collect()
//     )
// }
