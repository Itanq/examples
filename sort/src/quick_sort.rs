use std::fmt::Debug;
use std::collections::VecDeque;

fn quick_sort<T: Ord + Default>(arr: &mut [T]) {
    let len = arr.len() as isize;
    quick_sort_impl(arr, 0, len-1);
}

fn quick_sort_impl<T: Ord + Default>(arr: &mut [T], st: isize, ed: isize) {
    if st >= ed {
        return;
    }
    let pivot = select_pivot_by_pointer_swap(arr, st, ed);
    quick_sort_impl(arr, st, pivot - 1);
    quick_sort_impl(arr, pivot + 1, ed);
}

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
        if left < pivot - 1 {
            stack.push_back(left);
            stack.push_back(pivot-1);
        }
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
        println!("After sort: {:?}", nums);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        quick_sort(&mut nums1);
        println!("After sort: {:?}", nums1);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        quick_sort(&mut nums2);
        println!("After sort: {:?}", nums2);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        quick_sort(&mut strs);
        println!("After sort: {:?}", strs);
    }

    #[test]
    fn test_sort2() {
        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        quick_sort_no_recursive(&mut nums);
        println!("After sort: {:?}", nums);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        quick_sort_no_recursive(&mut nums1);
        println!("After sort: {:?}", nums1);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        quick_sort_no_recursive(&mut nums2);
        println!("After sort: {:?}", nums2);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        quick_sort_no_recursive(&mut strs);
        println!("After sort: {:?}", strs);
    }

}