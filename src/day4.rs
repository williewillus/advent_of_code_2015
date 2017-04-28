extern crate crypto;

use self::crypto::md5;
use self::crypto::digest::Digest;

const SECRET: &str = "ckczppom";

fn compute(start: &str) {
    let mut hasher = md5::Md5::new();

    for attempt in 0.. {
        hasher.reset();
        hasher.input_str(SECRET);
        hasher.input_str(&attempt.to_string());

        if hasher.result_str().starts_with(start) {
            println!("{}", attempt);
            break;
        }
    }
}

pub fn run() {
    compute("00000");

    // Run with optimizations :P
    compute("000000");
}