//! # Problem 8 — Data source abstraction
//! **Concepts:** associated types, bounds on associated types, comparison with
//! generics on the trait.
//!
//! Three sources: in-memory buffer (strings), synthetic counter (1..=N),
//! batched source (groups of elements).
//!
//! Requirements (encoded in the tests):
//! 1. `Source` with associated type `Item` and `next()` in Iterator style
//!    (but without implementing `Iterator`).
//! 2. Implementations for the three sources; for the batched one the item is
//!    itself a collection.
//! 3. `download_all`: consumes the source into a `Vec`.
//! 4. `summary`: ONLY for sources with printable elements. Format: first 3
//!    elements separated by ", "; if there are more, appends the literal
//!    suffix `" ... e altri K"` (left in Italian on purpose — the tests
//!    assert this exact string). Empty source -> the literal `"vuoto"`
//!    (same reason).
//! 5. Second version `SourceGen<E>` (generic over the trait) implemented
//!    TWICE for `MemoryBuffer`: as a source of `String` and of `usize` (the
//!    lengths). The test shows the disambiguation required from the caller.
//!
//! MASTERY QUESTIONS (answer below in a comment):
//! - Why does `Iterator` use an associated type and `From` a generic?
//!   Formulate the practical rule you derived from point 5.
//! - What does the bound `S::Item: Display` mean and where does it go?

use std::fmt::Display;

pub trait Source {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct MemoryBuffer {
    // Design the internal state yourself (data + current position).
    _data: Vec<String>,
}

impl MemoryBuffer {
    /// Builds a buffer over the given strings, ready to be drained in order
    /// via `Source::next`.
    ///
    /// TODO: implement this. You'll need to track a read cursor alongside
    /// the data (e.g. a separate `usize` index field, or by wrapping the
    /// `Vec` in something you can pop from the front of). Feel free to
    /// change the `_data` field above (and add others) to fit your design.
    pub fn new(data: Vec<String>) -> Self {
        let _ = data;
        todo!()
    }
}

pub struct Counter {
    // Generates the numbers from 1 to up_to, inclusive.
    _up_to: u32,
}

impl Counter {
    /// Builds a counter that will yield 1, 2, ..., up_to (inclusive) from
    /// successive calls to `Source::next`, then `None` once exhausted.
    ///
    /// TODO: implement this. Track the next value to emit (e.g. starting at
    /// 1) alongside the upper bound; `next()` should return it and advance,
    /// then return `None` once it would exceed `up_to`. Handle `up_to == 0`:
    /// the source must be empty from the very first call (see test
    /// `counter_generates_from_one_to_n`, which expects an empty vec for
    /// `Counter::new(0)`).
    pub fn new(up_to: u32) -> Self {
        let _ = up_to;
        todo!()
    }
}

pub struct Batched {
    _batches: Vec<Vec<i32>>,
}

impl Batched {
    /// Builds a source that yields each inner `Vec<i32>` as one item, in
    /// order.
    ///
    /// TODO: implement this. Same idea as `MemoryBuffer::new`: keep the
    /// batches plus a cursor so `Source::next` can hand out one `Vec<i32>`
    /// per call and return `None` once they're exhausted.
    pub fn new(batches: Vec<Vec<i32>>) -> Self {
        let _ = batches;
        todo!()
    }
}

/// TODO: implement `next`. Return the next string in insertion order,
/// advancing whatever cursor you stored in `MemoryBuffer::new`, or `None`
/// once every element has already been returned.
impl Source for MemoryBuffer {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        todo!()
    }
}

/// TODO: implement `next`. Return 1, then 2, ..., up to and including
/// `up_to`, then `None` forever after (including when `up_to == 0`, where
/// the very first call must already return `None`).
impl Source for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        todo!()
    }
}

/// TODO: implement `next`. Return each stored `Vec<i32>` batch in order,
/// then `None` once they've all been returned.
impl Source for Batched {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Vec<i32>> {
        todo!()
    }
}

/// Consumes the source into a `Vec`, in the order produced by `next`.
///
/// TODO: implement this. Loop calling `source.next()` until it returns
/// `None`, pushing each `Some(item)` into a `Vec` that you return at the
/// end. An empty source must yield an empty `Vec`.
pub fn download_all<S: Source>(source: S) -> Vec<S::Item> {
    let _ = source;
    todo!()
}

/// Human-readable summary of the source: the first 3 elements (formatted
/// with `Display`) joined by `", "`; if more than 3 elements exist, appends
/// the literal suffix `" ... e altri K"` where `K` is the count of the
/// remaining elements (left in Italian on purpose — the tests assert this
/// exact string). An empty source returns the literal `"vuoto"` (same
/// reason).
///
/// TODO: implement this. Drain the source (e.g. via `download_all`), then
/// build the string: handle the empty case first, then join
/// `elements[..elements.len().min(3)]` with `", "` using `Display`, then
/// append the `" ... e altri {}"` suffix only when there are more than 3
/// elements, with `elements.len() - 3` as `K`.
pub fn summary<S>(source: S) -> String
where
    S: Source,
    S::Item: Display,
{
    let _ = source;
    todo!()
}

// ---- Version with generic on the trait: educational comparison ----

pub trait SourceGen<E> {
    fn next_gen(&mut self) -> Option<E>;
}

/// TODO: implement `next_gen`. Behaves like `Source::next` for
/// `MemoryBuffer`: return the next string in order, `None` once exhausted.
impl SourceGen<String> for MemoryBuffer {
    fn next_gen(&mut self) -> Option<String> {
        todo!()
    }
}

/// Same type, second implementation: produces the LENGTHS of the strings.
/// With the associated type this would be impossible: explain why in the
/// mastery-question comment.
///
/// TODO: implement `next_gen`. Same cursor logic as the `String`
/// implementation above, but return `Some(s.len())` instead of `Some(s)` for
/// each string.
impl SourceGen<usize> for MemoryBuffer {
    fn next_gen(&mut self) -> Option<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buffer(words: &[&str]) -> MemoryBuffer {
        MemoryBuffer::new(words.iter().map(|s| s.to_string()).collect())
    }

    #[test]
    fn counter_generates_from_one_to_n() {
        assert_eq!(download_all(Counter::new(4)), vec![1, 2, 3, 4]);
        assert_eq!(download_all(Counter::new(0)), Vec::<u32>::new());
    }

    #[test]
    fn buffer_returns_in_order() {
        let b = buffer(&["a", "b", "c"]);
        assert_eq!(download_all(b), vec!["a", "b", "c"]);
    }

    #[test]
    fn batches_produce_collections() {
        let l = Batched::new(vec![vec![1, 2], vec![3]]);
        assert_eq!(download_all(l), vec![vec![1, 2], vec![3]]);
    }

    #[test]
    fn summary_short() {
        assert_eq!(summary(buffer(&["a", "b"])), "a, b");
        assert_eq!(summary(Counter::new(3)), "1, 2, 3");
    }

    #[test]
    fn summary_long() {
        assert_eq!(summary(Counter::new(5)), "1, 2, 3 ... e altri 2");
    }

    #[test]
    fn summary_empty() {
        assert_eq!(summary(buffer(&[])), "vuoto");
    }

    #[test]
    fn generic_over_trait_requires_disambiguation() {
        let mut b = buffer(&["osu", "kiai"]);
        // The caller MUST say which implementation it wants:
        let first: Option<String> = SourceGen::<String>::next_gen(&mut b);
        assert_eq!(first.as_deref(), Some("osu"));

        let mut b2 = buffer(&["osu", "kiai"]);
        let lengths: Option<usize> = SourceGen::<usize>::next_gen(&mut b2);
        assert_eq!(lengths, Some(3));
    }
}
