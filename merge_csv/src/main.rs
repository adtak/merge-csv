use std::env;
use std::error::Error;
use std::ffi::OsString;

fn main() -> Result<(), Box<dyn Error>> {
    let files_dir_path = get_first_args()?;
    merge_csv::run(files_dir_path)
}

fn get_first_args() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[cfg(test)]
mod tests {

}
