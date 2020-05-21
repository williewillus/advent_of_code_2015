use md5::Digest;

const SECRET: &str = "ckczppom";

fn compute<F>(mut done: F) where F: FnMut(Digest) -> bool {
    for attempt in 0.. {
        let s = format!("{}{}", SECRET, attempt.to_string());
        let digest = md5::compute(s.as_bytes());

        if done(digest) {
            println!("{}", attempt);
            break;
        }
    }
}

pub fn run() {
    compute(|d| d[0] == 0 && d[1] == 0 && d[2] & 0xF0 == 0);

    // Run with optimizations :P
    compute(|d| d[0] == 0 && d[1] == 0 && d[2] == 0);
}
