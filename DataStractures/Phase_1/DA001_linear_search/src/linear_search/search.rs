// Search.rs

use core::error;
use std::io::Result;

/// ---------------------------------------------------------------------------------------------------------------------
/// # Linear search to find the first occurrence of an item in a slice.
///  - Linear search == sequential search
/// ## Complexity
/// - Time: O(n) where n is the length of the array
/// - Space: O(1) - uses constant extra space
///
/// ## Arguments
/// * `item` - The item to search for
/// * `arr` - The slice to search in
///
/// ## Use Cases
/// 1. **Small Collections**: Best for arrays with < 100 elements
/// 2. **Unsorted Data**: Works without preprocessing
/// 3. **One-time Searches**: No setup overhead
/// 4. **Linked Lists**: Primary search method for linked structures
/// 5. **Real-time Systems**: Predictable O(n) behavior
/// 6. **Embedded Systems**: Minimal memory footprint
/// 7. **Unsorted Dynamic Data**: When data changes frequently
/// 8. **Finding First Match**: When you need the first occurrence
/// 9. **Search in Small Datasets**: Config files, small databases
/// 10. **Debugging/Development**: Simple to implement and debug
/// 11. **String Matching**: Pattern matching in text
/// 12. **Cache Lookups**: Small cache validation
/// 13. **Testing/Assertions**: Verifying presence of elements
/// 14. **User Input Validation**: Checking against small lists
/// 15. **Configuration Validation**: Checking allowed values
///
/// ## Performance Notes
/// - **Best Case**: O(1) - item is first element
/// - **Average Case**: O(n/2) - item is in middle
/// - **Worst Case**: O(n) - item is last or not found
/// - **Cache Friendly**: Sequential memory access pattern
///
/// ## When NOT to use
///  > - Large datasets (>1000 elements)
///  > - Frequent searches on the same dataset (use BinarySearch or HashMap)
///  > - When data is sorted and static (prefer BinarySearch)
///
/// # Alternative Approaches
/// - `binary_search()` - O(log n) for sorted data
/// - `HashMap` - O(1) average time for lookups
/// - `BTreeMap` - O(log n) sorted key lookups
/// - `contains()` - If you only need boolean presence
pub fn linear_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if data == item {
            return Some(i);
        }
    }
    None
}

//? R: above funciton with error handling
pub fn linear_search_res<T: Ord>(item: &T, arr: &[T]) -> std::result::Result<usize, String> {
    let mut res: Option<usize> = None;
    for (i, data) in arr.iter().enumerate() {
        if data == item {
            res = Some(i)
        }
    }
    res.ok_or_else(|| "Item not found".to_string())
}

/// ---------------------------------------------------------------------------------------------------------------------
/// # Returns all positions where the item occurs
///
/// ## Use Cases
/// - Finding all duplicates
/// - Text search with multiple matches
/// - Data cleaning and validation
pub fn linear_search_all<T: Ord>(item: &T, arr: &[T]) -> Vec<usize> {
    arr.iter().enumerate().filter_map(|(i, data)| if data == item { Some(i) } else { None }).collect()
}

pub fn linear_search_all_error_handling<T: Ord>(item: &T, arr: &[T]) -> std::result::Result<Vec<usize>, String> {
    let res: Vec<usize> = arr.iter().enumerate().filter_map(|(i, d)| if d == item { Some(i) } else { None }).collect();
    if !res.is_empty() { Ok(res) } else { Err("Item Not found".to_string()) }
}

/// ---------------------------------------------------------------------------------------------------------------------
/// # Linear search with custom predicate
///
/// ## Use Cases
/// - Searching with custom comparison logic
/// - Partial matching (strings containing substring)
/// - Range-based searches
/// - Complex object property matching
/// 1. What does F: FnMut(&T) -> bool mean?
///
/// # This is a trait bound for a generic type F. It tells the compiler: "The parameter predicate must be a function or closure that meets specific criteria."
///     F: A generic parameter representing the closure or function you pass in.
///     FnMut: A closure trait in Rust. It means this function can be called multiple times and is allowed to mutate (change) variables in its surrounding environment if it wants to.
///     (&T): The input argument type that the closure accepts. When called, the closure gets a reference to an element of type T.
///     -> bool: The return type of the closure. It must return true (if the element matches what you're looking for) or false (if it doesn't).
/// In short: predicate is a custom rule/test you pass into linear_search_by that checks an item and answers with true or false.
///
// Search directly on a field:
// linear_search_by(&users, |user| user.id == 42);
///
pub fn linear_search_by<F: FnMut(&T) -> bool, T>(arr: &[T], mut predicate: F) -> Option<usize> { arr.iter().position(|item| predicate(item)) }
