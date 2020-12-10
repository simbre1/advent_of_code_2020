#![allow(dead_code)]
use std::collections::*;

pub fn solve() {
    println!("Day 09");

    let input = std::fs::read_to_string("input/day09/input.txt").expect("peut");
    let preamble: usize = 25;

    let nums: Vec<i64> = input.lines()
        .map(|line|  line.parse::<i64>().unwrap())
        .collect();

    let x = part1(&nums, preamble);
    println!("part 1: {}", x);

    let parts = find_contiguous_parts(&nums, x);
    let sum = parts.iter().min().unwrap() + parts.iter().max().unwrap();
    println!("part 2: {} <- {}", sum, format!("{:?}", parts));

    //556543474 76096372
}

fn part1(nums: &Vec<i64>, preamble: usize) -> i64 {
    for i in preamble..(nums.len()-1) {
        let parts = find_parts(&nums[i-preamble..i], nums[i], 2);
        if parts.is_empty() {
            return nums[i];
        }
    }
    unreachable!("wrong input");
}

fn find_parts(nums: &[i64], split: i64, into: u8) -> Vec<i64> {
    let tree: BTreeSet<i64> = nums.iter().cloned().collect();
    return super::day01::find(&tree, split, into);
}

fn find_contiguous_parts(nums: &[i64], split: i64) -> &[i64] {
    let mut i = 0;
    let mut j = 0;
    let mut sum = nums[i];

    while i < nums.len() && j < nums.len() {
        if sum < split {
            j += 1;
            if nums[j] >= split {
                i = j;
                sum = nums[i];
            } else {
                sum += nums[j];
            }
        } else if sum > split {
            sum -= nums[i];
            i += 1;
            if i > j {
                j = i;
                sum = nums[i];
            }
        } else {
            break;
        }
    }
    return &nums[i..j+1];
}