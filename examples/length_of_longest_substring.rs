/*
Given a string s, find the length of the longest substring
without repeating characters.

Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.

Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

Example 4:

Input: s = ""
Output: 0

Constraints:

    0 <= s.length <= 5 * 104
    s consists of English letters, digits, symbols and spaces.

*/

fn main() {
    let s = "pwwkew".to_string();
    let sol = Solution::length_of_longest_substring(s);
    let expected = 3;

    assert_eq!(expected, sol)
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // "aab"
        // "abcabcbb"
        // "bbbbb"
        // "pwwkew"
        // ""

        if s.len() == 0 {
            return 0;
        }

        let chars = s.as_bytes();

        let mut counter = 0;
        let mut tmp_counter: i32 = 1;
        let mut i = 0;
        let mut j = 0;
        let mut window_just_slid = false;
        while i < s.len() {
            if !window_just_slid || i == j {
                j += 1
            };

            if j == s.len() {
                if tmp_counter > counter {
                    counter = tmp_counter;
                }
                break;
            }

            let mut repeat = false;
            for k in i..j {
                if chars[k] == chars[j] {
                    repeat = true;
                    break;
                }
            }

            if repeat {
                window_just_slid = true;
                if tmp_counter > counter {
                    counter = tmp_counter;
                }
                i += 1;
                tmp_counter = (j - i) as i32;
                if tmp_counter == 0 {
                    tmp_counter = 1;
                }
            } else {
                window_just_slid = false;
                tmp_counter += 1;
            }
        }
        counter
    }
}
