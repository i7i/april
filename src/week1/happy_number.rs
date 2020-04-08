pub(crate) use std::collections::HashSet;

/// Write an algorithm to determine if a number n is "happy".
///
/// A happy number is a number defined by the following process: Starting with any positive
/// integer, replace the number by the sum of the squares of its digits, and repeat the process
/// until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does
/// not include 1. Those numbers for which this process ends in 1 are happy numbers.
///
/// Return True if n is a happy number, and False if not.
pub struct HappyNumber {}

impl HappyNumber {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut num = n;

        while !(num == 1) {
            num = HappyNumber::sum_square_digits(num);
            if set.contains(&num) {
                return false;
            }
            set.insert(num);
        }
        true
    }

    fn sum_square_digits(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += (num % 10).checked_pow(2).unwrap();
            num = num / 10;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_number_19_is_happy() {
        let have = HappyNumber::is_happy(19);
        let want = true;

        assert_eq!(want, have);
    }

    #[test]
    fn the_number_2_is_sad() {
        let have = HappyNumber::is_happy(2);
        let want = false;

        assert_eq!(want, have);
    }

    #[test]
    fn the_number_7_is_sad() {
        let have = HappyNumber::is_happy(7);
        let want = true;

        assert_eq!(want, have);
    }
}
