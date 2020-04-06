#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |res, num| res ^ num)
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
