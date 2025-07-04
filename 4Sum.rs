https://leetcode.com/problems/4sum/

SOL:

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let n = nums.len();
        if n < 4 {
            return res;
        }
        
        nums.sort(); // Sort the array for easy duplicate skipping and two-pointer use

        for i in 0..n-3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicate i
            }
            for j in i+1..n-2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue; // skip duplicate j
                }
                let mut left = j + 1;
                let mut right = n - 1;

                while left < right {
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;

                    if sum == target as i64 {
                        res.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                        // Skip duplicates for left
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        // Skip duplicates for right
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }

                        left += 1;
                        right -= 1;
                    } else if sum < target as i64 {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }

        res
    }
}
