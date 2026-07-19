pub mod char;

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
/// ## Returns
/// * `Some(index)` - The index of the first occurrence
/// * `None` - If the item is not found
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
        if item == data {
            return Some(i);
        }
    }
    None
}

/// better Error handling.
pub fn linear_search_res<T: Ord>(item: &T, arr: &[T]) -> Result<usize, String> { linear_search(item, arr).ok_or_else(|| "Item not found in array".to_string()) }

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

/// ---------------------------------------------------------------------------------------------------------------------
/// # Linear search with custom predicate
///
/// ## Use Cases
/// - Searching with custom comparison logic
/// - Partial matching (strings containing substring)
/// - Range-based searches
/// - Complex object property matching
pub fn linear_search_by<F, T>(arr: &[T], mut predicate: F) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    arr.iter().position(|item| predicate(item))
}

pub fn linear_search_by_any<F, T>(arr: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    arr.iter().any(predicate)
}

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
