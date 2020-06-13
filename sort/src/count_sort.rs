use std::collections::HashMap;

// 计数排序，用途非常有限，仅对个元素之间相差不是很大的整数比较有效
fn count_sort(arr: &mut [i32]) {
    if arr.len() <= 0 {
        return;
    }

    let mut min= arr[0];
    let mut max = arr[0];
    for i in 0..arr.len() {
        if arr[i] < min {
            min = arr[i];
        } else if arr[i] > max {
            max = arr[i];
        }
    }

    let mut hash = Vec::<i32>::with_capacity((max-min+1) as usize);
    unsafe {
        hash.set_len( (max-min+1) as usize );
    }
    for i in 0..(max-min+1) as usize {
        hash[i] = 0;
    }

    for i in 0..arr.len() {
        hash[ (arr[i]-min) as usize ] += 1;
    }

    let mut idx = 0;
    for i in 0..hash.len() {
        for c in 0..hash[i] {
            arr[idx] = i as i32 + min;
            idx += 1;
        }
    }
}

mod test {

    use super::count_sort;

    #[test]
    fn test_sort() {

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        count_sort(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        count_sort(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        count_sort(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);

        let mut nums3 = vec![1,2,3,4,5,6];
        println!("Before sort: {:?}", nums3);
        count_sort(&mut nums3);
        println!("After sort: {:?}\n", nums3);
        assert_eq!(nums3, vec![1, 2, 3, 4, 5,6]);

        let mut nums4 = vec![1];
        println!("Before sort: {:?}", nums4);
        count_sort(&mut nums4);
        println!("After sort: {:?}\n", nums4);
        assert_eq!(nums4, vec![1]);
    }

}