
struct Solution();

impl Solution {
    /*
Input: nums = [1,2,3,4]
Output: [2,4,4,4]
Explanation: The first pair [1,2] means we have freq = 1 and val = 2 so we generate the array [2].
The second pair [3,4] means we have freq = 3 and val = 4 so we generate [4,4,4].
At the end the concatenation [2] + [4,4,4] is [2,4,4,4].
     */
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        /*
        let mut v = Vec::new();
        let middle = nums.len()/2;
        for i in 0..middle {
            for j in 0..nums[i*2] {
                v.push(nums[i*2+1]);
            }
        }
        v
         */

        let mut v = vec!();
        for ch in nums.chunks(2) {
            v.extend(vec![ch[1];ch[0] as usize].iter());
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_decompress_rl_elist() {
        let input = vec![1,1,2,3];
        let output = vec![1,3,3];
        assert_eq!(Solution::decompress_rl_elist(input), output);
    }
}