mod chapter_1;
mod chapter_2;

fn main() {}

#[test]
fn insertion_sort_test() {
    let mut actual: Vec<i32> = vec![31, 41, 59, 26, 41, 58];
    let expected: Vec<i32> = vec![26, 31, 41, 41, 58, 59];
    chapter_2::insertion_sort(&mut actual);
    assert_eq!(actual, expected);
}

#[test]
fn insertion_sort_descending_test() {
    let mut actual: Vec<i32> = vec![31, 41, 59, 26, 41, 58];
    let expected: Vec<i32> = vec![59, 58, 41, 41, 31, 26];
    chapter_2::insertion_sort_descending(&mut actual);
    assert_eq!(actual, expected);
}

#[test]
fn linear_search_test() {
    let source = [59, 58, 41, 41, 31, 26];
    let result = chapter_2::linear_search(41, &source);
    assert_eq!(result.unwrap(), 2 as usize)
}