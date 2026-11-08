use std::cmp::Ordering;
pub mod my_version;

// check if len is 1+ and first_ele < last_element
pub fn is_ascending_order<T: Ord>(arr: &[T]) -> bool { arr.len() > 1 && arr[0] < arr[arr.len() - 1] }

pub fn match_compair<T: Ord>(item: &T, arr: &[T], left: &mut usize, right: &mut usize, is_asending: bool) -> bool {
    let mid = *left + (*right - *left) / 2;
    let cmp_res = item.cmp(&arr[mid]);

    match (is_asending, cmp_res) {
        (true, Ordering::Less) | (false, Ordering::Greater) => {
            *right = mid;
        },
        (true, Ordering::Greater) | (false, Ordering::Less) => {
            *left = mid + 1;
        },
        (_, Ordering::Equal) => {
            *left = mid;
            return true;
        },
    }
    false
}

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        if match_compair(item, arr, &mut left, &mut right, is_ascending_order(arr)) {
            return Some(left);
        }
    }
    None
}
