use crate::util;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

fn read_input() -> (Vec<(String, String)>, String) {
    let mut rules = Vec::new();
    let mut init = None;

    for line in util::lines("d19_input.txt").unwrap() {
        if line.trim().is_empty() {
            continue;
        }

        if line.contains("=>") {
            let mut splits = line.split("=>");
            let left = splits.next().unwrap().trim();
            let right = splits.next().unwrap().trim();
            rules.push((left.to_owned(), right.to_owned()));
        } else {
            init = Some(line);
        }
    }
    return (rules, init.expect("Input file did not contain an initial value"));
}

fn p1(rules: &[(String, String)], init: &str) -> usize {
    let mut seen = HashSet::new();
    let mut scratch = String::new();

    for (ant, cons) in rules {
        let re = Regex::new(ant).unwrap();
        for f in re.find_iter(&init) {
            scratch.clear();

            // add everything before the match
            scratch += &init[..f.start()];

            // replace the match with the consequent
            scratch += cons;

            // add everything after the match
            scratch += &init[f.end()..];

            if !seen.contains(&scratch) {
                seen.insert(scratch.clone());
            }
        }
    }
    seen.len()
}

/* failed bfs
fn p2(rules: &[(String, String)], target: &str) -> usize {
    let mut scratch = String::new();
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back((0, "e".to_owned()));

    while !q.is_empty() {
        let (dist, cur) = q.pop_front().unwrap();
        if cur == target {
            return dist;
        }

        for (ant, cons) in rules {
            let re = Regex::new(ant).unwrap();
            for f in re.find_iter(&cur) {
                scratch.clear();

                // add everything before the match
                scratch += &cur[..f.start()];

                // replace the match with the consequent
                scratch += cons;

                // add everything after the match
                scratch += &cur[f.end()..];

                if !seen.contains(&scratch) {
                    seen.insert(scratch.clone());
                    q.push_back((dist + 1, scratch.clone()));
                }
            }
        }
    }

    eprintln!("Exhausted search space without finding target");
    0
}
*/

fn p2(target: &str) -> usize {
    // Source: https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju/
    /*
    The gist of it is that the rules only take the following form:
    X -> JK (a molecule produces two other molecules)
    X -> JRnKAr (a molecule produces another molecule followed by the parenthetical clause Rn K Ar)
    X -> JRnKYLAr (multiple molecules can be present in the parenthetical, in which case Y is used to separate them)
     */

    // Every element name is a capital and zero or more lowercase, so just counting capitals is enough
    let caps = target.chars().filter(char::is_ascii_uppercase).count();

    let open = Regex::new("Rn").unwrap().find_iter(target).count();
    let close = Regex::new("Ar").unwrap().find_iter(target).count();
    let comma = target.chars().filter(|&c| c == 'Y').count();

    // Now, the number of rule applications to reach target is the same as the number to shrink the target back to `e` using the rules
    // This is:
    caps - 1 // The number of symbols itself minus 1 (using the X -> XX rules)
        - open - close // Less the number of opens and closes (Rn's and Ar's). This is because applying the rule in reverse eats them for 'free'
        - 2 * comma // Less twice the number of commas. Each comma and the element that follows it is eaten for 'free'.
}

pub fn run() {
    let (rules, init) = read_input();
    println!("Part 1: {}", p1(&rules, &init));
    println!("Part 1: {}", p2(&init));
}
