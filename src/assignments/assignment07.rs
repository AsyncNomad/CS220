//! Assignment 7: Mastering advanced types (2/2).
//!
//! The primary goal of this assignment is to understand generics, traits, and lifetimes.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-07.sh` works fine.
//! See `assignment07_grade.rs` and `/scripts/grade-07.sh` for the test script.

struct FindIter<'s, T: Eq> {
    query: &'s [T],
    base: &'s [T],
    curr: usize,
}

impl<T: Eq> Iterator for FindIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.query.len();
        let b = self.base.len();
        let mut x: usize = 0;
        if a > b {
            return None;
        }
        if self.curr > b - a {
            return None;
        }
        if self.curr == 0 {
            for i in 0..(b - a + 1) {
                for j in 0..a {
                    if self.query[j] == self.base[i + j] {
                        x += 1;
                    }
                }
                if x == a {
                    self.curr = 1;
                    return Some(i);
                }
                x = 0;
            }
        }
        for i in (self.curr + 1)..(b - a + 1) {
            for j in 0..a {
                if self.query[j] == self.base[i + j] {
                    x += 1;
                }
            }
            if x == a {
                self.curr = i;
                return Some(i);
            }
            x = 0;
        }
        None
    }
}

/// Returns an iterator over substring query indexes in the base.
pub fn find<'s, T: Eq>(query: &'s [T], base: &'s [T]) -> impl 's + Iterator<Item = usize> {
    FindIter {
        query,
        base,
        curr: 0,
    }
}
