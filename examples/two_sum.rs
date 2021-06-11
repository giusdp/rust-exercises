use std::collections::HashMap;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let sol = Solution::two_sum(nums, target);
    assert_eq!(sol, vec![0, 1])
}

struct Solution;

impl Solution {

    // 1 pass hashmap
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut numbers_table: HashMap<i32, i32> = HashMap::new();

        for (i, e) in nums.iter().enumerate() {
            let compl = target - e;
            if numbers_table.contains_key(&compl) {
                let first = numbers_table[&compl].to_owned();
                return vec![first, i as i32];
            } else {
                numbers_table.insert(*e, i as i32);
            }
        }

        vec![]
    }

    // 2 pass hashmap
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut numbers_table: HashMap<i32, i32> = HashMap::new();

    //     for (i, e) in nums.iter().enumerate() {
    //         numbers_table.insert(*e, i as i32);
    //     }

    //     for (i, _) in nums.iter().enumerate() {
    //         let compl = target - nums[i];
    //         if numbers_table.contains_key(&compl) {
    //             let second = numbers_table[&compl].to_owned();
    //             if second != i as i32 {
    //                 return vec![i as i32, second];
    //             }
    //         }
    //     }
    //     vec![]
    // }

    // Naive impl O(n^2)
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     for (i, _) in nums.iter().enumerate() {
    //         for (j, _) in nums.iter().enumerate() {
    //             if i != j && nums[i] + nums[j] == target {
    //                 return vec![i as i32,j as i32];
    //             }
    //         }
    //     }
    //     vec![]
    // }
}
