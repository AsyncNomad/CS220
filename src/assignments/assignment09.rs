//! Assignment 9: Iterators.
//!
//! The primary goal of this assignment is to get used to iterators.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-09.sh` works fine.
//! See `assignment09_grade.rs` and `/scripts/grade-09.sh` for the test script.

use std::collections::HashMap;

use itertools::*;

/// Returns whether the given sequence is a fibonacci sequence starts from the given sequence's first two terms.
///
/// Returns `true` if the length of sequence is less or equal than 2.
///
/// # Exmample
///
/// ```
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 13].into_iter()), true);
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 14].into_iter()), false);
/// ```
pub fn is_fibonacci(inner: impl Iterator<Item = i64>) -> bool {
    let mut element1 = (0, 0);
    let mut element2 = (1, 0);
    for element in inner.enumerate() {
        if element.0 == 0 {
            element1.1 = element.1;
        } else if element.0 == 1 {
            element2.1 = element.1;
        } else if element.1 != element1.1 + element2.1 {
            return false;
        } else {
            element1 = element2;
            element2 = element;
        }
    }
    true
}

/// Returns the sum of `f(v)` for all element `v` the given array.
///
/// # Exmaple
///
/// ```
/// assert_eq!(sigma([1, 2].into_iter(), |x| x + 2), 7);
/// assert_eq!(sigma([1, 2].into_iter(), |x| x * 4), 12);
/// ```
pub fn sigma<T, F: Fn(T) -> i64>(inner: impl Iterator<Item = T>, f: F) -> i64 {
    /*let mut sum: i64 = 0;
    for x in inner {
        sum += f(x);
    }
    sum*/
    inner.map(f).sum()
}

/// Alternate elements from three iterators until they have run out.
///
/// # Example
///
/// ```
/// assert_eq!(
///     interleave3([1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave3<T>(
    list1: impl Iterator<Item = T>,
    list2: impl Iterator<Item = T>,
    list3: impl Iterator<Item = T>,
) -> Vec<T> {
    let mut result = Vec::new();
    let mut a = HashMap::new();
    let mut i = 0;
    let mut n = 0;
    for element in list1 {
        let x = a.insert(i, element);
        i += 3;
        n += 1;
    }
    i = 1;
    for element in list2 {
        let x = a.insert(i, element);
        i += 3;
        n += 1;
    }
    i = 2;
    for element in list3 {
        let x = a.insert(i, element);
        i += 3;
        n += 1;
    }
    for j in 0..n {
        result.push(a.remove(&j).unwrap());
    }
    result
}

/// Returns mean of k smallest value's mean.
///
/// # Example
///
/// ```
/// assert_eq!(
///     k_smallest_mean(vec![1, 3, 2].into_iter(), 2),
///     ((1 + 2) as f64 / 2.0)
/// );
/// assert_eq!(
///     k_smallest_mean(vec![7, 5, 3, 6].into_iter(), 3),
///     ((3 + 5 + 6) as f64 / 3.0)
/// );
/// ```
pub fn k_smallest_mean(inner: impl Iterator<Item = i64>, k: usize) -> f64 {
    let a = inner.k_smallest(k);
    let mut sum: i64 = 0;
    for element in a {
        sum += element;
    }
    sum as f64 / k as f64
}

/// Returns mean for each class.
///
/// # Exmaple
///
/// ```
/// assert_eq!(
///     calculate_mean(
///         [
///             ("CS100".to_string(), 60),
///             ("CS200".to_string(), 60),
///             ("CS200".to_string(), 80),
///             ("CS300".to_string(), 100),
///         ]
///         .into_iter()
///     ),
///     [
///         ("CS100".to_string(), 60.0),
///         ("CS200".to_string(), 70.0),
///         ("CS300".to_string(), 100.0)
///     ]
///     .into_iter()
///     .collect()
/// );
/// ```
pub fn calculate_mean(inner: impl Iterator<Item = (String, i64)>) -> HashMap<String, f64> {
    let mut result: HashMap<String, f64> = HashMap::new();
    let mut count: HashMap<String, (i64, f64)> = HashMap::new();
    for (class, score) in inner {
        if let Some(v) = count.get(&class) {
            let sum = v.1 + score as f64;
            let n = v.0 + 1;
            let mean = sum / n as f64;
            let a = count.insert(class.clone(), (n, sum));
            let b = result.insert(class.clone(), mean);
        } else {
            let a = count.insert(class.clone(), (1, score as f64));
            let b = result.insert(class.clone(), score as f64);
        }
    }
    result
}

/// Among the cartesian product of input vectors, return the number of sets whose sum equals `n`.
///
/// # Example
///
/// The cartesian product of [1, 2, 3] and [2, 3] are:
///     [1, 2], [1, 3], [2, 2], [2, 3], [3, 2], [3, 3].
///
/// Among these sets, the number of sets whose sum is 4 is 2, which is [1, 3] and [2, 2].
///
/// ```
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 3), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 4), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 5), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 6), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 2), 0);
/// ```
pub fn sum_is_n(inner: Vec<Vec<i64>>, n: i64) -> usize {
    let mut i = 0;
    let mut k: usize = 0;
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for element in inner {
        if i == 0 {
            a = element;
            i += 1;
        } else if i == 1 {
            b = element;
            i += 1;
        } else {
            c = element;
            i += 1;
        }
    }
    if i == 1 {
        for element in a {
            if element == n {
                k += 1;
            }
        }
        return k;
    } else if i == 2 {
        let c = a.into_iter().cartesian_product(b.into_iter());
        for element in c {
            if element.0 + element.1 == n {
                k += 1;
            }
        }
    } else {
        for element in iproduct!(a.into_iter(), b.into_iter(), c.into_iter()) {
            if element.0 + element.1 + element.2 == n {
                k += 1;
            }
        }
    }
    k
}

/// Returns a new vector that contains the item that appears `n` times in the input vector.
///
/// # Example
///
/// ```
/// assert_eq!(find_count_n(vec![1, 2], 1), vec![1, 2]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 1), vec![1]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 2), vec![3]);
/// assert_eq!(find_count_n(vec![1, 2, 3, 4, 4], 1), vec![1, 2, 3]);
/// ```
pub fn find_count_n(inner: Vec<usize>, n: usize) -> Vec<usize> {
    let counts = inner.into_iter().counts();
    let mut result = Vec::new();
    for element in counts {
        if element.1 == n {
            result.push(element.0);
        }
    }
    result.sort();
    result
}

/// Return the position of the median element in the vector.
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// Please following these rules:
///
/// - If the list is empty, returns `None`.
/// - If several elements are equally median, the position of the first of them is returned.
///
/// # Exmaple
///
/// ```
/// assert_eq!(position_median(vec![1, 3, 3, 6, 7, 8, 9]), Some(3));
/// assert_eq!(position_median(vec![1, 3, 3, 3]), Some(1));
/// ```
pub fn position_median<T: Ord>(inner: Vec<T>) -> Option<usize> {
    if inner.is_empty() {
        return None;
    }
    let n: usize = if inner.len() % 2 == 0 {
        (inner.len() / 2) + 1
    } else {
        (inner.len() + 1) / 2
    };
    let mut current = &inner[0];
    let mut imin = &inner[0];
    let mut max = &inner[0];
    let mut x: usize = 0;
    for i in &inner {
        if i > max {
            max = i;
        }
    }
    let mut k = 0;
    let mut a = 0;
    while k < n {
        for j in &inner {
            if j < current {
                if imin < j {
                    current = j;
                    a = 1;
                }
            } else if current == j && a != 0 {
                k += 1;
            }
        }
        imin = current;
        current = max;
        k += 1;
        a = 0;
    }
    for (i, j) in inner.iter().enumerate() {
        if &inner[i] == imin {
            x = i;
            break;
        }
    }
    Some(x)
}
