
struct Solution();

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut new_array = Vec::new();
        // 统计在当前索引之后的所有小于该值的计数，然后输出在新数组中
        let len = nums.len();
        for i in 0 .. len {
            let mut count = 0;
            for j in 0..len {
                if nums[i] > nums[j]  {
                    count = count +1;
                }
            }
            new_array.insert(i,count);
        }

        new_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t_smaller_numbers_than_current(){
        let v = vec![4,0,1,1,3];
        let v1 = vec![8,1,2,2,3];
        assert_eq!(v, Solution::smaller_numbers_than_current(v1));
    }
}