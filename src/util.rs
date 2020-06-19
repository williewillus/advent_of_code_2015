use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

pub fn read_all(path: &str) -> Result<String> {
    let f = File::open(path)?;

    let mut input = String::new();
    let mut rdr = BufReader::new(f);

    rdr.read_to_string(&mut input)?;

    Ok(input)
}

pub fn lines(path: &str) -> Result<Vec<String>> {
    let f = File::open(path)?;
    let rdr = BufReader::new(f);
    let mut ret = Vec::new();

    for l in rdr.lines() {
        ret.push(l?);
    }

    Ok(ret)
}
