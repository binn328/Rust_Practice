//! # My Crate
//!
//! `my_crate` is a collection of utilities to make perfoming certain
//! calculations more convenient.

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let file = 5;
///
/// assert_eq!(6, cargo::add_one(5));
/// ```
/// 
/// # Panics
///
/// # Errors
///
/// # Safety
pub fn add_one(x: i32) -> i32 {
    x + 1
}
