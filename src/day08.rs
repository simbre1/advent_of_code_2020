#![allow(dead_code)]

struct IntOp {
    code: String,
    param: i32,
}

struct RunResult {
    acc: i32,
    prog_counter: usize,
    max_prog_counter: usize,
}

pub fn solve() {
    println!("Day0 8");

    let input = std::fs::read_to_string("input/day08/input.txt").expect("peut");
    let ops: Vec<IntOp> = input.lines()
        .map(|line| {
            let words: Vec<&str> = line.split_ascii_whitespace().collect();
            IntOp {
                code: String::from(words[0]),
                param: words[1].parse::<i32>().unwrap(),
            }
        })
        .collect();

    let result1 = run_prog(&ops, ops.len());

    println!("acc 1: {}", result1.acc);
    
    let mut result2: RunResult = RunResult{acc: 0, prog_counter: 0, max_prog_counter: 0};
    for i in (0..result1.max_prog_counter).rev() {
        if ops[i].code != "acc" {
            result2 = run_prog(&ops, i);
            if result2.prog_counter >= ops.len() {
                break;
            }
        }
    }
    println!("acc 2: {}", result2.acc);
}

fn run_prog(ops: &Vec<IntOp>, flip: usize) -> RunResult {
    let mut trace = vec![0u32; ops.len()];
    let mut acc = 0;
    let mut prog_counter = 0;
    let mut max_prog_counter = 0;
    while prog_counter < ops.len() {
        max_prog_counter = std::cmp::max(max_prog_counter, prog_counter);

        trace[prog_counter] += 1;
        if trace[prog_counter] > 1 {
            break;
        }

        let op = &ops[prog_counter];
        let is_flip = flip == prog_counter;
        if (op.code == "jmp" && !is_flip) || (op.code == "nop" && is_flip) {
            prog_counter = (prog_counter as i32 + op.param) as usize;
            continue;
        } else if op.code == "acc" {
            acc += op.param;
        }
        prog_counter += 1;
    }

    return RunResult {
        acc: acc,
        prog_counter: prog_counter,
        max_prog_counter: max_prog_counter,
    }
}