https://leetcode.com/problems/longest-valid-parentheses/description/

SOL:

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![-1]; // Stack to store indices
        let mut max_len = 0;

        for (i, ch) in s.chars().enumerate() {
            if ch == '(' {
                stack.push(i as i32);
            } else {
                stack.pop(); // Pop the matching '(' or -1

                if let Some(&last) = stack.last() {
                    max_len = max_len.max(i as i32 - last);
                } else {
                    stack.push(i as i32); // Reset base index
                }
            }
        }

        max_len
    }
}
