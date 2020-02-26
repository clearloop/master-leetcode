impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        use std::i32::MIN;
        match n {
            o if o == 0 => 1.0,
            o if o == MIN => Self::my_pow(x * x, MIN / 2),
            o if o < 0 => 1.0 / Self::my_pow(x, -n),
            _ => match n % 2 {
                0 => Self::my_pow(x * x, n / 2),
                _ => x * Self::my_pow(x * x, n / 2),
            },
        }
    }
}
