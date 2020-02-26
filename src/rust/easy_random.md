# Random

## 20.valid-parentheses
```rust
impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::VecDeque;
        let mut r = true;
        let mut stk = VecDeque::<char>::new();
        s.chars().for_each(|x| match x {
            '(' | '{' | '[' => stk.push_back(x),
            ')' => r = (stk.pop_front() == Some('(')) && r,
            '}' => r = (stk.pop_front() == Some('{')) && r,
            ']' => r = (stk.pop_front() == Some('[')) && r,
            _ => r = false,
        });
        r && stk.is_empty()
    }
}
```

## 50.powx-n
```rust
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
```
