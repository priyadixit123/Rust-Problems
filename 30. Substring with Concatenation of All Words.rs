https://leetcode.com/problems/substring-with-concatenation-of-all-words/
SOL

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let total_len = word_len * words.len();
        let s_len = s.len();
        let mut result = Vec::new();

        if s_len < total_len {
            return result;
        }

        let word_count: HashMap<&str, i32> = {
            let mut m = HashMap::new();
            for word in &words {
                *m.entry(word.as_str()).or_insert(0) += 1;
            }
            m
        };

        for i in 0..=s_len - total_len {
            let mut seen: HashMap<&str, i32> = HashMap::new();
            let mut j = 0;
            while j < words.len() {
                let start = i + j * word_len;
                let end = start + word_len;
                let word = &s[start..end];
                if !word_count.contains_key(word) {
                    break;
                }
                *seen.entry(word).or_insert(0) += 1;
                if seen[word] > word_count[word] {
                    break;
                }
                j += 1;
            }
            if j == words.len() {
                result.push(i as i32);
            }
        }

        result
    }
}
