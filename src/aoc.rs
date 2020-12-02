use std::collections::HashMap;

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
