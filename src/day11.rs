#![allow(dead_code)]

use std::cmp::*;
use std::collections::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pos {
    Occ,
    Free,
    Flr
}

type Dir = (i32, i32);

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self { Occ => "#", Free => "L", _ => "."})
    }
}

use Pos::*;

type Layout = Vec<Vec<Pos>>;

pub fn solve() {
    println!("Day 11");

    let input = std::fs::read_to_string("input/day11/input.txt").expect("peut");

    let layout: Layout = to_layout(&input);
    let r1 = part1(&layout);
    println!("part 1: {}", r1);

    let r2 = part2(&layout);
    println!("part 2: {}", r2);
}

fn to_layout(input: &str) -> Layout {
    input.lines()
        .map(|line| 
            line.chars()
                .map(|c| match c { '#' => Occ, 'L' => Free, _ => Flr })
                .collect())
        .collect()
}

fn part1(layout: &Layout) -> usize {
    let stable = find_stable_layout(layout, next_layout_part1, false).0;
    return vec2d_count(&stable, &Occ);
}

fn part2(layout: &Layout) -> usize {
    let stable = find_stable_layout(layout, next_layout_part2, false).0;
    return vec2d_count(&stable, &Occ);
}

fn find_stable_layout(layout: &Layout, gen_func: fn(&Layout) -> Layout, debug: bool) -> (Layout, u64) {
    let mut prev = layout.clone();
    let mut next = gen_func(&prev);

    let mut i = 1;
    while !vec2d_equals(&prev, &next) {
        if debug {
            println!("gen {}", i);
            println!("{}", layout_to_string(&next));
            println!();
        }

        prev = next;
        next = gen_func(&prev);
        i += 1;
    }
    return (next.clone(), i);
}

fn next_layout_part1(layout: &Layout) -> Layout {
    let rows = layout.len();
    let cols = layout[0].len();
    let mut r: Layout = vec![vec![Flr; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            if layout[i][j] == Flr {
                r[i][j] = Flr;
                continue;
            }

            let mut num_occ = 0;
            for ii in max(0, i as i64 -1) as usize..min(rows, i+2) {
                for jj in max(0, j as i64 -1) as usize..min(cols, j+2) {
                    // println!("{}", layout[ii][jj]);
                    if (i != ii || j != jj) && layout[ii][jj] == Occ {
                        num_occ += 1;
                    }
                }
            }
            r[i][j] = match layout[i][j] {
                Occ => if num_occ >= 4 { Free } else { Occ },
                Free => if num_occ == 0 { Occ } else {Free },
                _ => Flr,
            };
        }
    }
    return r;
}

fn next_layout_part2(layout: &Layout) -> Layout {
    let dirs = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1)
    ];
    let mut dir_occ_map: HashMap<Dir,bool> = HashMap::new();

    let rows = layout.len();
    let cols = layout[0].len();
    let mut r: Layout = vec![vec![Flr; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            if layout[i][j] == Flr {
                r[i][j] = Flr;
                continue;
            }

            dir_occ_map.clear();

            let mut radius = 1;
            while dir_occ_map.len() != dirs.len() {
                for dir in dirs.iter() {
                    if !dir_occ_map.contains_key(dir) {
                        let ii = i as i32 + (dir.0 * radius);
                        let jj = j as i32 + (dir.1 * radius);
                        if ii < 0 || ii >= rows as i32 || jj < 0 || jj >= cols as i32 {
                            dir_occ_map.insert(*dir, false);
                        } else {
                            match layout[ii as usize][jj as usize] {
                                Occ => { dir_occ_map.insert(*dir, true); },
                                Free => { dir_occ_map.insert(*dir, false); },
                                _ => {},
                            };
                        }
                    }
                }
                radius += 1;
            }

            let num_occ = dir_occ_map.values().filter(|is_occ| **is_occ).count();
            
            r[i][j] = match layout[i][j] {
                Occ => if num_occ >= 5 { Free } else { Occ },
                Free => if num_occ == 0 { Occ } else {Free },
                _ => Flr,
            };
        }
    }
    return r;
}

fn vec2d_equals(a: &Vec<Vec<Pos>>, b: &Vec<Vec<Pos>>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i].len() != b[i].len() {
            return false;
        }
        for j in 0..a[i].len() {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    return true;
}

fn vec2d_count<T: Eq>(v: &Vec<Vec<T>>, t: &T) -> usize {
    return v.iter()
        .map(|vv| vv.iter().filter(|val| *val == t).count())
        .sum();
}

fn layout_to_string(layout: &Layout) -> String {
    layout.iter()
        .map(|row| 
            row.iter()
                .map(|pos| pos.to_string())
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "\
            L.LL.LL.LL\n\
            LLLLLLL.LL\n\
            L.L.L..L..\n\
            LLLL.LL.LL\n\
            L.LL.LL.LL\n\
            L.LLLLL.LL\n\
            ..L.L.....\n\
            LLLLLLLLLL\n\
            L.LLLLLL.L\n\
            L.LLLLL.LL";

        let layout = super::to_layout(&input);

        let count = super::part1(&layout);
        assert_eq!(count, 37);
    }

    #[test]
    fn test_part2() {
        let input = "\
            L.LL.LL.LL\n\
            LLLLLLL.LL\n\
            L.L.L..L..\n\
            LLLL.LL.LL\n\
            L.LL.LL.LL\n\
            L.LLLLL.LL\n\
            ..L.L.....\n\
            LLLLLLLLLL\n\
            L.LLLLLL.L\n\
            L.LLLLL.LL";

        let layout = super::to_layout(&input);

        let count = super::part2(&layout);
        assert_eq!(count, 26);
    }
}