https://leetcode.com/problems/minimum-difference-in-sums-after-removal-of-elements/description/?envType=daily-question&envId=2025-07-18

SOL:

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len() / 3;
        let mut left_heap = BinaryHeap::new();
        let mut right_heap = BinaryHeap::new();
        let mut left_sum = 0i64;
        let mut right_sum = 0i64;

        let mut left_prefix = vec![0i64; nums.len()];
        let mut right_suffix = vec![0i64; nums.len()];

        // Process left side: track smallest n sums for prefix
        for i in 0..2 * n {
            left_sum += nums[i] as i64;
            left_heap.push(nums[i] as i64);
            if left_heap.len() > n {
                if let Some(max_val) = left_heap.pop() {
                    left_sum -= max_val;
                }
            }
            if left_heap.len() == n {
                left_prefix[i] = left_sum;
            }
        }

        // Process right side: track largest n sums for suffix (use min-heap)
        for i in (n..3 * n).rev() {
            right_sum += nums[i] as i64;
            right_heap.push(Reverse(nums[i] as i64));
            if right_heap.len() > n {
                if let Some(Reverse(min_val)) = right_heap.pop() {
                    right_sum -= min_val;
                }
            }
            if right_heap.len() == n {
                right_suffix[i] = right_sum;
            }
        }

        // Calculate minimum difference
        let mut result = i64::MAX;
        for i in n - 1..2 * n {
            let left = left_prefix[i];
            let right = right_suffix[i + 1];
            result = result.min(left - right);
        }

        result
    }
}
