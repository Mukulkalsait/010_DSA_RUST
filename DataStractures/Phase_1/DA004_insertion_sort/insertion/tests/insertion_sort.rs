use std::fmt::Debug;

pub use insertion;
use pretty_assertions::assert_eq;
use test_case::test_case;

#[test_case(vec![1], vec![1]; "one_element")]
#[test_case(vec![1, 2, 3], vec![1, 2, 3]; "already_sorted")]
#[test_case(vec![4, 1, 3, 2], vec![1, 2, 3, 4]; "basic")]
#[test_case(vec![4, 1, 3, 5, 2], vec![1, 2, 3, 4, 5]; "odd_number")]
#[test_case(vec![5, 5, 5, 5], vec![5, 5, 5, 5]; "repeated_elements")]
#[test_case(vec!['c', 'a', 'b'], vec!['a', 'b', 'c']; "chars")]
#[test_case(vec!["c", "a", "b"], vec!["a", "b", "c"]; "strings")]
fn test_insertiong_sort<T: Ord + Copy + Debug>(mut arr: Vec<T>, expected: Vec<T>) {
    insertion::insertion::insertion_sort(&mut arr);
    assert_eq!(arr, expected);
}

fn test_insertiong_sort_empty() {
    let mut arr: [u8; 0] = [];
    let cloned = arr;
    insertion::insertion::insertion_sort(&mut arr);
}
