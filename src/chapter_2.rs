
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

// Линейный поиск (2.1.3)
pub fn linear_search(v: i32, arr: &[i32]) -> Option<usize> {
    for j in 0..arr.len() {
        let key = arr[j];
        if v == key {
            return Some(j)
        }
    }
    None
} 
