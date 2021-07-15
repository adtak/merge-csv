use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;
use std::path::Path;

#[derive(Debug, Deserialize, Hash, PartialEq, Serialize)]
struct CsvRecord {
    tweet_id: u64,
    tweet: String,
    raw_tweet: String,
    reply_tweet_id: u64,
    reply_tweet: String,
    raw_reply_tweet: String,
}

impl Eq for CsvRecord {}

pub fn run<P>(files_dir_path: P) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut csv_records = Vec::new();
    for entry in fs::read_dir(files_dir_path)? {
        let path = entry.unwrap().path();
        if "csv" == path.extension().unwrap() {
            csv_records.append(&mut read_csv(path)?);
        }
    }
    let unique_records: Vec<_> = HashSet::<CsvRecord>::from_iter(csv_records)
        .into_iter()
        .collect();
    println!("Result len: {}", unique_records.len());
    write_csv(unique_records, "./merged.csv")?;

    Ok(())
}

fn read_csv<P>(path: P) -> Result<Vec<CsvRecord>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut records = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: CsvRecord = result?;
        records.push(record);
    }
    Ok(records)
}

fn write_csv<T, I, P>(records: T, path: P) -> Result<(), Box<dyn Error>>
where
    T: IntoIterator<Item = I>,
    I: serde::Serialize,
    P: AsRef<Path>,
{
    let mut wtr = csv::Writer::from_path(path)?;
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {}
