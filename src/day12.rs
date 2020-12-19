//#![allow(dead_code)]

type N = f32;
type P = super::aoc::P2D<N>;

fn pan(p: &P, dir: &P, amount: N) -> P {
    p.add(
        dir.x * amount,
        dir.y * amount)
}

fn pan_boat(boat: &Boat, dir: &P, amount: N) -> Boat {
    Boat {
        position: pan(&boat.position, dir, amount),
        heading: boat.heading,
    }
}

fn pan_wp(boat: &BoatWP, dir: &P, amount: N) -> BoatWP {
    BoatWP {
        waypoint: pan(&boat.waypoint, dir, amount),
        position: boat.position,
    }
}

fn turn_boat(boat: &Boat, deg: N) -> Boat {
    Boat {
        position: boat.position,
        heading: (boat.heading + deg + 360 as N) % (360 as N),
    }
}

fn turn_wp(boat: &BoatWP, deg: N) -> BoatWP {
    let wp = boat.waypoint.rotate_deg(deg);
    BoatWP {
        waypoint: P{
            x: wp.x as N,
            y: wp.y as N,
        },
        position: boat.position,
    }
}

fn travel(boat: &Boat, amount: N) -> Boat {
    Boat {
        position: boat.position.add(
            (boat.heading as f32).to_radians().sin() * amount,
            (boat.heading as f32).to_radians().cos() * amount),
        heading: boat.heading,
    }
}

fn travel_wp(boat: &BoatWP, amount: N) -> BoatWP {
    BoatWP {
        waypoint: boat.waypoint,
        position: boat.position.add(boat.waypoint.x * amount, boat.waypoint.y * amount),
    }
}

#[derive(Clone)]
struct Boat {
    position: P,
    heading: N, //in degrees
}

#[derive(Clone)]
struct BoatWP {
    position: P,
    waypoint: P,
}

impl std::fmt::Display for Boat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "position:{} heading:{}", self.position, self.heading)
    }
}

impl std::fmt::Display for BoatWP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "position:{} waypoint:{}", self.position, self.waypoint)
    }
}

fn do_op(op: &str, boat: &Boat) -> Boat {
    let code = &op[0..1];
    let amount = &op[1..].parse::<i32>().unwrap();

    match code {
        "N" => pan_boat(boat, &P::new(0 as N, -1 as N), *amount as N),
        "S" => pan_boat(boat, &P::new(0 as N, 1 as N), *amount as N),
        "E" => pan_boat(boat, &P::new(1 as N, 0 as N), *amount as N),
        "W" => pan_boat(boat, &P::new(-1 as N, 0 as N), *amount as N),
        "L" => turn_boat(boat, *amount as N),
        "R" => turn_boat(boat, -*amount as N),
        "F" => travel(boat, *amount as N),
        _ => boat.clone(),
    }
}

fn do_op2(op: &str, boat: &BoatWP) -> BoatWP {
    let code = &op[0..1];
    let amount = &op[1..].parse::<i32>().unwrap();

    match code {
        "N" => pan_wp(boat, &P::new(0 as N, -1 as N), *amount as N),
        "S" => pan_wp(boat, &P::new(0 as N, 1 as N), *amount as N),
        "E" => pan_wp(boat, &P::new(1 as N, 0 as N), *amount as N),
        "W" => pan_wp(boat, &P::new(-1 as N, 0 as N), *amount as N),
        "L" => turn_wp(boat, *amount as N),
        "R" => turn_wp(boat, -*amount as N),
        "F" => travel_wp(boat, *amount as N),
        _ => boat.clone(),
    }
}

pub fn solve() {
    println!("Day 12");

    let input = std::fs::read_to_string("input/day12/input.txt").expect("peut");

    let r1 = part1(&input);
    println!("part 1: {}", r1);

    let r2 = part2(&input);
    println!("part 2: {}", r2);
}

fn part1(input: &String) -> N {
    let mut boat = Boat {
        position: P{ x: 0 as N, y: 0 as N, },
        heading: 90 as N,
    };
    for line in input.lines() {
        boat = do_op(line, &boat);
        println!("{}", boat);
    }

    boat.position.manhattan_distance(0 as N, 0 as N).round() as N
}

fn part2(input: &String) -> N {
    let mut boat = BoatWP {
        waypoint: P{ x: 10 as N, y: -1 as N, },
        position: P{ x: 0 as N, y: 0 as N},
    };
    for line in input.lines() {
        boat = do_op2(line, &boat);
        println!("{}", boat);
    }

    boat.position.manhattan_distance(0 as N, 0 as N).round() as N
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = String::from("\
        F10\n\
        N3\n\
        F7\n\
        R90\n\
        F11");

        let distance = super::part1(&input);
        assert_eq!(distance.round() as i64, 25 as i64);
    }

    #[test]
    fn test_part2() {
        let input = String::from("\
        F10\n\
        N3\n\
        F7\n\
        R90\n\
        F11");

        let distance = super::part2(&input);
        assert_eq!(distance.round() as i64, 286 as i64);
    }
}