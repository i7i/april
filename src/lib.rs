#[cfg(test)]
use std::collections::HashSet;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        let mut single: Option<i32> = None;
        for n in nums {
            single = match set.get(&n) {
                Some(_) => continue,
                None => {
                    set.insert(n);
                    Some(n)
                }
            };
        }

        match single {
            Some(n) => n,
            None => panic!("no duplicate values in vec"),
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
