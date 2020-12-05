#![allow(dead_code)]
use std::collections::HashMap;
use phf::{phf_map};
use regex::Regex;

fn not_required(_: Option<&String>) -> bool {
    return true;
}

// fn required(o: Option<&String>) -> bool {
//     return match o {
//         None => false,
//         Some(s) => !s.is_empty(),
//     };
// }

fn int_between(o: Option<&String>, min: i32, max: i32) -> bool {
    if o == None {
        return false;
    } else {
        return match o.unwrap().parse::<i32>() {
            Ok(i) => i >= min && i <= max,
            Err(_) => false,
        };
    }
}

fn height(o: Option<&String>) -> bool {
    if o == None {
        return false;
    } else {
        let s = o.unwrap();
        let inch = s.find("in");
        if inch != None {
            return match (&s[0..inch.unwrap()]).parse::<i32>() {
                Ok(i) => i >= 59 && i <= 76,
                Err(_) => false,
            }
        }
        let cm = s.find("cm");
        return cm != None && match (&s[0..cm.unwrap()]).parse::<i32>() {
            Ok(i) => i >= 150 && i <= 193,
            Err(_) => false,
        }
    }
}

fn  regex(o: Option<&String>, regex: &str) -> bool {
    let s = match o {
        None => "",
        Some(s) => s,
    };

    let re = Regex::new(regex).unwrap();
    return re.is_match(s);    
}

static FIELDS: phf::Map<&'static str, fn(Option<&String>) -> bool> = phf_map!{
        "byr" => |o| int_between(o, 1920, 2002),
        "iyr" => |o| int_between(o, 2010, 2020),
        "eyr" => |o| int_between(o, 2020, 2030),
        "hgt" => height,
        "hcl" => |o| regex(o, "^#[0-9a-f]{6}$"),
        "ecl" => |o| regex(o, "^(amb|blu|brn|gry|grn|hzl|oth)$"),
        "pid" => |o| regex(o, "^[0-9]{9}$"),
        "cid" => not_required,
};

struct Pp {
    fields: std::collections::HashMap<String, String>,
}

impl Pp {
    fn is_valid(&self) -> bool {
        for (field, pred) in FIELDS.entries() {
            if !pred(self.fields.get(&field.to_string())) {
                return false;
            }
        }
        return true;
    }
}

pub fn solve() {
    println!("Day 04");

    let input = std::fs::read_to_string("input/day04/input.txt").expect("peut");

    let mut pps: Vec<Pp> = Vec::new();

    let mut fields: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        for field in line.split_whitespace() {
            let mut split = field.split(":");
            let k = split.next().unwrap().to_string();
            let v = split.next().unwrap().to_string();
            fields.insert(k, v);
        }

        if line.len() == 0 {            
            let pp = Pp {
                fields: fields.clone(),
            };
            pps.push(pp);

            fields.clear();
        }
    }
    if !fields.is_empty() {
        let pp = Pp {
            fields: fields.clone(),
        };
        pps.push(pp);
    }
    
    solve_impl(&pps);
}

fn solve_impl(pps: &Vec<Pp>) {
    println!("# valid: {}", pps.iter().filter(|pp| pp.is_valid()).count());
}