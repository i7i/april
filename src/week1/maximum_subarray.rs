/// Given an integer array nums, find the contiguous subarray (containing at least one number)
/// which has the largest sum and return its sum.
pub struct MaximumSubarray {}

impl MaximumSubarray {
    /// Use dynamic programming to find solution
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut largest_sum = nums[0];
        let mut current_sum = nums[0];

        for i in 1..nums.len() {
            current_sum = nums[i].max(current_sum + nums[i]);
            largest_sum = largest_sum.max(current_sum);
        }
        largest_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_contiguous_subarray() {
        let have = MaximumSubarray::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        let want = 6;

        assert_eq!(want, have);
    }
}
