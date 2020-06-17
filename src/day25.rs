const SEED: usize = 20151125;

const fn sum_to(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

/// Given a row, column on the table, calculate which iteration of the generation algorithm it was calculated on
const fn unfold(row: i32, col: i32) -> usize {
    // which diagonal am I on? = the column that I end up at when I move as many rows as I can up and right
    let diagonal = col + row - 1;

    // account for all previous diagonal groups
    let previous_groups = sum_to(diagonal - 1);

    // then for my offset within my diagonal group
    (previous_groups + col) as usize
}

fn run_iter(v: &usize) -> Option<usize> {
    Some((v * 252533) % 33554393)
}

pub fn run() {
    let index = unfold(2978, 3083) - 1; // zero index
    let v = std::iter::successors(Some(SEED), run_iter).nth(index).unwrap();
    println!("Part 1: {}", v);
}


#[cfg(test)]
mod test {
    use super::unfold;
    
    #[test]
    fn test_unfold() {
        assert_eq!(1, unfold(1, 1));
        assert_eq!(16, unfold(6, 1));
        assert_eq!(20, unfold(2, 5));
        assert_eq!(21, unfold(1, 6));
    }
}
