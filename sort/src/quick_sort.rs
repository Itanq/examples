use std::fmt::Debug;
use std::collections::VecDeque;

fn quick_sort<T: Ord + Default>(arr: &mut [T]) {
    let len = arr.len() as isize;
    quick_sort_impl(arr, 0, len-1);
}

fn quick_sort_impl<T: Ord + Default>(arr: &mut [T], st: isize, ed: isize) {
    // 左右已经相等了则结束
    if st >= ed {
        return;
    }
    // 找到一个基准位置，pivot；并且小于基准位置的值的元素都在pivot的左边，
    // 大于基准位置的值的元素都在pivot的右边
    let pivot = select_pivot_by_pointer_swap(arr, st, ed);

    // 对左右边两边再做同样的事
    quick_sort_impl(arr, st, pivot - 1);
    quick_sort_impl(arr, pivot + 1, ed);
}

/// 非递归实现；
/// 一般对于快排这种对一个整体不对折半的操作都可以通过栈来实现
fn quick_sort_no_recursive<T: Ord + Default>(arr: &mut [T]) {
    let st = 0;
    let ed = arr.len() as isize - 1;
    let mut stack = VecDeque::new();
    stack.push_back(st);
    stack.push_back(ed);
    while !stack.is_empty() {
        let right = stack.pop_back().unwrap();
        let left = stack.pop_back().unwrap();
        let pivot = select_pivot(arr, left, right);
        // 左半部分区间还可折半，则入栈
        if left < pivot - 1 {
            stack.push_back(left);
            stack.push_back(pivot-1);
        }
        // 右半部分区间还可折半，则入栈
        if pivot + 1 < right {
            stack.push_back(pivot+1);
            stack.push_back(right);
        }
    }
}

fn select_pivot<T: Ord + Default>(arr: &mut [T], st: isize, ed: isize ) -> isize {
    let mut zero: T = T::default();
    let pivot = std::mem::replace(&mut arr[st as usize], zero);
    let mut left = st as usize;
    let mut right = ed as usize;
    let mut index = left;
    while left != right {
        while left < right {
            if arr[right] > pivot {
                right -= 1;
            } else {
                arr.swap(right, index);
                index = right;
                left += 1;
                break;
            }
        }
        while left < right {
            if arr[left] <= pivot {
                left += 1;
            } else {
                arr.swap(left, index);
                index = left;
                right -= 1;
                break;
            }
        }
    }
    arr[index] = pivot;
    index as isize
}

/// 指针交换法，先从右边开始找到第一个小于基准元素的位置；然后从左边开始找到第一个大于基准元素的位置；
/// 交换这两个位置；直到左右指针重合时，交换重合位置的元素和基准位置的元素；则此时基准元素左边的值均
/// 小于基准元素，右边的值均大于基准元素
fn select_pivot_by_pointer_swap<T: Ord>(arr: &mut [T], st: isize, ed: isize) -> isize {
    let pivot = st as usize;
    let mut left = st as usize;
    let mut right = ed as usize;
    while left != right {
        while left < right && arr[right] > arr[pivot] {
            right -= 1;
        }
        while left < right && arr[left] <= arr[pivot] {
            left += 1;
        }
        if left < right {
            arr.swap(left, right);
        }
    }
    arr.swap(left, pivot);
    left as isize
}


mod test {

    use super::quick_sort;
    use super::quick_sort_no_recursive;

    #[test]
    fn test_sort1() {

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        quick_sort(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        quick_sort(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        quick_sort(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);

        let mut nums3 = vec![1,2,3,4,5,6];
        println!("Before sort: {:?}", nums3);
        quick_sort(&mut nums3);
        println!("After sort: {:?}\n", nums3);
        assert_eq!(nums3, vec![1, 2, 3, 4, 5,6]);

        let mut nums4 = Vec::<i32>::new();
        println!("Before sort: {:?}", nums4);
        quick_sort(&mut nums4);
        println!("After sort: {:?}\n", nums4);
        assert_eq!(nums4, vec![]);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        quick_sort(&mut strs);
        println!("After sort: {:?}\n", strs);
        assert_eq!(strs, vec!["Bush1", "Bush2", "Carter", "Clinton", "Ford", "Hillary", "Nixon", "Obama", "Reagan", "Trump"]);
    }

    #[test]
    fn test_sort2() {
        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        quick_sort_no_recursive(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        quick_sort_no_recursive(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        quick_sort_no_recursive(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);

        let mut nums3 = vec![1,2,3,4,5,6];
        println!("Before sort: {:?}", nums3);
        quick_sort(&mut nums3);
        println!("After sort: {:?}\n", nums3);
        assert_eq!(nums3, vec![1, 2, 3, 4, 5,6]);

        let mut nums4 = vec![1];
        println!("Before sort: {:?}", nums4);
        quick_sort(&mut nums4);
        println!("After sort: {:?}\n", nums4);
        assert_eq!(nums4, vec![1]);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        quick_sort_no_recursive(&mut strs);
        println!("After sort: {:?}\n", strs);
        assert_eq!(strs, vec!["Bush1", "Bush2", "Carter", "Clinton", "Ford", "Hillary", "Nixon", "Obama", "Reagan", "Trump"]);
    }

}