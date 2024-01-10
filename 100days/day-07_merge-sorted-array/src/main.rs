fn main() {
    println!("Hello, world!");
    let sth=merge_arrays(vec![1, 2, 3, 0, 0, 0], vec![2, 5, 6]);
    println!("{:?}", sth);
}

fn merge_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let len1 = nums1.len();
    let len2 = nums2.len();
    let mut retar = nums1.clone();

    retar.splice(len1 - len2..len1, nums2);
 
    retar.sort();
    return retar;
}

#[test]
fn test_merge() {
    assert_eq!(
        merge_arrays(vec![1, 2, 3, 0, 0, 0], vec![2, 5, 6]),
        [1, 2, 2, 3, 5, 6]
    )
}


