
// 鸡尾酒排序，冒泡排序的升级版，区别就是冒泡排序是单向的，每次都是从左往右。
// 鸡尾酒排序就是双向的冒泡排序，从左往右之后再从右往左。
fn cocktail_sort<T: Ord>(arr: &mut [T]) {

    let mut is_sorted = true;

    // 遍历的区间
    let mut left_border = 0;
    let mut right_border = arr.len() - 1;

    // 分别标定数组中左右两边分别已有序的部分
    let mut last_left = 0;
    let mut last_right = arr.len() - 1;

    //每一轮两次遍历，故 n/2 轮
    for i in 0..=arr.len()/2 {

        // 从左向右遍历
        for j in left_border..right_border {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);

                // 存在交换操作，则整个数组不是有序态
                is_sorted = false;
                // last_right 之后的部分都是有序的，下次遍历时遍历到此处即可。
                last_right = j;
            }
        }
        // 已经有序了则退出循环
        if is_sorted {
            break;
        }
        // 更新需要遍历到的最右边的位置
        right_border = last_right;

        // 再次假定数组已经有序了
        is_sorted = true;

        // 从右向左遍历
        for j in (left_border..=right_border).rev() {
            if j >= 1 && arr[j] < arr[j-1] {
                arr.swap(j, j-1);
                is_sorted = false;
                // last_left 之前的部分都是有序的，下次遍历时遍历到此处即可
                last_left = j;
            }
        }
        if is_sorted {
            break;
        }
        // 更新需要遍历到的最左边的位置
        left_border = last_left;
    }
}

mod test {

    use super::cocktail_sort;

    #[test]
    fn test_sort1() {

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        cocktail_sort(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        cocktail_sort(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        cocktail_sort(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);


        let mut nums3 = vec![1,2,3,4,5,6];
        println!("Before sort: {:?}", nums3);
        cocktail_sort(&mut nums3);
        println!("After sort: {:?}\n", nums3);
        assert_eq!(nums3, vec![1, 2, 3, 4, 5,6]);

        let mut nums4 = vec![1];
        println!("Before sort: {:?}", nums4);
        cocktail_sort(&mut nums4);
        println!("After sort: {:?}\n", nums4);
        assert_eq!(nums4, vec![1]);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        cocktail_sort(&mut strs);
        println!("After sort: {:?}\n", strs);
        assert_eq!(strs, vec!["Bush1", "Bush2", "Carter", "Clinton", "Ford", "Hillary", "Nixon", "Obama", "Reagan", "Trump"]);
    }

}