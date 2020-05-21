use pcre::Pcre;
use std::collections::HashSet;
use std::str;

const INPUT: &str = "cqjxjnds";
const INPUT_SIZE: usize = INPUT.len();

fn rotate_str(s: &mut [u8]) {
    for idx in (0..s.len()).rev() {
        if s[idx] == b'z' {
            s[idx] = b'a';
        } else {
            s[idx] += 1;
            break;
        }
    }
}

fn is_valid(s: &[u8]) -> bool {
    if s.iter().any(|&c| c == b'i' || c == b'o' || c == b'l') {
        return false;
    }

    if s.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]) {
        // TODO: probably a better sliding-window-ish method that doesn't need regex
        let mut re = Pcre::compile(r"([a-z])\1").unwrap();
        let mut uniq_pairs = HashSet::new();

        for m in re.matches(str::from_utf8(s).unwrap()) {
            uniq_pairs.insert(m.group(1).chars().next());
        }

        return uniq_pairs.len() >= 2;
    }

    false
}

pub fn run() {
    let mut bytes = [0; INPUT_SIZE];
    bytes.copy_from_slice(INPUT.as_bytes());

    let mut valid_found = 0;

    loop {
        {
            if is_valid(&bytes) {
                println!("{}", str::from_utf8(&bytes).unwrap());

                valid_found += 1;
                if valid_found == 2 {
                    break;
                }
            }
        }

        rotate_str(&mut bytes);
    }
}
