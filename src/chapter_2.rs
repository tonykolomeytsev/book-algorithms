
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

pub fn summarize_binary(a: &Vec<u8>, b: &Vec<u8>, c: &mut Vec<u8>) {
    let len = a.len();
    assert_eq!(len, b.len());
    assert_eq!(len + 1, c.len());
    for i in 0..a.len() {
        let sum = a[i] + b[i] + c[i];
        c[i] = sum % 2;
        if sum >= 2 { c[i + 1] = 1 }
    }
}