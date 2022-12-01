// #![allow(dead_code)]

pub fn solve() {
    println!("Day 14");

    let input = std::fs::read_to_string("input/day14/input.txt").expect("peut");
    let puzzle = Puzzle::from_string(&input);

    let r1 = part1(&puzzle);
    println!("part 1: {}", r1.result());

    let r2 = part2(&puzzle);
    println!("part 2: {}", r2);
}

enum Op {
    Mask(String),
    Mem(i64, i64), // (addr, val)
    Noop,
}

use Op::*;

struct Puzzle {
    ops: Vec<Op>,
}

impl Puzzle {
    pub fn from_string(input: &str) -> Self {
        let ops: Vec<Op> = input
            .lines()
            .map(|line| {
                if line.starts_with("mask") {
                    Mask(String::from(line.split("=").nth(1).unwrap().trim()))
                } else if line.starts_with("mem") {
                    let i = line.find("[").unwrap();
                    let j = line.find("]").unwrap();
                    let addr = line[i+1..j].parse::<i64>().unwrap();
                    let val = line.split("=").nth(1).unwrap().trim().parse::<i64>().unwrap();
                    Mem(addr, val)
                } else {
                    Noop
                }
            })
            .collect();
        Puzzle { ops }
    }
}

//#[derive(Clone, Copy)]
struct Res {
    mem: std::collections::BTreeMap<i64, i64>,
}

impl Res {
    pub fn new() -> Self {
        Res { mem: std::collections::BTreeMap::new() }
    }

    pub fn result(&self) -> i64 {
        self.mem.values().sum()
    }
}

impl std::fmt::Display for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "mem size:{}", self.mem.len())
    }
}

fn part1(puzzle: &Puzzle) -> Res {
    let mut res = Res::new();
    let mut mask = &String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    for op in &puzzle.ops {
        if let Mask(m) = op {
            mask = m;
        }
        if let Mem(addr, val) = op {
            res.mem.insert(*addr, to_mem_val(mask, *val));
        }
    }
    // println!("mem size: {}, mem: {}", res.mem.len(), format!("{:?}", res.mem));
    res
}

fn to_mem_val(mask: &String, val: i64) -> i64 {
    let bin = format!("{:b}", val);
    let binpad = format!("{:0>36}", bin);
    let binmask: String = binpad
        .chars()
        .enumerate()
        .map(|(i, c)| 
            match mask.chars().nth(i).unwrap() {
                '1' => '1',
                '0' => '0',
                _ => c,
            })
        .collect();

    println!("bin {}: {}", val, binpad);

    let num = i64::from_str_radix(&binmask, 2).unwrap();
    num
}


fn part2(puzzle: &Puzzle) -> Res {
    let res = Res::new();
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let puzzle = super::Puzzle::from_string("\
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
            mem[8] = 11\n\
            mem[7] = 101\n\
            mem[8] = 0");

        let res = super::part1(&puzzle);
        assert_eq!(*(res.mem.get(&7).unwrap()), 101);
        assert_eq!(*(res.mem.get(&8).unwrap()), 64);
        assert_eq!(res.result(), 165);
    }

    // #[test]
    // fn test_part2() {
    //     let puzzle = super::Puzzle::from_string("\
    //     939\n\
    //     17,x,13,19");

    //     let res = super::part2(&puzzle);
    //     assert_eq!(res, 3417);
    // }
}