use std::fs::File;
use std::io::{BufReader, Result};
use std::path::Path;

pub fn get_reader<P>(filename: P) -> Result<BufReader<File>> where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}