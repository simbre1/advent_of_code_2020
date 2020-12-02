#![allow(dead_code)]
use super::aoc;

struct Pwd {
    min: u8,
    max: u8,
    c: char,
    password: String,
}

pub fn solve() {
    println!("Day 02");

    let input = std::fs::read_to_string("input/day02/input.txt").expect("peut");

    let mut pwds: Vec<Pwd> = Vec::new();
    for line in input.lines() {
        let dash = line.find("-").unwrap();
        let space = line.find(" ").unwrap();
        let colon = line.find(":").unwrap();

        let min = line[0..dash].parse::<u8>().unwrap();
        let max = line[dash+1..space].parse::<u8>().unwrap();
        let c = line.chars().nth(colon-1).unwrap();
        let password = line[colon..].trim();

        let pwd = Pwd {
            min, max, c, password: String::from(password),
        };
        pwds.push(pwd);
    }

    solve_impl(&pwds);
}

impl Pwd {
    fn is_valid(&self) -> bool {
        let counts = aoc::get_char_counts(&self.password);
        match counts.get(&self.c) {
            None => false,
            Some(count) => *count >= self.min && *count <= self.max,
        }
    }

    fn is_valid2(&self) -> bool {
        let match_min = match self.password.chars().nth((self.min + 1).into()) {
            None => false,
            Some(c) => c == self.c,
        };
        let match_max = match self.password.chars().nth((self.max + 1).into()) {
            None => false,
            Some(c) => c == self.c,
        };
        return match_min != match_max;
    }
}

fn solve_impl(pwds: &Vec<Pwd>) {
    println!("# valids 1: {}", pwds.iter().filter(|pwd| pwd.is_valid()).count());
    println!("# valids 2: {}", pwds.iter().filter(|pwd| pwd.is_valid2()).count());
}
