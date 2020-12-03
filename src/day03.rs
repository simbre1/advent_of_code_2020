
pub fn solve() {
    println!("Day 03");

    let input = std::fs::read_to_string("input/day03/input.txt").expect("peut");
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let mut map = vec![vec![false; cols]; rows];

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            map[row][col] = c == '#';
        }
    }

    println!("num trees: {}", count_trees(&map, 0, 0, 3, 1));

    println!(
        "product trees: {}",
        ( 
              count_trees(&map, 0, 0, 1, 1)
            * count_trees(&map, 0, 0, 3, 1)
            * count_trees(&map, 0, 0, 5, 1)
            * count_trees(&map, 0, 0, 7, 1)
            * count_trees(&map, 0, 0, 1, 2)
        )
    );
    
}

fn count_trees(map: &Vec<Vec<bool>>, x_start: u32, y_start: u32, x_velocity: u32, y_veloticy: u32) -> u32 {
    let mut num_trees = 0;
    let mut x: u32 = x_start % (map[0].len() as u32);
    let mut y: u32 = y_start;

    while y < map.len() as u32 {
        if map[y as usize][x as usize] {
            num_trees += 1;
        }

        y += y_veloticy;
        x = (x + x_velocity) % map[0].len() as u32
    }
    return num_trees;
}