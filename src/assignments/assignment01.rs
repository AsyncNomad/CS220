//! Assignment 1: Preparing Rust Development Environment.
//!
//! The primary goal of this assignment is bringing up SSH, VSCode, and all the other necessary tools to develop Rust programs.
//! Please make sure you're comfortable with developing Rust programs before moving on to the next assignments.
//!
//! You should fill out `add()` and `sub()` function bodies in such a way that `/scripts/grade-01.sh` works fine.
//! See `assignment01_grade.rs` and `/scripts/grade-01.sh` for the test script.
//!
//! Hint: <https://doc.rust-lang.org/std/primitive.usize.html>
use std::*;
/// Adds two unsigned words. If overflow happens, just wrap around.
pub fn add(lhs: usize, rhs: usize) -> usize {
    let (result, overflow) = lhs.overflowing_add(rhs);
    result
}

/// Subtracts two unsigned words. If overflow happens, just wrap around.
pub fn sub(lhs: usize, rhs: usize) -> usize {
    let (result, overflow) = lhs.overflowing_sub(rhs);
    result
}
