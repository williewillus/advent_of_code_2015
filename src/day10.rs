const INPUT: &str = "1113222113";

fn compute(iters: u32) -> usize {
    let mut s = String::from(INPUT);
    let mut temp = String::new();

    for _ in 0..iters {
        let mut prev_char = s.as_bytes()[0] as char;
        let mut char_count = 0;

        for (idx, c) in s.chars().enumerate() {
            if prev_char == c {
                char_count += 1;
            } else {
                temp += char_count.to_string().as_str();
                temp.push(prev_char);
                char_count = 1;
            }

            // if at the end, finish off current one also
            if idx == s.len() - 1 {
                temp += char_count.to_string().as_str();
                temp.push(c);
            }

            prev_char = c;
        }

        s.clear();
        s += temp.as_str();
        temp.clear();
    }

    s.len()
}

pub fn run() {
    println!("Part 1: {}", compute(40));
    println!("Part 2: {}", compute(50));
}
