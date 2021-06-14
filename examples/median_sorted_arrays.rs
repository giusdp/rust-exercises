/*
Given two sorted arrays nums1 and nums2 of size m and n respectively,
return the median of the two sorted arrays.
The overall run time complexity should be O(log (m+n)).

Example 1:

Input: nums1 = [1,3], nums2 = [2]
Output: 2.00000
Explanation: merged array = [1,2,3] and median is 2.

Example 2:

Input: nums1 = [1,2], nums2 = [3,4]
Output: 2.50000
Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.

Example 3:

Input: nums1 = [0,0], nums2 = [0,0]
Output: 0.00000

Example 4:

Input: nums1 = [], nums2 = [1]
Output: 1.00000

Example 5:

Input: nums1 = [2], nums2 = []
Output: 2.00000

Constraints:

    nums1.length == m
    nums2.length == n
    0 <= m <= 1000
    0 <= n <= 1000
    1 <= m + n <= 2000
    -1^6 <= nums1[i], nums2[i] <= 10^6
*/

fn main() {
    let nums1 = vec![1, 2, 5];
    let nums2 = vec![3, 4];
    let sol = Solution::find_median_sorted_arrays(nums1, nums2);
    let expected = 2.5;

    assert_eq!(expected, sol)
}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() == 0 && nums2.len() == 0 {
            return 0.0;
        } else if nums1.len() == 1 && nums2.len() == 0 {
            return nums1[0] as f64;
        } else if nums1.len() == 0 && nums2.len() == 1 {
            return nums2[0] as f64;
        }

        let length: i32 = (nums1.len() + nums2.len()) as i32;

        let mut nums1 = nums1;
        let mut nums2 = nums2;
        let sol = Solution::fill(&mut nums1, &mut nums2, vec![], 0, length);

        let m = length / 2;
        if length % 2 == 0 {
            ((sol[(m - 1) as usize] + sol[m as usize]) as f64) / 2.0
        } else {
            sol[m as usize] as f64
        }
    }

    fn fill(
        nums1: &mut Vec<i32>,
        nums2: &mut Vec<i32>,
        mut sol: Vec<i32>,
        l: i32,
        h: i32,
    ) -> Vec<i32> {
        // println!("{} {} {}", l, h, (h + l) / 2);
        if h - l < 2 {
            // println!("Caso base: {} {}", l, h);

            if nums1.len() > 0 && nums2.len() > 0 {
                if nums1[0] > nums2[0] {
                    sol.push(nums1.swap_remove(0));
                } else {
                    sol.push(nums2.swap_remove(0));
                }
            } else if nums1.len() > 0 {
                sol.push(nums1.swap_remove(0));
            } else if nums2.len() > 0 {
                sol.push(nums2.swap_remove(0));
            }

            return sol;
        }
        let m = (h + l) / 2;

        sol = Solution::fill(nums1, nums2, sol, l, m);
        sol = Solution::fill(nums1, nums2, sol, l + m, h);

        sol
    }
}

/* Basically mergesort solution

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        // 1, 3, 4
        // 2, 5
        // _ _ _ _ _

        if nums1.len() == 0 && nums2.len() == 0 {
            return 0.0;
        } else if nums1.len() == 1 && nums2.len() == 0 {
            return nums1[0] as f64;
        } else if nums1.len() == 0 && nums2.len() == 1 {
            return nums2[0] as f64;
        }
        let v = Solution::merge(nums1, nums2);

        let m = v.len() / 2;
        if v.len() % 2 == 0 {
            ((v[m - 1] + v[m]) as f64) / 2.0
        // } else if v.len() == 3 {
        //     v[m] as f64
        // } else {
        } else {
            v[m] as f64
        }
    }

    fn merge(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() < 2 && nums2.len() < 2 {
            if nums1.len() == 0 || nums2.len() == 0 {
                nums1.append(&mut nums2);
                return nums1;
            } else if nums1.len() == 1 && nums2.len() == 1 {
                if nums1[0] < nums2[0] {
                    nums1.append(&mut nums2);
                    return nums1;
                } else {
                    nums2.append(&mut nums1);
                    return nums2;
                }
            }
        }
        let m1 = nums1.len() / 2;
        let m2 = nums2.len() / 2;

        let mut left = Solution::merge(
            nums1[0..m1].to_vec(),
            nums2[0..m2].to_vec(),
        );
        let mut right = Solution::merge(
            nums1[m1..nums1.len()].to_vec(),
            nums2[m2..nums2.len()].to_vec(),
        );

        left.append(&mut right);
        left
    }
}
*/
