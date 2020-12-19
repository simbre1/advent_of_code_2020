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

#[derive(Clone, Copy)]
pub struct P2D<T> {
    pub x: T,
    pub y: T,
}

impl <T> P2D<T> 
where
    T: std::ops::Add<Output = T>, 
    T: std::ops::Sub<Output = T>,
    T: std::ops::Mul<Output = T>, 
    T: std::convert::Into::<f64>,
    T: Copy
{

    pub fn new(x: T, y: T) -> Self { 
        Self{ x, y }
    }

    pub fn add(&self, x: T, y: T) -> P2D<T> {
        P2D{
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn manhattan_distance(&self, x: T, y: T) -> f64 {
        T::into(self.x - x).abs() + T::into(self.y - y).abs()
    }

    pub fn rotate_deg<U: std::convert::Into<f64>>(&self, angle: U) -> P2D<f64> {
        let rad = U::into(angle).to_radians();
        let cos = rad.cos();
        let sin = rad.sin();
        P2D::<f64> {
            x: (T::into(self.x) * cos) + (T::into(self.y) * sin),
            y: (T::into(self.y) * cos) - (T::into(self.x) * sin)        }
    }
}

impl <T: std::fmt::Display> fmt::Display for P2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2d_rotate_deg() {
        let p = P2D::<i32> { x: 7, y: 7 };
        let p90 = p.rotate_deg(-90f64);
        assert_eq!(p90.x.round() as i32, -7);
        assert_eq!(p90.y.round() as i32, 7);
    }

    #[test]
    fn test_p2d_rotate_deg_360() {
        let p = P2D::<i32> { x: 7, y: 7 };
        let p90 = p.rotate_deg(720f64);
        assert_eq!(p90.x.round() as i32, 7);
        assert_eq!(p90.y.round() as i32, 7);
    }
}