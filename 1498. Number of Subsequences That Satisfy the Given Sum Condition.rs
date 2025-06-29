https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/description/?envType=daily-question&envId=2025-06-29

Sol:

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = nums.len();
        nums.sort();

        // Precompute powers of 2 modulo MOD
        let mut pow2 = vec![1; n];
        for i in 1..n {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
        }

        let (mut left, mut right) = (0, n - 1);
        let mut result = 0;

        while left <= right {
            if nums[left] + nums[right] <= target {
                result = (result + pow2[right - left]) % MOD;
                left += 1;
            } else {
                if right == 0 { break; } // prevent underflow
                right -= 1;
            }
        }

        result
    }
}
