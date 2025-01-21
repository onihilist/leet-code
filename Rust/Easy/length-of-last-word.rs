/*
    Runtime : 0ms    (Beats 100%)
    Memory  : 2.25MB  (Beats 65.11%)
*/

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut tmp = s.trim_end().split(" ");
        return tmp.last().unwrap().len() as i32
    }
}