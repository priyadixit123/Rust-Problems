https://leetcode.com/problems/count-and-say/description/

SOL:

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = String::from("1");

        for _ in 1..n {
            let mut temp = String::new();
            let mut chars = result.chars().peekable();
            
            while let Some(current_char) = chars.next() {
                let mut count = 1;
                while let Some(&next_char) = chars.peek() {
                    if next_char == current_char {
                        chars.next();
                        count += 1;
                    } else {
                        break;
                    }
                }
                temp.push_str(&count.to_string());
                temp.push(current_char);
            }

            result = temp;
        }

        result
    }
}
