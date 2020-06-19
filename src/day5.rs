use pcre::Pcre;
use crate::util;

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn has_consecutive_letters(s: &str) -> bool {
    let mut prev_char = s.chars().next().unwrap();

    for c in s.chars().skip(1) {
        if prev_char == c {
            return true;
        } else {
            prev_char = c;
        }
    }

    false
}

fn is_nice_1(s: &str) -> bool {
    !s.contains("ab")
        && !s.contains("cd")
        && !s.contains("pq")
        && !s.contains("xy")
        && s.chars().filter(|&c| is_vowel(c)).count() >= 3
        && has_consecutive_letters(s)
}

fn is_nice_2(s: &str) -> bool {
    let mut nonoverlap = Pcre::compile(r"([a-z]{2}).*?\1").unwrap();
    let mut repeat = Pcre::compile(r"([a-z])[a-z]\1").unwrap();

    let nonoverlap_count = nonoverlap.matches(s).count();
    let has_repeat = repeat.exec(s).is_some();

    nonoverlap_count >= 1 && has_repeat
}

pub fn run() {
    let lines = util::lines("d5_input.txt").unwrap();

    let p1 = lines.iter().filter(|s| is_nice_1(s)).count();
    println!("Part 1: {}", p1);

    let p2 = lines.iter().filter(|s| is_nice_2(s)).count();
    println!("Part 2: {}", p2);
}
