#![allow(dead_code)]

pub fn solve() {
    println!("Day 10");

    let input = std::fs::read_to_string("input/day10/input.txt").expect("peut");
    let step = 3;

    let mut adapters: Vec<i64> = input.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + step);

    let r = solve_imp(&adapters, step);

    let diff1 = r.0.iter().filter(|diff| **diff == 1).count();
    let diff3 = r.0.iter().filter(|diff| **diff == 3).count();

    println!("part 1: {}*{} = {}", diff1, diff3, diff1 * diff3);
    println!("part 2: {}", r.1);
}

fn solve_imp(adapters: &[i64], step: i64) -> (Vec<i64>, u64) {
    let mut diffs = vec![0i64;adapters.len()];
    let mut ps = vec![0u64;adapters.len()];
    ps[0] = 1;

    for i in 1..adapters.len() {
        diffs[i] = adapters[i] - adapters[i - 1];

        if diffs[i] > step {
            return (diffs, 0);
        }
        
        let mut j = i - 1;
        while (adapters[i] - adapters[j]) <= step {
            ps[i] += ps[j];
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
    return (diffs, ps[ps.len()-1]);
}