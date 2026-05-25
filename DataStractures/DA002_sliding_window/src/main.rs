// Sliding WIndow:
// Target: Find ***"Maximus sum of contigous subarray of size k"***
// --------------------------------------------------------------------
// Example: arr = [1, 4, 2, 10, 2, 3], k = 2
// Windows: [1,4]=5, [4,2]=6, [2,10]=12, [10,2]=12, [2,3]=5 → max = 12
// --------------------------------------------------------------------

use std::ops::{Add, Sub};

fn max_sum_sliding_window(arr: &[i32], k: usize) -> (i32, usize) {
    if arr.len() < k {
        panic!("Array is samaller than size of subarray.");
    }

    let mut window_sum: i32 = 0;
    for i in 0..k {
        window_sum += arr[i];
    }
    let mut max_sum = window_sum;
    let mut max_starter: usize = 0;

    for i in k..arr.len() {
        window_sum = window_sum - arr[i - k] + arr[i];
        println!("i={} | k is fixed =  {} | i-k={}", i, k, i - k);
        println!("i:{} | k:{} | i-k:{}", arr[i], arr[k], arr[i - k]);
        println!("---------------------------------");

        if window_sum > max_sum {
            max_starter = i - k + 1;
            max_sum = window_sum;
        }
    }
    (max_sum, max_starter)
}

/// # Each trait is a **capability** that type `T` must have:
///
/// | Trait | What it means | Example |
/// |-------|---------------|---------|
/// | `Copy` | Value can be duplicated just by copying bits (no heap allocation) | `i32`, `f64` → YES. `String` → NO |
/// | `Add<Output = T>` | You can use `+` operator between two T's and get another T | `5 + 3 = 8` |
/// | `Sub<Output = T>` | You can use `-` operator | `10 - 4 = 6` |
/// | `PartialOrd` | You can compare with `>` `<` `>=` `<=` | `5 > 3` works |
/// | `Default` | You can get a zero/empty value with `T::default()` | `i32::default()` = 0 |
///
/// ---
///
/// ## Why do we need each one in our code?
///
/// Look at where they're used:
///
/// ```rust
/// // Line 1: Default needed here
/// let mut window_sum = T::default();  // Need to start at 0
///
/// // Line 2: Add needed here
/// window_sum = window_sum + arr[i];    // Need + operator
///
/// // Line 3: Sub needed here  
/// window_sum = window_sum - arr[i - k] + arr[i];  // Need - operator
///
/// // Line 4: PartialOrd needed here
/// if window_sum > max_sum {   // Need > comparison
///
/// // Line 5: Copy needed everywhere
/// arr[i]  // Reading value - need Copy to not "move" ownership
fn max_sum_sliding_window_generic<T>(arr: &[T], k: usize) -> Option<(T, usize)>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd + Default,
{
    if arr.len() < k {
        return None;
    }

    let mut windows_sum = T::default();
    for i in 0..k {
        windows_sum = windows_sum + arr[i];
    }
    let mut max_sum = windows_sum;
    let mut startgin_index = 0;

    for i in k..arr.len() {
        windows_sum = windows_sum - arr[i - k] + arr[i];
        if windows_sum > max_sum {
            max_sum = windows_sum;
            startgin_index = i - k + 1;
        }
    }
    Some((max_sum, startgin_index))
}

fn main() {
    let arr = [1, 3, 4, 2, 5, 6, 19, 39, 4, 2, 0, 23, 4, 56, 12, 3, 4, 10, 23, 23];
    let k = 5;
    // let res = max_sum_sliding_window(&arr, k);
    // println!("Starter = {} | Sum = {}", res.1, res.0);

    let res2 = max_sum_sliding_window_generic(&arr, k);

    match res2 {
        Some((sum, index)) => println!("Starter = {}| Sum = {}", index, sum),
        None => println!("No Valid Window Found."),
    }
}
