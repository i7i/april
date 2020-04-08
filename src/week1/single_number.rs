#[cfg(test)]
pub struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn single_number(nums: &Vec<i32>) -> i32 {
        nums.iter().fold(0, |res, num| res ^ num)
    }

    pub fn two_single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let all = Solution::single_number(&nums);
        let lsb = Solution::lsb(all);
        let mut set_a = Vec::new();
        let mut set_b = Vec::new();

        nums.iter().for_each(|num| {
            if num & (1 << lsb) != 0 {
                set_a.push(*num);
            } else {
                set_b.push(*num);
            }
        });
        vec![
            Solution::single_number(&set_a),
            Solution::single_number(&set_b),
        ]
    }

    pub fn lsb(num: i32) -> i32 {
        let a = num - 1;
        let b = num | a;
        (b ^ a) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_ex_1() {
        let nums = vec![2, 2, 1];
        let have = Solution::single_number(&nums);
        let want = 1;

        assert_eq!(want, have);
    }

    #[test]
    fn single_number_ex_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let have = Solution::single_number(&nums);
        let want = 4;

        assert_eq!(want, have);
    }

    #[test]
    fn two_single_numbers_ex_1() {
        let nums = vec![3, 4, 8, 2, 1, 2, 8, 3];
        let have = Solution::two_single_numbers(nums);

        assert!(have.contains(&4));
        assert!(have.contains(&1));
        assert_eq!(have.len(), 2);
    }
}
