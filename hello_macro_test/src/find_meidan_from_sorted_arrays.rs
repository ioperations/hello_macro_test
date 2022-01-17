pub mod find_median_sorted_arrays {

    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: &mut Vec<i32>, nums2: &[i32]) -> Vec<i32> {
        for i in nums2.iter() {
            nums1.push(*i);
        }
        nums1.to_vec()
    }

    #[allow(dead_code)]
    pub fn find_median_sorted_arrays2(nums1: &[i32], nums2: &[i32]) -> f64 {
        let mut merged: Vec<i32> = vec![];

        let size: usize = nums1.len() + nums2.len();

        let mut i = 0;
        let mut j = 0;
        for _ in 0..size {
            if i < nums1.len() && j < nums2.len() {
                if nums1[i] < nums2[j] {
                    merged.push(nums1[i]);
                    i += 1;
                } else {
                    merged.push(nums2[j]);
                    j += 1;
                }
            } else if i == nums1.len() {
                merged.push(nums2[j]);
                j += 1;
            } else if j == nums2.len() {
                merged.push(nums1[i]);
                i += 1;
            }
        }

        if size % 2 == 1 {
            let index = (size - 1) / 2;
            return merged[index] as f64;
        }
        let index1 = (size - 1) / 2;
        let index2 = size / 2;
        let fenzi = (merged[index1] + merged[index2]) as f64;

        fenzi / 2.0
    }

    #[cfg(test)]
    pub mod tests {
        use super::*;

        #[test]
        pub fn find_median_sorted_arrays_test() {
            let mut nums1 = vec![1, 2, 3];
            let nums2 = vec![4, 5, 6];

            let ret = find_median_sorted_arrays(&mut nums1, &nums2);
            let expected = vec![1, 2, 3, 4, 5, 6];
            assert!(ret == expected);
        }

        #[test]
        pub fn find_median_sorted_arrays2_test() {
            let nums1 = vec![1, 2];
            let nums2 = vec![3, 4];

            let ret = find_median_sorted_arrays2(&nums1, &nums2);
            assert!((ret - 2.5).abs() < 0.000001);
        }

        #[test]
        pub fn find_median_sorted_arrays2_test2() {
            let nums1 = vec![1, 3];
            let nums2 = vec![2];

            let ret = find_median_sorted_arrays2(&nums1, &nums2);
            assert!((ret - 2.0).abs() < 0.000001);
        }
    }
}
