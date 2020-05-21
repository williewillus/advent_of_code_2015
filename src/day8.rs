use crate::util;
use regex::Regex;

fn eval_escapes(s: &str, hex: &Regex) -> usize {
    // remove start and end quotes, then replace easy sequences
    let nq = s[1..s.len() - 1]
        .replace(r"\\", r"\")
        .replace(r#"\""#, r#"""#);

    // Cheat: instead of actually replacing hex escapes with their character,
    // just remove them and add the count separately
    let hexes = hex.find_iter(&nq).count();
    let removed = hex.replace_all(&nq, "");
    removed.len() + hexes
}

fn expand(s: &str) -> usize {
    let mut res = 2; // Open and close quotes
    for c in s.chars() {
        if c == '"' || c == '\\' {
            res += 2;
        } else {
            res += 1;
        }
    }
    res
}

pub fn run() {
    let lines = util::lines("d8_input.txt").expect("Failed to read input");
    let r = Regex::new(r"\\x[a-f0-9][a-f0-9]").unwrap();
    let raw_sum = lines.iter().map(|s| s.len()).sum::<usize>();

    let eval_sum = lines.iter().map(|s| eval_escapes(s, &r)).sum::<usize>();
    println!("Part 1: {}", raw_sum - eval_sum);

    let expand_sum = lines.iter().map(|s| expand(s)).sum::<usize>();
    println!("Part 2: {}", expand_sum - raw_sum);
}
