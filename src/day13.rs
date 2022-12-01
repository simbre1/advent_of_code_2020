#![allow(dead_code)]

pub fn solve() {
    println!("Day 13");

    let input = std::fs::read_to_string("input/day13/input.txt").expect("peut");
    let puzzle = Puzzle::from_string(&input);

    let r1 = part1(&puzzle);
    println!("part 1: {}", r1);

    let r2 = part2(&puzzle);
    println!("part 2: {}", r2);
}

type Bus = Option<i64>;

struct Puzzle {
    depart_time: i64,
    busses: Vec<Bus>,
}

impl Puzzle {
    pub fn from_string(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        Puzzle {
            depart_time: lines[0].parse::<i64>().unwrap(),
            busses: lines[1].split(",").map(|x| x.parse::<i64>().ok()).collect(),
        }
    }
}

#[derive(Clone, Copy)]
struct Res {
    bus_id: i64,
    bus_time: i64,
    depart_time: i64,
}

impl Res {
    pub fn new(bus_id: i64, bus_time: i64, depart_time: i64) -> Self {
        Res { bus_id, bus_time, depart_time }
    }

    pub fn result(&self) -> i64 {
        self.bus_id * (self.bus_time - self.depart_time)
    }
}

impl std::fmt::Display for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "bus_id:{} bus_time:{} depart_time:{} result:{}", self.bus_id, self.bus_time, self.depart_time, self.result())
    }
}

fn part1(puzzle: &Puzzle) -> Res {
    let busses: Vec<i64> = puzzle.busses.iter()
        .filter(|b| **b != None)
        .map(|b| b.unwrap())
        .collect();

    let bus_id_time = busses.iter()
        .map(|bus_id| (bus_id, smallest_multiple(*bus_id, puzzle.depart_time)))
        .min_by_key(|bus_id_time| bus_id_time.1)
        .unwrap();
    
    Res::new(*bus_id_time.0, bus_id_time.1, puzzle.depart_time)
}

fn smallest_multiple(part: i64, threshold: i64) -> i64 {
    ((threshold / part) + 1) * part
}

fn least_common_multiple(a: i64, b: i64) -> i64 {
    // input is all primes so fuck it
    a * b
}

fn part2(puzzle: &Puzzle) -> i64 {
    let mut lcm = 1;
    let mut sum = 0;

    for (i, bus) in puzzle.busses.iter().enumerate() {
        if *bus == None {
            continue;
        }
        let bus_time = bus.unwrap();

        while sum == 0 || ((sum + i as i64) % bus_time) != 0 {
            sum += lcm;
        }
        lcm = least_common_multiple(lcm, bus_time);
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let puzzle = super::Puzzle::from_string("\
        939\n\
        7,13,x,x,59,x,31,19");

        let res = super::part1(&puzzle);
        assert_eq!(res.bus_id, 59);
        assert_eq!(res.bus_time, 944);
        assert_eq!(res.depart_time, 939);
        assert_eq!(res.result(), 295);
    }

    #[test]
    fn test_part2() {
        let puzzle = super::Puzzle::from_string("\
        939\n\
        17,x,13,19");

        let res = super::part2(&puzzle);
        assert_eq!(res, 3417);
    }
}