impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        use std::i32::MIN;
        if n == 0 {
            return 1.0;
        }

        if n < 0 {
            return 1.0 / Self::my_pow(x, -n);
        }

        if n % 2 == 0 {
            return Self::my_pow(x * x, n / 2);
        } else {
            return x * Self::my_pow(x * x, n / 2);
        }
    }
}
