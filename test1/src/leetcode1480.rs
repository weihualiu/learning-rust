use std::collections::HashMap;

struct Solution();

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut vec:Vec<i32> = Vec::new();
        let mut val = 0;
        for i in nums.iter() {
            val+=i;
            vec.push(val);
        }
        vec
    }

    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // 第i个数值+扩展值大于等于数组值为true
        let mut vec:Vec<bool> = Vec::new();
        let mut max = 0;
        for val in candies.iter() {
            if max <= *val {
                max = *val;
            }
        }
        for val in candies.iter() {
            if val + extra_candies >= max {
                vec.push(true);
            }else {
                vec.push(false);
            }
        }
        vec
    }

    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut vec = Vec::new();
        for i in 0..(nums.len()-1) {
            vec.push(nums[i]);
            vec.push(nums[nums.len()/2+i]);
        }
        vec
    }

    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }

    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        //let mut count = 0;
        let mut result = 0;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let x = nums[i] - 1;
            let v = match map.get(&x) {
                Some(v1) => *v1,
                None => 0 as i32,
            };
            result = result + v;
            map.insert(x, v + 1);
        }

        result
    }

    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        true
    }

    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut count = 0;
        let mut map:HashMap<u8, i32> = HashMap::new();
        for v in s.as_bytes() {
            let value = map.get(v).unwrap_or(&0);
            map.insert(*v, value + 1);
        }
        for v in j.as_bytes() {
            count = count + map.get(v).unwrap_or(&0);
        }
        count
    }

    pub fn number_of_steps (num: i32) -> i32 {
        let mut count = 0;
        let mut temp = num;
        while temp > 0 {
            if temp % 2 == 0 {
                temp = temp / 2;
            } else {
                temp = temp - 1;
            }
            count = count + 1;
        }
        count
    }
}