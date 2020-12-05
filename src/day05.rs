use super::aoc;

pub fn solve() {
    println!("Day 05");

    let input = std::fs::read_to_string("input/day05/input.txt").expect("peut");

    let mut taken = [false; 1024]; //[false; u32::pow(2, 10) as usize];

    let max = input.lines()
        .into_iter()
        .map(decode)
        .map(to_seat_id)
        .map(|i| { taken[i as usize] = true; i })
        .max();

    println!("max seat: {}", max.unwrap());

    for i in 2..taken.len() {
        if taken[i-2] && !taken[i-1] && taken[i] {
            println!("empty seat: {}", i-1);
        }
    }
}

fn decode(bp: &str) -> aoc::P2D {
    let row = isize::from_str_radix(
        &bp[0..7]
            .replace("F", "0")
            .replace("B", "1"), 
        2)
        .unwrap();
    let col = isize::from_str_radix(
    &bp[7..]
        .replace("L", "0")
        .replace("R", "1"), 
    2)
    .unwrap();
    return aoc::P2D {x: row as i32, y: col as i32};
}

fn to_seat_id(seat: aoc::P2D) -> u32 {
    return (seat.x * 8 + seat.y) as u32;
}
