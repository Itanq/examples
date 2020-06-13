
// 相邻的元素两两交换的才是冒泡排序
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    // N 个元素需要排序，共 N 轮
    for i in 0..arr.len() {
        // 第 i 轮需要比较的元素是 n - 1 - i
        for j in 0..arr.len()-1-i {
            if arr[j+1] < arr[j] {
                arr.swap(j+1, j);
            }
        }
    }
}

fn bubble_sort_optimize<T: Ord>(arr: &mut [T]) {
    // 标记数组是否已有序
    let mut is_sorted = true;

    // 数组中已有序和无序的边界
    let mut right_border = arr.len() - 1;

    // 最后发生元素交换的位置就是有序和无序的边界位置
    let mut last_right = arr.len() - 1;

    for i in 0..arr.len() {

        is_sorted = true;
        for j in 0..right_border {
            if arr[j+1] < arr[j] {
                arr.swap(j+1, j);

                is_sorted = false;
                last_right = j;
            }
        }
        if is_sorted {
            break;
        }
        right_border = last_right;
    }
}

mod test {

    use super::bubble_sort;
    use super::bubble_sort_optimize;

    #[test]
    fn test_sort1() {

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        bubble_sort(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        bubble_sort(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        bubble_sort(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        bubble_sort(&mut strs);
        println!("After sort: {:?}\n", strs);
        assert_eq!(strs, vec!["Bush1", "Bush2", "Carter", "Clinton", "Ford", "Hillary", "Nixon", "Obama", "Reagan", "Trump"]);
    }

    #[test]
    fn test_sort2() {

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        bubble_sort_optimize(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        bubble_sort_optimize(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        bubble_sort_optimize(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        bubble_sort_optimize(&mut strs);
        println!("After sort: {:?}\n", strs);
        assert_eq!(strs, vec!["Bush1", "Bush2", "Carter", "Clinton", "Ford", "Hillary", "Nixon", "Obama", "Reagan", "Trump"]);
    }
}