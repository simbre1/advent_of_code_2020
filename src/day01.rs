#![allow(dead_code)]
use std::collections::*;

pub fn solve() {
    println!("Day 01");

    let input = std::fs::read_to_string("input/day01/input.txt").expect("peut");

    let mut nums: BTreeSet<i64> = BTreeSet::new();

    for line in input.lines() {
        nums.insert(line.parse::<i64>().unwrap());
    }

    solve_impl(&nums, 2020, 2);
    solve_impl(&nums, 2020, 3);
}

fn solve_impl(nums: &BTreeSet<i64>, split: i64, into: u8) {
    let result = find(&nums, split, into);
    let product: i64 = result.iter().product();
    println!("parts: {}, product: {}", format!("{:?}", result), product);
}

pub fn find(nums: &BTreeSet<i64>, split: i64, into: u8) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    if into > 1 {
        for num in nums.range(..split) {
            let parts = find(nums, split - num, into - 1);
            if parts.len() > 0 {
                result.push(*num);
                result.extend(parts);
                return result;
            }
        }
        return result;
    } else if nums.contains(&split) {
        result.push(split)
    }
    return result;
}
