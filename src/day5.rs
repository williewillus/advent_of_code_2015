extern crate pcre;

use self::pcre::Pcre;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART_2: bool = true;

fn is_vowel(c: &char) -> bool {
    match *c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

fn has_consecutive_letters(s: &String) -> bool {
    let mut prev_char = s.chars().next().unwrap();

    for c in s.chars().skip(1) {
        if prev_char == c {
            return true;
        } else {
            prev_char = c;
        }
    }

    return false;
}

fn is_nice_1(s: &String) -> bool {
    !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy")
        && s.chars().filter(is_vowel).count() >= 3
        && has_consecutive_letters(s)
}

fn is_nice_2(s: &String) -> bool {
    let mut nonoverlap = Pcre::compile(r"([a-z]{2}).*?\1").unwrap();
    let mut repeat = Pcre::compile(r"([a-z])[a-z]\1").unwrap();

    let nonoverlap_count = nonoverlap.matches(s).count();
    let has_repeat = repeat.exec(s).is_some();

    nonoverlap_count >= 1 && has_repeat
}

pub fn run() {
    let rdr = BufReader::new(File::open("d5_input.txt").expect("Couldn't open input file!"));

    let nice_count = rdr.lines()
                        .map(|r| r.expect("Failure reading line"))
                        .filter(if PART_2 { is_nice_2 } else { is_nice_1 }).count();
    println!("{} nice {} strings", nice_count, if PART_2 { "v2" } else { "v1" });
}