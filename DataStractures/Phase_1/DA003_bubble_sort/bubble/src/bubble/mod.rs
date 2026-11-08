use core::num;

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut sorted: bool = false;
    let mut len = arr.len();

    while !sorted {
        sorted = true;
        for i in 0..len - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        len -= 1;
    }
}
