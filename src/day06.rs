#![allow(dead_code)]
pub fn solve() {
    println!("Day 06");

    let input = std::fs::read_to_string("input/day06/input.txt").expect("peut");

    let start = 'a' as usize;
    let mut sum = 0;
    let mut sum2 = 0;

    for block in input.split("\n\n") {
        let mut arr = [0; 26];

        for line in block.lines() {
            for c in line.chars() {
                arr[c as usize - start] += 1;
            }
        }
        sum += arr.iter().filter(|a| **a > 0).count();

        let n = block.lines().count();
        sum2 += arr.iter().filter(|a| **a == n).count();
    }

    println!("sum: {}", sum);
    println!("sum2: {}", sum2);
}