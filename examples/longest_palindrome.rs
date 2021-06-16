/*
Given a string s, return the longest palindromic substring in s.

Example 1:

Input: s = "babad"
Output: "bab"
Note: "aba" is also a valid answer.

Example 2:

Input: s = "cbbd"
Output: "bb"

Example 3:

Input: s = "a"
Output: "a"

Example 4:

Input: s = "ac"
Output: "a"

Constraints:
    1 <= s.length <= 1000
    s consist of only digits and English letters (lower-case and/or upper-case),
*/

fn main() {
    let s = "ccc".to_string();
    let sol = Solution::longest_palindrome(s);
    let expected = "ccc";

    assert_eq!(expected, sol)
}

struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 1 {
            return String::from("");
        }
        
        let bytes = s.as_bytes();
        let (mut start, mut end) = (0, 0);
        
        for i in 0..bytes.len() {
            let (l1, r1) = expand_around_center(bytes, i, i);
            let (l2, r2) = expand_around_center(bytes, i, i + 1);
            
            let len1 = r1 - l1 + 1;
            let len2 = r2 - l2 + 1;
            let cur_len = end - start + 1;
            
            if len1 >= len2 && len1 > cur_len {
                start = l1;
                end = r1;
            } else if len2 > len1 && len2 > cur_len {
                start = l2;
                end = r2;
            }
        }
        
        String::from_utf8(bytes[start..=end].to_vec()).unwrap()
    }
}

fn expand_around_center(s: &[u8], left: usize, right: usize) -> (usize, usize) {
    
    let (mut l, mut r) = (left, right);
    let (mut start, mut end) = (0, 0);
    
    let len = s.len();
    
    while r < len && s[l] == s[r] {
        start = l;
        end = r;
        
        if l == 0 {
            break;
        }
        
        l -= 1;
        r += 1;
    }
    (start, end)
}
