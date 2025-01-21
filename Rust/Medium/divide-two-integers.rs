/*
    Runtime : 0ms    (Beats 100%)
    Memory  : 2.32MB  (Beats --.--%)
*/

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX; 
        }
        dividend / divisor
    }
}