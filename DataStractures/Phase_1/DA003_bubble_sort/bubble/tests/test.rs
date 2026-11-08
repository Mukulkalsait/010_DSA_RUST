use pretty_assertions::assert_eq;
use test_case::test_case;

#[test_case(&mut [6,3,5,4,2] ,  &mut [2,3,4,5,6] )]
#[test_case(&mut [7,6,5,4,3,2,1,0] ,  &mut [0,1,2,3,4,5,6,7] )]
fn test_bineary_search<T: Ord + std::fmt::Debug>(arr: &mut [T], sorted: &mut [T]) {
    bubble::bubble::bubble_sort(arr);
    assert_eq!(arr, sorted)
}
