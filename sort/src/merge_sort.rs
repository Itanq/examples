use std::fmt::Debug;
use std::collections::VecDeque;

/// 归并排序本质就是合并两个有序的数组到一个数组中，同时使的合并后的数组还是有序数组。
/// 所以它需要额外O(n)的空间去临时存储归并的数组
fn merge_sort<T: Ord + Copy >(arr: &mut [T]) {
    let len = arr.len();
    merge_sort_impl(arr, 0, len-1);
}

/// 递归实现就是不断把一个数组分成左右两部分，直到只有一个元素位置。因为一个元素肯定是有序的。
/// 所以此时归并两个有序的元素到一个元素中。
fn merge_sort_impl<T: Ord + Copy>(arr: &mut [T], st: usize, ed: usize) {
    if st >= ed {
        return;
    }
    let mid = st + (ed - st) / 2;
    merge_sort_impl(arr, st, mid);
    merge_sort_impl(arr, mid+1, ed);
    merge(arr, st, mid, ed);
}

/// 直接从归并的大小 1 开始，
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
    // 申请一个能都容纳两个数组的数组
    let mut sorted = Vec::with_capacity(ed-st+1);

    // 指向第一个数组的起始位置
    let mut p1 = st;

    // 指向第二个数组的起始位置
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
        // 归并后的素数复制到元素组中
        arr[ i + st ] = sorted[ i ];
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