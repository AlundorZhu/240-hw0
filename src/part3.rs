/*
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    The Rust borrow checker may help avoid some possible bugs.

    Then answer this question:
    Q: A common source of error in swap implementations is failing to work if
       the two references are the same. Why don't you need to worry about this
       case in Rust?

    A: Rust one allow atmost one mutatble reference at all time.

    (Try writing a unit test where they are both
    the same, i.e. swap_ints(&mut x, &mut x).)
*/

// Won't compile
// #[test]
// fn swap_same_test() {
//     let mut x = 1;
//     swap_ints(&mut x, &mut x);
// }

pub fn swap_ints(x1: &mut i32, x2: &mut i32) {
    let tmp  = *x1;

    *x1 = *x2;
    *x2 = tmp;
}

/*
    Problem 2: String duplication
*/
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}
// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?
// A: String is a owership, one string can have one owner. Also 
// because String doesn't implement copy by default because it allocates in heap

// Q2. How come it works fine here?
// A: i32 implements copy by default 
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Now implement the following function that duplicates a string n times.
fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    let mut r = Vec::new();  
    for _ in 0..times {
        r.push(s.to_string());
    }
    r
}

#[test]
fn duplicate_string_test() {
    assert_eq!(
        duplicate_string("test", 3),
        vec!["test", "test", "test"]
    )
}

/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

fn copy_me(string: /* Change in here only*/ &String) -> String {
    string.clone()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(/* Change in here only*/ &str1));
}

#[test]
fn copy_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1 /* Change in here only*/);
    assert_eq!(str1, str2);
}

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/
static  EMPTY: String = String::new();
fn new_ref_string() -> &'static String {
    &EMPTY
}

fn new_ref_str() -> &'static str {
    "hello"
}

// The same function from part2
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len(){
        return s1;
    } else {
        return s2;
    }
}

/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    If the vector is empty, return "".

    Q1. In pick_longest_in_v2, if you were to explicitly specify the lifetime
        of the input and output, what should it be?
    A: The lifetime of the output string should be as long as the string in the vectors

    Q2. What are the pros and cons of v1 and v2?
    
    A: 
        v1 pros: don't need lifetime annotations, easier to read and write
        v1 cons: It only accepts Vec<String>. It needs a .to_string() call which allocate the memory

        v2 pros: It accepts Vec<&str> and Vec<String> 
        v2 cons: It needs lifetime annotations. And the returned str have it's lifetime tide to the str in 
                 the vector. Can't use it after the str in the vector get dropped.
*/

fn pick_longest_in_v1(v: Vec<String>) -> String {
    let mut r  = "";
    for s in &v {
        r = pick_longest2(r, s.as_str());
    }
    r.to_string()
}

#[test]
fn pick_longest_test_v1() {
    assert_eq!(pick_longest_in_v1(vec![]), "");
    assert_eq!(pick_longest_in_v1(vec!["a".to_string(), "abc".to_string(), "ab".to_string()]), "abc")
}

fn pick_longest_in_v2<'a>(v: Vec<&'a str>) -> &'a str {
    let mut r = "";
    for s in v {
        r = pick_longest2(r, s);
    }
    return r;
}


#[test]
fn pick_longest_test_v2() {
    assert_eq!(pick_longest_in_v2(vec![]), "");
    assert_eq!(pick_longest_in_v2(vec!["a", "abc", "ab"]), "abc")
}

/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() if necessary to make any additional unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?

    A: I prefer v3, it's the most efficient because it didn't copy the old vec
*/

fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    assert!(v.len() <= desired_len);
    let mut r = v.clone();
    for _ in v.len()..desired_len {
        r.push(0);
    }
    debug_assert_eq!(r.len(), desired_len);
    r
}

fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    assert!(slice.len() <= desired_len);
    let mut r = slice.to_vec();
    for _ in slice.len()..desired_len {
        r.push(0);
    }
    debug_assert_eq!(r.len(), desired_len);
    r
}

fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    assert!(v.len() <= desired_len);
    for _ in v.len()..desired_len {
        v.push(0);
    }
    debug_assert_eq!(v.len(), desired_len);
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?

    A: First it's more efficient then making a copy and then append, 
       second it makes sense to take ownership when you are mutating it.

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?

    A: here row can take either Vec<bool> or a slice.  
*/

fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row);
}

fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    grid.first().is_some_and(|first| first == row)
}

#[test]
fn append_row_test() {
    let mut grid = Vec::new();
    append_row(&mut grid, vec![true, false]);
    assert_eq!(grid, vec![vec![true, false]]);

    append_row(&mut grid, vec![false]);
    append_row(&mut grid, vec![]);
    assert_eq!(grid, vec![vec![true, false], vec![false], vec![]]);
}

#[test]
fn is_first_row_test() {
    // Empty grid has no first row
    assert!(!is_first_row(&[], &[true]));
    assert!(!is_first_row(&[], &[]));

    let grid = vec![vec![true, false], vec![false, true]];
    assert!(is_first_row(&grid, &[true, false]));
    // Matches a later row, not the first
    assert!(!is_first_row(&grid, &[false, true]));
    // Prefix and different length don't match
    assert!(!is_first_row(&grid, &[true]));
    assert!(!is_first_row(&grid, &[true, false, true]));
    // Works with a Vec passed as a slice too
    let row = vec![true, false];
    assert!(is_first_row(&grid, &row));
}

/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::collections::HashMap;

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    let mut r: HashMap<i32, String> = HashMap::new();
    for (k, v) in v {
        r.insert(*k, v.clone());
    }
    r
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    h.retain(|k, _| *k >= 0);
}

#[test]
fn vector_to_hashmap_test() {
    assert!(vector_to_hashmap(&[]).is_empty());

    let v = vec![(1, String::from("one")), (-2, String::from("neg two"))];
    let h = vector_to_hashmap(&v);
    assert_eq!(h.len(), 2);
    assert_eq!(h[&1], "one");
    assert_eq!(h[&-2], "neg two");
    // v is only borrowed, so it's still usable
    assert_eq!(v.len(), 2);

    // Duplicate keys: the later value wins
    let h = vector_to_hashmap(&[(1, "a".to_string()), (1, "b".to_string())]);
    assert_eq!(h, HashMap::from([(1, "b".to_string())]));
}

#[test]
fn delete_negative_keys_test() {
    let mut h = HashMap::from([(-3, 1), (-1, 2), (0, 3), (5, -4)]);
    delete_negative_keys(&mut h);
    // 0 is kept, and negative values don't matter
    assert_eq!(h, HashMap::from([(0, 3), (5, -4)]));

    let mut h = HashMap::from([(-1, 1), (-2, 2)]);
    delete_negative_keys(&mut h);
    assert!(h.is_empty());

    let mut h: HashMap<i32, i32> = HashMap::new();
    delete_negative_keys(&mut h);
    assert!(h.is_empty());
}

/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String,String>
) {
    for (k, v) in add {
        merged.entry(k)
            .and_modify(|existing| *existing += v.as_str())
            .or_insert(v);
    }
}

#[test]
fn merge_maps_test() {
    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    // Existing keys get appended, new keys get inserted, untouched keys stay
    let mut merged = map(&[("a", "x"), ("b", "y")]);
    merge_maps(&mut merged, map(&[("a", "z"), ("c", "w")]));
    assert_eq!(merged, map(&[("a", "xz"), ("b", "y"), ("c", "w")]));

    // Merging again keeps appending
    merge_maps(&mut merged, map(&[("a", "!")]));
    assert_eq!(merged["a"], "xz!");

    // Empty add changes nothing
    merge_maps(&mut merged, HashMap::new());
    assert_eq!(merged, map(&[("a", "xz!"), ("b", "y"), ("c", "w")]));

    // Empty merged just becomes a copy of add
    let mut merged = HashMap::new();
    merge_maps(&mut merged, map(&[("k", "v")]));
    assert_eq!(merged, map(&[("k", "v")]));

    // Appending an empty string leaves the value as is
    let mut merged = map(&[("a", "x")]);
    merge_maps(&mut merged, map(&[("a", "")]));
    assert_eq!(merged["a"], "x");
}
