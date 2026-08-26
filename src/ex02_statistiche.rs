//! # Problem 2 — Generic statistics library
//! **Concepts:** generic functions, multiple trait bounds, `where`.
//!
//! Reusable numeric utility module that works with any sensible type.
//!
//! Requirements:
//! 1. `minimum` and `maximum`: reference to the min/max element, `None` if
//!    empty.
//! 2. `range`: (min, max) pair in a SINGLE pass (no double call to the
//!    functions above: write a single fold/loop).
//! 3. `average`: for float types, going through the `FromCount` trait that
//!    you must implement for `f32` and `f64`.
//! 4. Everything must work with `i32`, `f64` and the custom type `Score`.
//!
//! MASTERY QUESTIONS:
//! - Difference between an inline bound (`<T: A + B>`) and `where`: when is
//!   `where` NECESSARY and not just cosmetic?
//! - Why doesn't `f64` implement `Ord`, and what consequence did that have
//!   here?

use std::ops::{Add, Div};

/// Converts a count (usize) into the target numeric type.
pub trait FromCount {
    fn from_usize(n: usize) -> Self;
}

impl FromCount for f32 {
    fn from_usize(n: usize) -> Self {
        n as f32
    }
}

impl FromCount for f64 {
    fn from_usize(n: usize) -> Self {
        n as f64
    }
}

// The derives are already provided because the tests need to compile;
// in the mastery questions, explain WHAT each one generates.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Score(pub u32);

impl Add for Score {
    type Output = Score;
    fn add(self, rhs: Score) -> Score {
        // `Score` is a tuple struct with a SINGLE field (`.0`), so its
        // constructor takes exactly one argument: unwrap both operands with
        // `.0`, add them as plain `u32`s, then rewrap the result.
        Score(self.0 + rhs.0)
    }
}

/// Reference to the smallest element, `None` if the slice is empty.
///
/// TODO: implement this.
/// Hint: `T` only bounds on `PartialOrd`, not `Ord` (see the mastery
/// question about `f64`), so `Iterator::min()` is not available — you need
/// something like `iter().min_by(|a, b| a.partial_cmp(b).unwrap())`, or an
/// explicit fold that keeps the smaller of two elements at each step.
pub fn minimum<T: PartialOrd>(values: &[T]) -> Option<&T> {
       values.iter().min_by(|a,b| a.partial_cmp(b).unwrap())
}

/// Reference to the largest element, `None` if the slice is empty.
///
/// TODO: implement this, symmetric to `minimum` (use `max_by` or an
/// equivalent fold, comparing with `>` instead of `<`).
pub fn maximum<T: PartialOrd>(values: &[T]) -> Option<&T> {
   values.iter().max_by(|a,b| a.partial_cmp(b).unwrap())
}

/// (min, max) in a single pass.
///
/// TODO: implement this WITHOUT calling `minimum`/`massimo` (that would be
/// two passes over the slice). Use a single `fold`/loop that starts from
/// the first element and, for every subsequent element, updates the running
/// min and max as needed. Return `None` on an empty slice.
pub fn range<T: PartialOrd>(values: &[T]) -> Option<(&T, &T)> {
    let _ = values;
    todo!()
}

/// Arithmetic average (mean) of the slice, `None` if empty.
///
/// TODO: implement this. Sum the elements with `fold` and the `Add` bound,
/// then divide by `T::from_usize(values.len())` (that's what `FromCount` is
/// for: turning the element count into a `T` you can `Div` by, since not
/// every numeric type can be divided by a `usize` directly).
pub fn average<T>(values: &[T]) -> Option<T>
where
    T: Copy + Add<Output = T> + Div<Output = T> + FromCount,
{
    let _ = values;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_and_maximum_with_integers() {
        let v = [3, -1, 7, 0];
        assert_eq!(minimum(&v), Some(&-1));
        assert_eq!(maximum(&v), Some(&7));
    }

    #[test]
    fn empty_slice() {
        let v: [i32; 0] = [];
        assert_eq!(minimum(&v), None);
        assert_eq!(maximum(&v), None);
        assert_eq!(range(&v), None);
        let f: [f64; 0] = [];
        assert_eq!(average(&f), None);
    }

    #[test]
    fn works_with_floats() {
        let v = [2.5, 1.5, 9.0];
        assert_eq!(minimum(&v), Some(&1.5));
        assert_eq!(range(&v), Some((&1.5, &9.0)));
    }

    #[test]
    fn works_with_custom_type() {
        let v = [Score(10), Score(3), Score(99)];
        assert_eq!(minimum(&v), Some(&Score(3)));
        assert_eq!(maximum(&v), Some(&Score(99)));
        assert_eq!(Score(2) + Score(3), Score(5));
    }

    #[test]
    fn range_single_element() {
        let v = [42];
        assert_eq!(range(&v), Some((&42, &42)));
    }

    #[test]
    fn float_average() {
        let v = [1.0f64, 2.0, 3.0, 4.0];
        assert_eq!(average(&v), Some(2.5));
        let w = [1.5f32, 2.5];
        assert_eq!(average(&w), Some(2.0f32));
    }
}
