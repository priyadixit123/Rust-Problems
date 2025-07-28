https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/description/?envType=daily-question&envId=2025-07-28
SOL:

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        fn dfs(
            nums: &Vec<i32>, 
            index: usize, 
            current_or: i32, 
            max_or: i32, 
            count: &mut i32, 
            max_val: &mut i32
        ) {
            if index == nums.len() {
                if current_or == *max_val {
                    *count += 1;
                } else if current_or > *max_val {
                    *max_val = current_or;
                    *count = 1;
                }
                return;
            }

            // Include current index
            dfs(nums, index + 1, current_or | nums[index], max_or, count, max_val);

            // Exclude current index
            dfs(nums, index + 1, current_or, max_or, count, max_val);
        }

        let mut max_val = 0;
        let mut count = 0;
        dfs(&nums, 0, 0, 0, &mut count, &mut max_val);
        count
    }
}
