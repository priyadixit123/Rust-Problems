https://leetcode.com/problems/sqrtx/ 

SOL:

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }

        let mut left = 1;
        let mut right = x / 2;
        let mut ans = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_squared = mid as i64 * mid as i64; // Use i64 to avoid overflow

            if mid_squared == x as i64 {
                return mid;
            } else if mid_squared < x as i64 {
                ans = mid; // mid is a valid candidate
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans
    }
}
