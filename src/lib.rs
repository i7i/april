#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, u32> = HashMap::new();

        nums.iter().for_each(|num| {
            if let Some(count) = hm.get_mut(&num) {
                *count += 1;
            } else {
                hm.insert(*num, 0);
            }
        });

        hm.retain(|_, v| v == &0);
        if let Some(num) = hm.keys().cloned().collect::<Vec<i32>>().pop() {
            num
        } else {
            panic!("no duplicate values in vec");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_ex_1() {
        let nums = vec![2, 2, 1];
        let have = Solution::single_number(nums);
        let want = 1;

        assert_eq!(want, have);
    }

    #[test]
    fn single_number_ex_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let have = Solution::single_number(nums);
        let want = 4;

        assert_eq!(want, have);
    }
}
