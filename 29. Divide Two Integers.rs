https://leetcode.com/problems/divide-two-integers/
SOL:
 
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let max = i32::MAX;
        let min = i32::MIN;

        // Handle overflow
        if dividend == min && divisor == -1 {
            return max;
        }

        let mut dvd = (dividend as i64).abs();
        let dvr = (divisor as i64).abs();
        let mut result: i64 = 0;

        while dvd >= dvr {
            let mut temp = dvr;
            let mut multiple = 1;

            while dvd >= (temp << 1) {
                temp <<= 1;
                multiple <<= 1;
            }

            dvd -= temp;
            result += multiple;
        }

        let sign = if (dividend > 0) == (divisor > 0) { 1 } else { -1 };
        (result * sign) as i32
    }
}
