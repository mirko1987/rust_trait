//! # Problem 5 — Generic registry with capacity
//! **Concepts:** generic struct, conditional impls, different bounds for
//! different blocks.
//!
//! Requirements (encoded in the tests):
//! 1. Inserting beyond capacity: the item is NOT lost, it comes back to the
//!    caller inside the `RegistryFull<T>` error.
//! 2. For any T: `new`, `insert`, `count`, `is_full`.
//! 3. Only if T is comparable for equality: `contains`.
//! 4. Only if T is orderable: `maximum`.
//! 5. Only if T is printable: `listing` (one line per item, numbered from 1,
//!    format "N. item", lines separated by '\n', no trailing newline).
//! 6. `Registry<Vec<u8>>` must be able to use the base methods — and ONLY
//!    those.
//!
//! MASTERY QUESTIONS:
//! - With `#[derive(Clone)]` on `Registry<T>`, for which T is the registry
//!   cloneable? What does the derive actually generate (implicit bound)?
//! - Why is returning the item inside the error a better design than a
//!   purely descriptive error?

/// The error returns ownership of the rejected item to the caller.
#[derive(Debug, PartialEq)]
pub struct RegistryFull<T> {
    pub item: T,
}

pub struct Registry<T> {
    // Design the fields yourself (private).
    _placeholder: std::marker::PhantomData<T>,
}

impl<T> Registry<T> {
    /// Creates a new, empty registry with the given `capacity`.
    ///
    /// TODO: implement this. Store the capacity and an empty container
    /// (e.g. `Vec<T>`) in the struct's fields — you'll need to replace the
    /// `_placeholder: PhantomData<T>` field above with real fields, since
    /// `PhantomData` cannot actually hold values.
    pub fn new(capacity: usize) -> Self {
        let _ = capacity;
        todo!()
    }

    /// Inserts `item` if there is still room. Otherwise returns it back to
    /// the caller inside `RegistryFull`, without losing it.
    ///
    /// TODO: implement this. Compare the current count against the stored
    /// capacity; if there's room, push the item and return `Ok(())`, else
    /// return `Err(RegistryFull { item })` — note `item` is moved into the
    /// error, not lost or cloned. Edge case: capacity `0` must always reject
    /// the very first insertion.
    pub fn insert(&mut self, item: T) -> Result<(), RegistryFull<T>> {
        let _ = item;
        todo!()
    }

    /// Number of items currently stored.
    ///
    /// TODO: implement this. Simply return the length of the underlying
    /// container.
    pub fn count(&self) -> usize {
        todo!()
    }

    /// Whether the registry has reached its capacity.
    ///
    /// TODO: implement this. Compare `count()` against the stored capacity
    /// (`>=`, so that a capacity of `0` is considered full immediately).
    pub fn is_full(&self) -> bool {
        todo!()
    }
}

impl<T: PartialEq> Registry<T> {
    /// Whether `target` is present among the stored items.
    ///
    /// TODO: implement this. Use `Iterator::any` (or equivalent) with
    /// `PartialEq` comparison over the stored items; this block only
    /// requires `PartialEq`, not `Eq` or `Ord`.
    pub fn contains(&self, target: &T) -> bool {
        let _ = target;
        todo!()
    }
}

impl<T: PartialOrd> Registry<T> {
    /// Reference to the largest stored item, `None` if the registry is
    /// empty.
    ///
    /// TODO: implement this. `T` only bounds on `PartialOrd` (not `Ord`), so
    /// `Iterator::max()` is not available — use `max_by(|a, b|
    /// a.partial_cmp(b).unwrap())` or an explicit fold that keeps the larger
    /// of two items at each step.
    pub fn maximum(&self) -> Option<&T> {
        todo!()
    }
}

impl<T: std::fmt::Display> Registry<T> {
    /// One line per item, numbered from 1, format `"N. item"`, lines
    /// separated by `'\n'`, with no trailing newline.
    ///
    /// TODO: implement this. Enumerate the stored items starting at 1,
    /// format each as `"{n}. {item}"` via `Display`, then join the lines
    /// with `'\n'` (e.g. with `.collect::<Vec<_>>().join("\n")`). Edge case:
    /// an empty registry should produce an empty string.
    pub fn listing(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_within_capacity() {
        let mut r = Registry::new(2);
        assert!(r.insert(10).is_ok());
        assert!(r.insert(20).is_ok());
        assert_eq!(r.count(), 2);
        assert!(r.is_full());
    }

    #[test]
    fn rejected_item_is_recoverable() {
        let mut r = Registry::new(1);
        r.insert(String::from("dentro")).ok();
        let err = r.insert(String::from("fuori")).unwrap_err();
        // The item was not lost: we get it back by value.
        assert_eq!(err.item, "fuori");
        assert_eq!(r.count(), 1);
    }

    #[test]
    fn zero_capacity() {
        let mut r = Registry::new(0);
        assert!(r.is_full());
        let err = r.insert(1).unwrap_err();
        assert_eq!(err.item, 1);
    }

    #[test]
    fn contains_requires_only_partial_eq() {
        let mut r = Registry::new(3);
        r.insert("ken").ok();
        r.insert("tsuki").ok();
        assert!(r.contains(&"ken"));
        assert!(!r.contains(&"geri"));
    }

    #[test]
    fn maximum_requires_ordering() {
        let mut r = Registry::new(3);
        r.insert(2.5).ok();
        r.insert(9.1).ok();
        r.insert(0.3).ok();
        assert_eq!(r.maximum(), Some(&9.1));

        let empty: Registry<i32> = Registry::new(3);
        assert_eq!(empty.maximum(), None);
    }

    #[test]
    fn numbered_listing() {
        let mut r = Registry::new(3);
        r.insert("mae").ok();
        r.insert("mawashi").ok();
        assert_eq!(r.listing(), "1. mae\n2. mawashi");
    }

    #[test]
    fn type_without_bound_uses_only_base_methods() {
        // Vec<u8> is not Display: this compiles ONLY because listing() lives
        // in a separate impl block with its own bound.
        let mut r: Registry<Vec<u8>> = Registry::new(2);
        assert!(r.insert(vec![1, 2, 3]).is_ok());
        assert_eq!(r.count(), 1);
        assert!(r.contains(&vec![1, 2, 3])); // Vec<u8> is PartialEq: ok
        // r.listing(); // <- uncommenting this must NOT compile: explain why
    }
}
