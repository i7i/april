/// Given an array nums, write a function to move all 0's to the end of it while maintaining the
/// relative order of the non-zero elements.
pub struct MoveZeroes {}

impl MoveZeroes {
    pub fn main(nums: &mut Vec<i32>) {
        let zeroes: Vec<i32> = nums.drain_filter(|&mut n| n == 0).collect();
        nums.extend_from_slice(&zeroes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_zeros_are_at_the_end() {
        let mut have = vec![0, 1, 0, 3, 12];
        MoveZeroes::main(&mut have);
        let want = vec![1, 3, 12, 0, 0];

        assert_eq!(want, have);
    }
}
