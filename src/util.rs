use std::fs::File;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_all(path: &str) -> Option<String> {
    let f = File::open(path);

    if f.is_ok() {
        let mut input = String::new();
        let mut rdr = BufReader::new(f.unwrap());

        if rdr.read_to_string(&mut input).is_ok() {
            return Some(input);
        }
    }

    return None;
}

pub fn lines(path: &str) -> Option<Vec<String>> {
    let f = File::open(path);

    if f.is_ok() {
        let rdr = BufReader::new(f.unwrap());
        return Some(rdr.lines().filter_map(Result::ok).collect());
    }

    return None;
}
