use std::fmt::Debug;
use std::collections::VecDeque;

fn merge_sort<T: Ord + Copy >(arr: &mut [T]) {
    let len = arr.len();
    merge_sort_impl(arr, 0, len-1);
}

fn merge_sort_impl<T: Ord + Copy>(arr: &mut [T], st: usize, ed: usize) {
    if st >= ed {
        return;
    }
    let mid = st + (ed - st) / 2;
    merge_sort_impl(arr, st, mid);
    merge_sort_impl(arr, mid+1, ed);
    merge(arr, st, mid, ed);
}

fn merge_sort_no_recursive<T: Ord + Copy + Debug>(arr: &mut [T]) {

    let len = arr.len() - 1;

    let mut size = 1;   //<! 归并的大小
    while size <= len {
        let mut left = 0;
        while left + size <= len {
            let mid = left + size - 1;
            let mut right = mid + size;
            if right > len {
                right = len;
            }
            merge(arr, left, mid,right);
            left = right + 1;
        }
        size *= 2;
    }
}

fn merge<T: Ord + Copy> (arr: &mut [T], st: usize, mid: usize, ed: usize) {
    let mut sorted = Vec::with_capacity(ed-st+1);
    let mut p1 = st;
    let mut p2 = mid+1;
    while p1 <= mid && p2 <= ed {
        if arr[p1] < arr[p2] {
            sorted.push(arr[p1].clone());
            p1 += 1;
        } else {
            sorted.push(arr[p2].clone());
            p2 += 1;
        }
    }
    while p1 <= mid {
        sorted.push(arr[p1].clone());
        p1 += 1;
    }
    while p2 <= ed {
        sorted.push(arr[p2].clone());
        p2 += 1;
    }
    for i in 0..sorted.len() {
        arr[i+st] = sorted[i];
    }
}

mod test {

    use super::merge_sort;
    use super::merge_sort_no_recursive;

    #[test]
    fn test_sort1() {

        let mut nums = Vec::new();
        nums.push(11);
        nums.push(12);

        println!("res={:?}", nums);

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        merge_sort(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        merge_sort(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        merge_sort(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        merge_sort(&mut strs);
        println!("After sort: {:?}\n", strs);
        assert_eq!(strs, vec!["Bush1", "Bush2", "Carter", "Clinton", "Ford", "Hillary", "Nixon", "Obama", "Reagan", "Trump"]);
    }

    #[test]
    fn test_sort2() {

        let mut nums = Vec::new();
        nums.push(11);
        nums.push(12);

        println!("res={:?}", nums);

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        merge_sort_no_recursive(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        merge_sort_no_recursive(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        merge_sort_no_recursive(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        merge_sort_no_recursive(&mut strs);
        println!("After sort: {:?}\n", strs);
        assert_eq!(strs, vec!["Bush1", "Bush2", "Carter", "Clinton", "Ford", "Hillary", "Nixon", "Obama", "Reagan", "Trump"]);
    }
}