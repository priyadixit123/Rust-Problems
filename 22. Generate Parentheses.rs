https://leetcode.com/problems/generate-parentheses/description/

SOL:

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(current: String, open: i32, close: i32, max: i32, result: &mut Vec<String>) {
            if current.len() == (max * 2) as usize {
                result.push(current);
                return;
            }

            if open < max {
                backtrack(format!("{}(", current), open + 1, close, max, result);
            }
            if close < open {
                backtrack(format!("{})", current), open, close + 1, max, result);
            }
        }

        let mut result = Vec::new();
        backtrack(String::new(), 0, 0, n, &mut result);
        result
    }
}
