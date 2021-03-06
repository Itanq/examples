use std::fmt::Debug;

// 利用最大/小堆的性质不断的输出堆顶元素来进行的排序
fn heap_sort<T: Ord + Copy + Debug>(arr: &mut [T]) {
    // 构建最大堆
    build_heap( arr );

    // 循环删除最大堆的堆顶到数组的最后位置,进行 n 次后得到升序数组
    for i in 0..arr.len() {
        pop(arr, arr.len() - i);
    }
}

fn pop<T: Ord + Debug>(arr: &mut [T], len: usize) {
    arr.swap(0, len - 1);
    down_adjust(arr, 0, len-1);
}

fn down_adjust<T: Ord + Debug>(arr: &mut [T], parent: usize, len: usize) {
    let mut child = parent * 2 + 1;
    let mut parent = parent;
    while child < len {
        // 找到左右子节点中最大的节点
        if child + 1 < len && arr[ child ] < arr[ child + 1 ] {
            child += 1;
        }

        // 当前父节点小于最大的子节点，则父节点下沉到最大子节点的位置，
        // 对应的，最大子节点则上浮到父节点位置。
        if arr[ parent ] < arr[ child ] {
            arr.swap(child, parent);
        }
        parent = child;
        child = parent * 2 + 1;
    }
}

fn build_heap<T: Ord + Debug>(arr: &mut [T]) {
    // 从最后一个非叶子节点开始做向下调整来构建最大堆
    for i in (0..=(arr.len()-1)/2).rev() {
        down_adjust(arr, i, arr.len());
    }
}

mod test {

    use super::heap_sort;

    #[test]
    fn test_sort1() {

        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        println!("Before sort: {:?}", nums);
        heap_sort(&mut nums);
        println!("After sort: {:?}\n", nums);
        assert_eq!(nums, vec![-345, -234, 1, 1, 3, 4, 5, 8, 9, 9, 55, 56, 67, 89, 123]);

        let mut nums1 = vec![5, 4, 2, 3, 1];
        println!("Before sort: {:?}", nums1);
        heap_sort(&mut nums1);
        println!("After sort: {:?}\n", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5]);

        let mut nums2 = vec![5, 4, 3, 2, 1];
        println!("Before sort: {:?}", nums2);
        heap_sort(&mut nums2);
        println!("After sort: {:?}\n", nums2);
        assert_eq!(nums2, vec![1, 2, 3, 4, 5]);

        let mut nums3 = vec![1,2,3,4,5,6];
        println!("Before sort: {:?}", nums3);
        heap_sort(&mut nums3);
        println!("After sort: {:?}\n", nums3);
        assert_eq!(nums3, vec![1, 2, 3, 4, 5,6]);

        let mut nums4 = vec![1];
        println!("Before sort: {:?}", nums4);
        heap_sort(&mut nums4);
        println!("After sort: {:?}\n", nums4);
        assert_eq!(nums4, vec![1]);

        let mut strs = vec!["Obama", "Trump", "Bush2", "Clinton", "Bush1", "Reagan", "Carter", "Ford", "Nixon", "Hillary"];
        println!("Before sort: {:?}", strs);
        heap_sort(&mut strs);
        println!("After sort: {:?}\n", strs);
        assert_eq!(strs, vec!["Bush1", "Bush2", "Carter", "Clinton", "Ford", "Hillary", "Nixon", "Obama", "Reagan", "Trump"]);
    }

}