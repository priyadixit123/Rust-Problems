5. Longest Palindromic Substring
https://leetcode.com/problems/longest-palindromic-substring/description/


SOL:

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n == 0 {
            return String::new();
        }

        let mut start = 0;
        let mut end = 0;

        for i in 0..n {
            let len1 = Self::expand_around_center(&chars, i, i);     // odd-length
            let len2 = Self::expand_around_center(&chars, i, i + 1); // even-length
            let len = len1.max(len2);

            if len > (end - start) {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }

        chars[start..=end].iter().collect()
    }

    fn expand_around_center(s: &[char], left: usize, right: usize) -> usize {
        let (mut l, mut r) = (left as isize, right as isize);
        while l >= 0 && (r as usize) < s.len() && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }
        (r - l - 1) as usize // palindrome length
    }
}
