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
mod tests {
    use super::*;

    #[test]
    fn test_read_csv() -> Result<(), Box<dyn Error>> {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resource/test_read.csv");
        let records = read_csv(d)?;

        assert!(records.iter().len() == 1);
        for record in records {
            assert!(record.tweet_id == 1999999999999999999);
            assert!(record.tweet == "テスト");
            assert!(record.raw_tweet == "テスト_テスト");
            assert!(record.reply_tweet_id == 2999999999999999999);
            assert!(record.reply_tweet == "test");
            assert!(record.raw_reply_tweet == "test_test");
        }
        Ok(())
    }

    #[test]
    fn test_write_csv() -> Result<(), Box<dyn Error>> {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resource/test_write.csv");
        let expected = vec![CsvRecord {
            tweet_id: 3999999999999999999,
            tweet: "tweet str".to_string(),
            raw_tweet: "raw tweet str".to_string(),
            reply_tweet_id: 4999999999999999999,
            reply_tweet: "reply str".to_string(),
            raw_reply_tweet: "raw reply str".to_string(),
        }];
        write_csv(&expected, &d)?;
        let actual = read_csv(&d)?;

        assert!(actual.iter().len() == expected.iter().len());
        for z in actual.iter().zip(expected.iter()) {
            let (a, e) = z;
            assert!(a.tweet_id == e.tweet_id);
            assert!(a.tweet == e.tweet);
            assert!(a.raw_tweet == e.raw_tweet);
            assert!(a.reply_tweet_id == e.reply_tweet_id);
            assert!(a.reply_tweet == e.reply_tweet);
            assert!(a.raw_reply_tweet == e.raw_reply_tweet);
        }
        fs::remove_file(&d)?;
        Ok(())
    }
}
