
pub fn insertion_sort(arr: &mut Vec<i32>) {
    for j in 2..arr.len() {
        let key = arr[j];
        let mut i = j - 1;
        while i > 0 && arr[i] > key {
            arr.swap(i + 1, i);
            i -= 1;
        }
        if arr[0] > key { arr.swap(1, 0) }
    }
}