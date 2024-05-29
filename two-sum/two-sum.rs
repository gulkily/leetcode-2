use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_indices = HashMap::new();  // HashMap to store indices of numbers
        
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&complement_index) = num_indices.get(&complement) {
                return vec![complement_index as i32, i as i32];
            }
            num_indices.insert(num, i);
        }
        
        // Since the problem guarantees exactly one solution, we don't need to return an empty vector.
        // We add this line to satisfy the return type, but it should never be reached.
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result); // Output should be [0, 1]
}
