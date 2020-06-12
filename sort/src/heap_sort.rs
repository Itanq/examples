
fn down_adjust<T: Ord + Clone>(arr: &mut [T], parent: usize) {
    let mut child_idx = parent * 2 + 1;
    let len = arr.len();
    let mut parent = parent;
    while child_idx < len && arr[ parent ] < arr[ child_idx ]{
        if arr[ child_idx ] < arr[ child_idx + 1 ] {
            child_idx += 1;
        }
        arr.swap(child_idx, parent);
        parent = child_idx;
        child_idx = parent * 2 + 1;
    }
}

fn build_heap<T: Ord + Clone>(arr: &mut [T]) {
    for i in (0..=(arr.len()-1)/2).rev() {
        down_adjust(arr, i);
    }
}


mod test {

    use super::build_heap;

    #[test]
    fn test_sort1() {
        let mut nums = vec![9, 3, 67, 4, 1, 89, -234, 5, -345, 56, 123, 8, 1, 9, 55];
        build_heap(&mut nums);
        println!("nums: {:?}", nums);
    }

}