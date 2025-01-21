/*
    Runtime : 0ms    (Beats 100%)
    Memory  : 2.20MB (Beats 70.35%)
*/

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut vec_temp: Vec<i32> = nums1.clone();
        vec_temp.extend(nums2.iter().cloned());

        vec_temp.sort();

        let len = vec_temp.len();
        if len % 2 == 0 {
            (vec_temp[len / 2 - 1] as f64 + vec_temp[len / 2] as f64) / 2.0
        } else {
            vec_temp[len / 2] as f64
        }
    }
}