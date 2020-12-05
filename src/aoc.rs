use std::collections::HashMap;
use std::fmt;

pub fn get_char_counts(s: &String) -> HashMap<char, u8> {
    let mut m: HashMap<char, u8> = HashMap::new();
    for c in s.chars() {
        let n = match m.get(&c) {
            None => 1,
            Some(n) => n+1,
        };
        m.insert(c, n);
    }
    return m;
}    

pub struct P2D {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for P2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}