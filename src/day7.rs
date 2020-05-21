use crate::util;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
enum Ref<'a> {
    Var(&'a str),
    Lit(u16),
}

#[derive(Debug)]
enum Ant<'a> {
    Ref(Ref<'a>),
    And(Ref<'a>, Ref<'a>),
    Or(Ref<'a>, Ref<'a>),
    Lshift(Ref<'a>, Ref<'a>),
    Rshift(Ref<'a>, Ref<'a>),
    Not(Ref<'a>),
}

#[derive(Debug)]
struct Rule<'a> {
    antecedent: Ant<'a>,
    consequent: &'a str,
}

fn parse_ref(s: &str) -> Ref {
    match s.parse::<u16>() {
        Ok(lit) => Ref::Lit(lit),
        Err(_) => Ref::Var(s),
    }
}

fn parse_antecedent(s: &str) -> Ant {
    if let Some((left, right)) = s.split("OR").map(str::trim).next_tuple() {
        return Ant::Or(parse_ref(left), parse_ref(right));
    }

    if let Some((left, right)) = s.split("AND").map(str::trim).next_tuple() {
        return Ant::And(parse_ref(left), parse_ref(right));
    }

    if let Some((left, right)) = s.split("LSHIFT").map(str::trim).next_tuple() {
        return Ant::Lshift(parse_ref(left), parse_ref(right));
    }

    if let Some((left, right)) = s.split("RSHIFT").map(str::trim).next_tuple() {
        return Ant::Rshift(parse_ref(left), parse_ref(right));
    }

    if s.starts_with("NOT") {
        let r = s[3..].trim();
        return Ant::Not(parse_ref(r));
    }

    match s.parse() {
        Ok(l) => Ant::Ref(Ref::Lit(l)),
        Err(_) => Ant::Ref(Ref::Var(s)),
    }
}

fn parse_rule(line: &str) -> Rule {
    let (ant, consequent) = line.split("->").map(str::trim).next_tuple().unwrap();
    let antecedent = parse_antecedent(ant);
    Rule {
        antecedent,
        consequent,
    }
}

fn deref<'a>(r: &Ref<'a>, state: &mut HashMap<&'a str, Option<u16>>) -> Option<u16> {
    match *r {
        Ref::Lit(l) => Some(l),
        Ref::Var(v) => *state.entry(v).or_insert(None),
    }
}

fn run_to_a<'a>(
    state: &mut HashMap<&'a str, Option<u16>>,
    rules: &[Rule<'a>],
    override_b: Option<u16>,
) {
    let mut next = HashMap::new();
    loop {
        if override_b.is_some() {
            state.insert("b", override_b);
        }
        next.clear();
        for rule in rules {
            let new_value = match &rule.antecedent {
                Ant::Ref(v) => deref(v, state),
                Ant::Not(v) => deref(v, state).map(|l| !l),
                Ant::Or(l, r) => match (deref(l, state), deref(r, state)) {
                    (Some(left), Some(right)) => Some(left | right),
                    _ => None,
                },
                Ant::And(l, r) => match (deref(l, state), deref(r, state)) {
                    (Some(left), Some(right)) => Some(left & right),
                    _ => None,
                },
                Ant::Lshift(l, r) => match (deref(l, state), deref(r, state)) {
                    (Some(left), Some(right)) => Some(left << right),
                    _ => None,
                },
                Ant::Rshift(l, r) => match (deref(l, state), deref(r, state)) {
                    (Some(left), Some(right)) => Some(left >> right),
                    _ => None,
                },
            };

            if new_value.is_some() {
                next.insert(rule.consequent, new_value.unwrap());
            }
        }

        for (k, v) in next.drain() {
            let e = state.entry(k).or_insert(None);
            *e = Some(v);
        }
        if state.contains_key("a") && state["a"].is_some() {
            break;
        }
    }
}

pub fn run() {
    let input = util::lines("d7_input.txt").expect("Error reading input");
    let rules = input.iter().map(|l| parse_rule(l)).collect::<Vec<_>>();

    let mut state = HashMap::new();
    run_to_a(&mut state, &rules, None);
    let a = state["a"];
    println!("Part 1: {:?}", a);

    state.clear();
    run_to_a(&mut state, &rules, a);
    println!("Part 2: {:?}", state["a"]);
}
