//! Assignment 13: Parallelism.
//!
//! The primary goal of this assignment is to get used to data parallelism.
//!
//! Refer to your solution for assignment 09. You will implement the parallelized version of assignment 09.
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-13.sh` works fine.
//! See `assignment13_grade.rs` and `/scripts/grade-13.sh` for the test script.

use rayon::prelude::*;

/// Returns the sum of `f(v)` for all element `v` the given array.
///
/// # Exmaple
///
/// ```
/// use cs220::assignments::assignment13::sigma;
/// use rayon::iter::IntoParallelIterator;
///
/// assert_eq!(sigma([1, 2].into_par_iter(), |x| x + 2), 7);
/// assert_eq!(sigma([1, 2].into_par_iter(), |x| x * 4), 12);
/// ```
pub fn sigma<T, F: Fn(T) -> i64 + Sync + Send>(
    inner: impl ParallelIterator<Item = T>,
    f: F,
) -> i64 {
    inner.map(f).sum()
}

/// Alternate elements from three iterators until they have run out.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment13::interleave3;
/// use rayon::iter::IntoParallelIterator;
///
/// assert_eq!(
///     interleave3([1, 2].into_par_iter(), [3, 4].into_par_iter(), [5, 6].into_par_iter()),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave3<T: Send>(
    list1: impl IndexedParallelIterator<Item = T>,
    list2: impl IndexedParallelIterator<Item = T>,
    list3: impl IndexedParallelIterator<Item = T>,
) -> Vec<T> {
    let l1: Vec<T> = list1.collect();
    let l2: Vec<T> = list2.collect();
    let l3: Vec<T> = list3.collect();

    let mut result = Vec::new();
    let mut a = std::collections::HashMap::new();
    let mut i = 0;
    let mut n = 0;
    for element in l1.into_iter() {
        let x = a.insert(i, element);
        i += 3;
        n += 1;
    }
    i = 1;
    for element in l2.into_iter() {
        let x = a.insert(i, element);
        i += 3;
        n += 1;
    }
    i = 2;
    for element in l3.into_iter() {
        let x = a.insert(i, element);
        i += 3;
        n += 1;
    }
    for j in 0..n {
        result.push(a.remove(&j).unwrap());
    }
    result
}
