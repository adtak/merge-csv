extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::iter::FromIterator;
use std::process;

#[derive(Debug, Deserialize, Hash, PartialEq, Serialize)]
// #[serde(rename_all = "PascalCase")]
struct CsvRecord {
    tweet_id: u64,
    tweet: String,
    raw_tweet: String,
    reply_tweet_id: u64,
    reply_tweet: String,
    raw_reply_tweet: String,
}

impl Eq for CsvRecord {}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_args()?;
    let mut csv_records = vec!();
    for entry in fs::read_dir(file_path)? {
        let path = entry.unwrap().path();
        if "csv" == path.extension().unwrap() {
            let mut rdr = csv::Reader::from_path(path)?;
            for result in rdr.deserialize() {
                let record: CsvRecord = result?;
                csv_records.push(record);
            }
        }
    }
    let unique_records: Vec<_> = HashSet::<CsvRecord>::from_iter(csv_records).into_iter().collect();

    println!("Result len: {}", unique_records.len());

    let mut wtr = csv::Writer::from_path("./merged.csv")?;
    for record in unique_records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
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
//         mut tweet: Vec<Text>,
//         mut replay: Vec<Text>
// ) -> Result<Vec<Text>, Box<dyn Error>> {
    // A
    // tweet.append(&mut replay);
    // Ok(HashSet::<Text>::from_iter(tweet).into_iter().collect())

    // B
    // let th: HashSet<Text> = HashSet::from_iter(tweet);
    // let rh: HashSet<Text> = HashSet::from_iter(replay);
    // Ok(
    //     th.union(&rh)
    //         .into_iter()
    //         .map(|x| (*x).clone())
    //         .collect()
    // )
// }
