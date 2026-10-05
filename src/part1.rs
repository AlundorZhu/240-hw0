/*
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?

    A: I prefer version 1, it require less from the call site and passing reference for i32 doesn't save anything, since i32 is just 4bytes.
*/

use std::collections::BTreeSet;

pub fn double_v1(n: i32) -> i32 {
    n + n
}

pub fn double_v2(n: &i32) -> i32 {
    2 * n
}

pub fn double_v3(n: &mut i32) {
    *n *= 2
}

// Example unit test (so you can recall the syntax)
#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
}
#[test]
fn test_double_v2() {
    assert_eq!(double_v2(&1), 2);
    assert_eq!(double_v2(&3), 6);
}
#[test]
fn test_double_v3() {
    let mut n = 2i32;
    double_v3(&mut n);
    assert_eq!(n, 4);

    double_v3(&mut n);
    assert_eq!(n, 8)
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/
pub fn sqrt(n: usize) -> usize {
    n.isqrt()
}

// Remember to write unit tests here (and on all future functions)
#[test]
fn test_sqrt() {
    for i in 0..100 {
        assert!(sqrt(i) * sqrt(i) <= i);
    }
}
/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?

    A: version 2 looks nicer
*/
pub fn sum_v1(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut result = 0i32;
    for &v in slice {
        result += v;
    }
    result
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut result = 0i32;
    for v in slice {
        result += v;
    }
    result
}

#[test]
fn test_sum_v1() {
    assert_eq!(sum_v1(&[]), 0);
    assert_eq!(sum_v1(&[5]), 5);
    assert_eq!(sum_v1(&[1, 2, 3, 4]), 10);
    assert_eq!(sum_v1(&[-1, -2, 3]), 0);
}

#[test]
fn test_sum_v2() {
    assert_eq!(sum_v2(&[]), 0);
    assert_eq!(sum_v2(&[5]), 5);
    assert_eq!(sum_v2(&[1, 2, 3, 4]), 10);
    assert_eq!(sum_v2(&[-1, -2, 3]), 0);
}

#[test]
fn test_sum_versions_agree() {
    let v = vec![7, -3, 0, 12, -8, 100];
    assert_eq!(sum_v1(&v), sum_v2(&v));
}

/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    let mut s = BTreeSet::new();
    for v in slice {
        if !s.contains(v) {
            s.insert(v);
        }
    }
    s.into_iter().cloned().collect()
}

#[test]
fn test_unique() {
    assert_eq!(unique(&[]), Vec::<i32>::new());
    assert_eq!(unique(&[1]), vec![1]);
    assert_eq!(unique(&[1, 2, 3]), vec![1, 2, 3]);
    assert_eq!(unique(&[2, 2, 2]), vec![2]);
    assert_eq!(unique(&[1, 2, 1, 3, 2]), vec![1, 2, 3]);
    assert_eq!(unique(&[-1, 0, -1, 5]), vec![-1, 0, 5]);
}

#[test]
fn test_unique_no_duplicates() {
    let v = unique(&[4, 1, 4, 9, 1, 0, 9]);
    assert_eq!(v.len(), 4);
    for x in [0, 1, 4, 9] {
        assert!(v.contains(&x));
    }
}

/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    slice.iter().cloned().filter(|&v| pred(v)).collect()
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
    assert_eq!(filter(&[], is_even), Vec::<i32>::new());
    assert_eq!(filter(&[1, 3, 5], is_even), Vec::<i32>::new());
    assert_eq!(filter(&[-4, -3, 0, 7], is_even), vec![-4, 0]);
}

#[test]
fn test_filter_closures() {
    // keep everything / nothing
    assert_eq!(filter(&[1, 2, 3], |_| true), vec![1, 2, 3]);
    assert_eq!(filter(&[1, 2, 3], |_| false), Vec::<i32>::new());
    // preserves order and duplicates
    assert_eq!(filter(&[5, 1, 5, 9, 2], |x| x > 3), vec![5, 5, 9]);
    // closure capturing a variable
    let threshold = 10;
    assert_eq!(filter(&[3, 10, 15, 20], |x| x >= threshold), vec![10, 15, 20]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    let mut r = vec![n1, n2];
    for i in 2..out_size {
        r.push(r[i - 1] + r[i - 2]);
    }
    r
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?

    A: Because for the second version the caller have to give up ownership by either `.clone` or put things in `{}`
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    s1.to_string() + s2
}

pub fn string_concat(s1: String, s2: String) -> String {
    s1 + s2.as_str()
}

#[test]
fn test_str_concat() {
    assert_eq!(str_concat("hello", " world"), "hello world");
    assert_eq!(str_concat("", ""), "");
    assert_eq!(str_concat("abc", ""), "abc");
    assert_eq!(str_concat("", "abc"), "abc");
}

#[test]
fn test_string_concat() {
    assert_eq!(
        string_concat("hello".to_string(), " world".to_string()),
        "hello world"
    );
    assert_eq!(string_concat(String::new(), String::new()), "");
    assert_eq!(string_concat("abc".to_string(), String::new()), "abc");
    assert_eq!(string_concat(String::new(), "abc".to_string()), "abc");
}

#[test]
fn test_concat_versions_agree() {
    for (a, b) in [("x", "y"), ("", "z"), ("rust", "acean")] {
        assert_eq!(
            str_concat(a, b),
            string_concat(a.to_string(), b.to_string())
        );
    }
}

/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    v.as_slice().concat()
}

#[test]
fn test_concat_all() {
    fn strings(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }
    assert_eq!(concat_all(vec![]), "");
    assert_eq!(concat_all(strings(&["solo"])), "solo");
    assert_eq!(concat_all(strings(&["a", "b", "c"])), "abc");
    assert_eq!(concat_all(strings(&["hello", " ", "world"])), "hello world");
    // empty strings contribute nothing
    assert_eq!(concat_all(strings(&["", "x", "", "y", ""])), "xy");
    assert_eq!(concat_all(strings(&["", "", ""])), "");
    // order is preserved
    assert_eq!(concat_all(strings(&["z", "y", "1"])), "zy1");
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    v.iter().map(|s| s.parse::<i32>().expect("valid int")).collect()
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    v.iter().map(|i| i.to_string()).collect()
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    let v = fibonacci(1, 1, n);
    let v = filter(v.as_slice(), |x| x % 2 == 0);
    let v = print_all(v);
    concat_all(v)
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
