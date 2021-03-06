use std::fmt::Debug;

fn insert_sort<T: Ord>(arr: &mut [T]) {
    // i 表示已经有序和无序的分界点，即，前 i 个元素是有序的。
    for i in 1..arr.len() {

        // idx 表示有序元素后的第一个无序元素。
        let mut idx = i;

        // 从有序列表的最后位置往前遍历，并找到可以插入第 idx 个元素的位置，
        // 使得原本有序的列表还是有序的
        for j in (0..=i-1).rev() {
            if arr[idx] < arr[j] {
                arr.swap(j, idx);
                idx = j;
            }
        }
    }
}

mod test {

    use super::insert_sort;

    #[test]
    fn test_sort1() {

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        insert_sort(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        insert_sort(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        insert_sort(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);

        let mut nums3 = vec![1,2,3,4,5,6];
        println!("Before sort: {:?}", nums3);
        insert_sort(&mut nums3);
        println!("After sort: {:?}\n", nums3);
        assert_eq!(nums3, vec![1, 2, 3, 4, 5,6]);

        let mut nums4 = vec![1];;
        println!("Before sort: {:?}", nums4);
        insert_sort(&mut nums4);
        println!("After sort: {:?}\n", nums4);
        assert_eq!(nums4, vec![1]);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        insert_sort(&mut strs);
        println!("After sort: {:?}\n", strs);
        assert_eq!(strs, vec!["Bush1", "Bush2", "Carter", "Clinton", "Ford", "Hillary", "Nixon", "Obama", "Reagan", "Trump"]);
    }

}