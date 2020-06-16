const INPUT: usize = 36000000;

fn factorize(num: usize, factors: &mut Vec<usize>) {
    let until = (num as f64).sqrt().ceil() as usize;
    for i in 1..=until {
        if num % i == 0 {
            factors.push(num / i);
            factors.push(i);
        }
    }
}

fn presents_at(factors: &mut Vec<usize>, house: usize, p2: bool) -> usize {
    factors.clear();
    factorize(house, factors);
    if p2 {
        11 * factors.iter().filter(|&f| house / f <= 50).sum::<usize>()
    } else {
        10 * factors.iter().sum::<usize>()
    }
}

pub fn run() {
    let mut scratch = Vec::new();
    let p1 = (1..).map(|i| (i, presents_at(&mut scratch, i, false)))
        .find(|&(_, p)| p >= INPUT).unwrap().0;
    println!("Part 1: {}", p1);

    let p2 = (1..).map(|i| (i, presents_at(&mut scratch, i, true)))
        .find(|&(_, p)| p >= INPUT).unwrap().0;
    println!("Part 2: {}", p2);
}
