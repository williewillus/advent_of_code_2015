use std::collections::{HashMap, HashSet};
use crate::util;
use regex::Regex;

/// Iterator yielding the positions of the one bits in the given value, from the right (LSB is position 0)
struct OneBitIterator {
    val: usize,
    next: Option<usize>,
}

impl OneBitIterator {
    fn new(val: usize) -> Self {
        let next = if val == 0 {
            None
        } else {
            Some(val.trailing_zeros() as usize)
        };
        Self {
            val,
            next,
        }
    }

}

impl Iterator for OneBitIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next.take();
        if let Some(idx) = ret {
            for x in idx + 1..std::mem::size_of::<usize>() * 8 {
                if self.val & (1 << x) != 0 {
                    self.next = Some(x);
                    break;
                }
            }
        }
        ret
    }
}

mod test {
    use super::OneBitIterator;
    
    #[test]
    fn test_obi() {
        assert_eq!(0, OneBitIterator::new(0).count());
        assert_eq!(vec![0, 2, 3, 4], OneBitIterator::new(0b11101).collect::<Vec<usize>>());
        assert_eq!((0..std::mem::size_of::<usize>() * 8).collect::<Vec<usize>>(), OneBitIterator::new(!0).collect::<Vec<usize>>());
    }
}

fn read_input() -> (Vec<String>, Vec<Vec<usize>>) {
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let lines = util::lines("d9_input.txt").unwrap();
    let mut city_ids: Vec<String> = Vec::new();
    let mut tmp = Vec::new();

    for line in lines {
        let caps = re.captures(&line).unwrap();
        let a = caps.get(1).unwrap().as_str();
        let b = caps.get(2).unwrap().as_str();
        let cost = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        let a_idx;
        match city_ids.iter().enumerate().find(|&(_, s)| s == a) {
            None => {
                a_idx = city_ids.len();
                city_ids.push(a.to_owned());
            },
            Some(p) =>  {
                a_idx = p.0;
            }
        }

        let b_idx;
        match city_ids.iter().enumerate().find(|&(_, s)| s == b) {
            None => {
                b_idx = city_ids.len();
                city_ids.push(b.to_owned());
            },
            Some(p) =>  {
                b_idx = p.0;
            }
        }

        tmp.push((a_idx, b_idx, cost));
    }

    let mut distances = vec![vec![0; city_ids.len()]; city_ids.len()];
    for (a_idx, b_idx, cost) in tmp {
        distances[a_idx][b_idx] = cost;
        distances[b_idx][a_idx] = cost;
    }

    (city_ids, distances)
}

const UNCOMPUTED: usize = 0xDEADBEEF;

pub fn run() {
    let (city_ids, costs) = read_input();
    // dp[x][y] stores the path of minimum length from city y back to the start, passing once through each of the remaining cities x
    let mut dp = vec![vec![UNCOMPUTED; city_ids.len()]; 1 << city_ids.len()];

    // mask values: 0 -> all cities visited (no cities left). -1 -> no cities visited (all cities left)

    // fill in base case
    for x in 0..city_ids.len() {
        // the only path from city x back to the start, passing through the remaining city {x}, is to just go back to the start. this is cost 0.
        dp[1 << x][x] = 0;
    }

    for num_unvisited in 2..=city_ids.len() {
        // there are still num_unvisited cities left. for each combination of those remaining cities
        for mask in 0usize..1 << city_ids.len() {
            if mask.count_ones() as usize == num_unvisited {
                // for each remaining city
                for k in OneBitIterator::new(mask) {
                    let mask_no_k = mask & !(1 << k);
                    // for each remaining city m *besides* k, get the minimum cost of going from m back to start, passing through the remaining cities not including k
                    // then adding in the cost from k to m
                    let v = OneBitIterator::new(mask_no_k)
                        .map(|m| {
                            let recur = dp[mask_no_k][m];
                            if recur == UNCOMPUTED {
                                panic!("case {:08b}, {} wasn't computed when I asked for it", mask_no_k, m)
                            }
                            recur + costs[k][m]
                        })
                        .min()
                        .expect("A minimum should always exist");
                    // and that is the answer for the min cost of going from k back to the start, passing through the remaining cities
                    dp[mask][k] = v;
                }
            }
        }
    }

    // now, the final answer is the minimum over each choice of start city and completing a path back to start passing through every city
    let ans = (0..city_ids.len())
        .map(|k| dp[(1 << city_ids.len()) - 1][k])
        .min().expect("A minimum should always exist");
    println!("Part 1: {}", ans);
}
