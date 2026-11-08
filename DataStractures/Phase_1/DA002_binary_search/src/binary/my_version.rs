use std::{ascii::Char::LeftCurlyBracket, cmp::Ordering};

pub fn binary_at_once_res<T: Ord>(item: &T, arr: &[T]) -> Result<usize, String> {
    // index NOT VALUES  R:
    let mut l = 0;
    let mut r = arr.len();
    let is_ass: bool = !arr.is_empty() && arr[0] < arr[arr.len() - 1];

    while l < r {
        let mid = l + (r - l) / 2;
        match (item.cmp(&arr[mid]), is_ass) {
            (Ordering::Less, true) | (Ordering::Greater, false) => r = mid,
            (Ordering::Greater, true) | (Ordering::Less, false) => l = mid + 1,
            (Ordering::Equal, _) => return Ok(mid),
        }
    }
    Err("Not found".to_string())
}

pub fn binary_at_once_option<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let assending: bool = !arr.is_empty() && arr[0] < arr[arr.len() - 1];
    let mut left: usize = 0; // index
    let mut right: usize = arr.len(); // index wall

    while left < right {
        let mid = left + (right - left) / 2;
        match (item.cmp(&arr[mid]), assending) {
            (Ordering::Less, true) | (Ordering::Greater, false) => right = mid,
            (Ordering::Greater, true) | (Ordering::Less, false) => left = mid + 1,
            (Ordering::Equal, _) => return Some(mid),
        }
    }
    None
}
