use regex::Regex;
use serde_json::{self, Error, Map, Number, Value};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

// The problem input is shallow enough, so don't mind recursive calls
fn compute_sum(v: &Value) -> i64 {
    match *v {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(ref n) => n.as_i64().expect("not an i64?"),
        Value::Array(ref vec) => {
            let mut s = 0;
            for val in vec {
                s += compute_sum(val);
            }
            s
        },
        Value::Object(ref m) => {
            let mut s = 0;
            for v in m.values() {
                match *v {
                    Value::String(ref v_str) => {
                        if v_str == "red" {
                            return 0;
                        }
                    },
                    _ => s += compute_sum(v)
                }
            }
            s
        }
    }
}

pub fn run() {
    let input = {
        let mut input = String::new();
        let mut rdr = BufReader::new(File::open("d12_input.txt").expect("Couldn't open input file!"));
        rdr.read_to_string(&mut input).expect("failure reading input");
        input
    };

    let sum = Regex::new(r"-?\d+").unwrap()
        .find_iter(&input)
        .map(|mat| mat.as_str().parse::<i64>().expect("couldn't parse number?"))
        .sum::<i64>();

    println!("sum is {}", sum);

    let v: Value = serde_json::from_str(&input).unwrap();
    println!("proper sum is {}", compute_sum(&v));
}
