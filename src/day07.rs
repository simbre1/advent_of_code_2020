#![allow(dead_code)]
use std::collections::HashMap;

struct Content {
    color: String,
    count: u32,
}

struct Bag {
    color: String,
    children: Vec<Content>,
}

pub fn solve() {
    println!("Day 07");

    let input = std::fs::read_to_string("input/day07/input.txt").expect("peut");

    let mut bags: HashMap<String, Bag> = HashMap::new();
    for line in input.lines() {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();

        let color = format!("{} {}", words[0], words[1]);

        let mut content: Vec<Content> = Vec::new();
        if words.len() > 7 {
            for i in 1..(words.len() / 4) {
                let j = i * 4;
                let child = Content {
                    count: words[j].parse::<u32>().unwrap(),
                    color: format!("{} {}", words[j+1], words[j+2]),
                };
                content.push(child);
            }
        }
        let bag = Bag {
            color: color.clone(),
            children: content,
        };
        bags.insert(color, bag);
    }

    println!("count 1: {}", bags.keys().filter(|color| bag_contains(&bags, &String::from("shiny gold"), color)).count());
    println!("count 2: {}", bag_count_children(&bags, &String::from("shiny gold")));
}

fn bag_contains(bags: &HashMap<String, Bag>, needle: &String, haystack: &String) -> bool {
    return match bags.get(haystack) {
        Some(bag) => {
            for child in &bag.children {
                if child.color == *needle || bag_contains(bags, needle, &child.color) {
                    return true;
                }
            }
            return false;
        },
        None => false
    }
}

fn bag_count_children(bags: &HashMap<String, Bag>, color: &String) -> u32 {
    match bags.get(color) {
        Some(bag) => bag.children.iter().map(|child| (child.count * (1 + bag_count_children(bags, &child.color)))).sum(),
        None => 0
    }
}