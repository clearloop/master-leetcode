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
