use regex::Regex;
use std::collections::HashMap;
use crate::util;

#[derive(Copy, Clone, Debug)]
struct Attribute {
    speed: usize,
    duration: usize,
    rest: usize,
}

fn simulate_one(attr: Attribute, duration: usize) -> usize {
    let complete_cycles = duration / (attr.duration + attr.rest);
    let complete_time = complete_cycles * attr.duration;
    let complete_dist = complete_time * attr.speed;

    // we know anything over `attr.duration` is just resting
    let partial_time = (duration % (attr.duration + attr.rest)).min(attr.duration);
    let partial_dist = partial_time * attr.speed;

    complete_dist + partial_dist
}

fn simulate<'a>(attrs: &'a HashMap<String, Attribute>, duration: usize) -> HashMap<&'a str, usize> {
    let mut ret = HashMap::new();
    for (k, v) in attrs {
	ret.insert(k.as_ref(), simulate_one(*v, duration));
    }
    ret
}

fn leaders<'a>(standings: &HashMap<&'a str, usize>) -> Vec<(&'a str, usize)> {
    let max = standings.values().max().expect("Empty standings?");
    standings.iter().filter(|p| p.1 == max).map(|(k, v)| (*k, *v)).collect()
}

pub fn run() {
    let mut attrs = HashMap::new();
    let r = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
    for line in util::lines("d14_input.txt").expect("Failed to read input") {
	let v = r.captures(&line).unwrap();
	let speed = v[2].parse::<usize>().unwrap();
	let duration = v[3].parse::<usize>().unwrap();
	let rest = v[4].parse::<usize>().unwrap();
	attrs.insert(v[1].to_owned(), Attribute { speed, duration, rest });
    }

    let p1 = simulate(&attrs, 2503);
    println!("Part 1: {:?}", leaders(&p1));

    let mut p2 = HashMap::new();
    for i in 1..2503 {
	for (name, _) in leaders(&simulate(&attrs, i)) {
	    *p2.entry(name).or_insert(0) += 1;
	}
    }
    println!("Part 2: {:?}", leaders(&p2));
}
