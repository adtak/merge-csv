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
use std::path::Path;
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
            csv_records.append(&mut read_csv(path)?);
        }
    }
    let unique_records: Vec<_> = HashSet::<CsvRecord>::from_iter(csv_records).into_iter().collect();
    println!("Result len: {}", unique_records.len());
    write_csv(unique_records, "./merged.csv")?;

    Ok(())
}

fn get_first_args() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn read_csv<P>(path: P) -> Result<Vec<CsvRecord>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut records = vec!();
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: CsvRecord = result?;
        records.push(record);
    }
    Ok(records)
}

fn write_csv<T, I, P>(records: T, path: P) -> Result<(), Box<dyn Error>>
where 
    T: IntoIterator<Item=I>,
    I: serde::Serialize,
    P: AsRef<Path>
{
    let mut wtr = csv::Writer::from_path(path)?;
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

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

#[cfg(test)]
mod tests {

}