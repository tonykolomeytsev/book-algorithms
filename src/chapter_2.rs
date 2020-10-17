
/// Сортировка вставками (2.1.1)
pub fn insertion_sort(arr: &mut Vec<i32>) {
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i = j - 1;
        while i > 0 && arr[i] > key {
            arr.swap(i + 1, i);
            i -= 1;
        }
        if arr[0] > key { arr.swap(1, 0) }
    }
}

/// Сортировка вставками в убывающем порядке (2.1.2)
pub fn insertion_sort_descending(arr: &mut Vec<i32>) {
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i = j - 1;
        while i > 0 && arr[i] < key {
            arr.swap(i + 1, i);
            i -= 1;
        }
        if arr[0] < key { arr.swap(1, 0) }
    }
}
