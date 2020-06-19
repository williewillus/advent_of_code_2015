use crate::util;
use itertools::Itertools;

fn compute(input: &[usize], p2: bool) -> usize {
    let total_weight = input.iter().sum::<usize>();
    let factor = if p2 { 4 } else { 3 };

    let mut min_state = (usize::MAX, usize::MAX);
    
    for g1_len in 1..=input.len() {
        for comb in input.iter().copied().combinations(g1_len) {
            let g1_weight = comb.iter().sum::<usize>();
            if g1_weight * factor != total_weight {
                continue;
            }

            let state = (comb.len(), comb.iter().product());
            min_state = min_state.min(state);
        }
    }

    min_state.1
}

pub fn run() {
    let input = util::lines("d24_input.txt").unwrap()
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", compute(&input, false));
    println!("Part 2: {}", compute(&input, true));
}

#[cfg(test)]
mod test {
    use super::compute;
    
    #[test]
    fn test_example() {
        let input = &[1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(compute(input, false), 99);
        assert_eq!(compute(input, true), 44);
    }
}
