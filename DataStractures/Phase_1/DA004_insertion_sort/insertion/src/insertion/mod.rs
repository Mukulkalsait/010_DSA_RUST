/// # WORKING:
///  1. leave the 0th index and start from 1st index.
///  2. the 1st element is always sorted as its only 1 element.
///  3. at every index we store the value in current
///  4. **we run inner loop that stops at index 1 or if the current element is greater than its left element.**
///  5.if both conduction fails we assign the arr[window] => value of current.
///
///  with this we do:
///  1st loop runs from 1 to arr.len()
///  2nd inner loop checks if the element that is current is
///   - smaller or bigger thatn its previous element and if bigger =>
///   - it moves the biger element to right, and check element before that.
///   * if not bigger than we assign that place current as current sits there.
///
/// Time complexity is `O(n^2)`, where `n` is the number of elements.
/// Space complexity is `O(1)` as it sorts elements in-place.
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let current = arr[i];
        let mut window = i;

        while window > 0 && current < arr[window - 1] {
            arr[window] = arr[window - 1];
            window -= 1;
        }
        {}

        arr[window] = current;
    }
}
