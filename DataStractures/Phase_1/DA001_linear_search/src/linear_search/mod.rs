pub mod char;
pub mod search;

use std::error::Error;

pub use search::linear_search;

pub fn x_linear_one<T: Ord>(it: &T, arr: &[T]) -> Result<usize, String> {
    let mut a: Option<usize> = None;
    for (i, item) in arr.iter().enumerate() {
        if item == it {
            a = Some(i);
            break;
        }
    }
    a.ok_or_else(|| "Not found".to_string())
}

pub fn x_linear_all<T: Ord>(item: &T, arr: &[T]) -> Result<Vec<usize>, String> {
    let res: Vec<usize> = arr.iter().enumerate().filter_map(|(i, d)| if d == item { Some(i) } else { None }).collect();
    if !res.is_empty() { Ok(res) } else { Err("Not Found".to_string()) }
}
