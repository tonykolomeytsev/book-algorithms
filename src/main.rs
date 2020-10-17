mod chapter_1;
mod chapter_2;

fn main() {}

#[test]
fn insertion_sort_test() {
    let mut actual: Vec<i32> = vec![1, 9, 5, 4, 3, 7, 8, 6, 2, 0, 15, 999, -1000];
    let expected: Vec<i32> = vec![-1000, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 15, 999];
    chapter_2::insertion_sort(&mut actual);
    assert_eq!(actual, expected);
}