https://leetcode.com/problems/longest-harmonious-subsequence/?envType=daily-question&envId=2025-06-30


SoL:


use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for &num in &nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut longest = 0;
        for (&key, &value) in &map {
            if let Some(&next_val) = map.get(&(key + 1)) {
                longest = longest.max(value + next_val);
            }
        }
        longest
    }
}
