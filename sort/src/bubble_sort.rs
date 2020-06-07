
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[j] < arr[i] {
                arr.swap(i, j);
            }
        }
    }
}

mod test {

    use super::bubble_sort;

    #[test]
    fn test_sort1() {

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        bubble_sort(&mut nums);
        println!("After sort: {:?}", nums);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        bubble_sort(&mut nums1);
        println!("After sort: {:?}", nums1);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        bubble_sort(&mut nums2);
        println!("After sort: {:?}", nums2);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        bubble_sort(&mut strs);
        println!("After sort: {:?}", strs);
    }

}