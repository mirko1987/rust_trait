//! # Problem 9 — Error architecture for a full service
//! **Concepts:** `Error::source`, layered errors, `Box<dyn Error>`, error API
//! design, dependency injection via traits.
//!
//! Meeting-room booking service across three layers:
//! - Request parsing: "room;YYYY-MM-DD;start-hour;duration-minutes"
//! - Business: duration 15..=240 min, start 8..=20, room must exist
//! - Simulated storage: can fail with an opaque error
//!
//! Requirements (encoded in the tests):
//! 1. One error type per layer; `ServiceError` wraps them all. The storage
//!    error is kept as an INSPECTABLE CAUSE, never flattened into a string.
//! 2. `source()` correctly implemented on `ServiceError`.
//! 3. `error_chain` is GENERIC: it works with any error, and produces
//!    "msg <- cause <- deeper cause" by following `source()` all the way
//!    down.
//! 4. `book` traverses the three layers with `?`.
//! 5. The room list is injected via a trait (`RoomList`) for testability;
//!    same for storage (`Storage`). The tests use fake implementations.
//! 6. Design to justify in writing: public enum vs. opaque type with
//!    inspection methods; the role of `#[non_exhaustive]`.
//!
//! MASTERY QUESTIONS (answer below in a comment):
//! - Difference between wrapping a cause (`source`) and formatting it into
//!   the Display message: why is the former more powerful?
//! - What is the `'static` bound in `Box<dyn Error>` / `source()` for?

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParsingError {
    WrongFieldCount(usize),
    MalformedDate(String),
    InvalidNumber(String),
}

#[derive(Debug)]
pub enum BusinessError {
    DurationOutOfRange(u32),
    InvalidStartTime(u32),
    RoomNotFound(String),
}

/// Opaque error, in the style of a database driver.
#[derive(Debug)]
pub struct StorageError(pub String);

#[derive(Debug)]
pub enum ServiceError {
    Parsing(ParsingError),
    Business(BusinessError),
    Storage(StorageError),
}

impl fmt::Display for ParsingError {
    /// Implement this: write a human-readable message with `write!(f, ...)`.
    /// Match on `self` and produce distinct text per variant, e.g.
    /// `WrongFieldCount(n)` -> mention that 4 fields were expected but `n`
    /// were found; `MalformedDate(s)` / `InvalidNumber(s)` -> mention the
    /// offending substring `s`. Return the `fmt::Result` produced by
    /// `write!`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!()
    }
}
impl Error for ParsingError {}

impl fmt::Display for BusinessError {
    /// Implement this: write a human-readable message with `write!(f, ...)`.
    /// Match on `self` and produce distinct text per variant, e.g.
    /// `DurationOutOfRange(n)` -> mention the invalid duration `n`;
    /// `InvalidStartTime(n)` -> mention the invalid start hour `n`;
    /// `RoomNotFound(name)` -> mention the missing room `name`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!()
    }
}
impl Error for BusinessError {}

impl fmt::Display for StorageError {
    /// Implement this: write a human-readable message with `write!(f, ...)`
    /// that includes the wrapped `self.0` string (e.g. "storage error:
    /// {}").
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!()
    }
}
impl Error for StorageError {}

impl fmt::Display for ServiceError {
    /// Implement this: match on the active layer (`Parsing`/`Business`/
    /// `Storage`) and produce a short message identifying which layer
    /// failed, WITHOUT repeating the wrapped error's own text — that text
    /// already lives in `source()`, and duplicating it here would make it
    /// appear twice for any caller that prints both the error and its
    /// chain.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!("layer message, WITHOUT repeating the cause: that lives in source()")
    }
}

impl Error for ServiceError {
    /// Implement this: match on `self` and return `Some(&inner)` where
    /// `inner` is the wrapped `ParsingError` / `BusinessError` /
    /// `StorageError` — each of those types already implements
    /// `std::error::Error`, so a shared reference to it coerces to
    /// `&(dyn Error + 'static)` automatically.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}

impl From<ParsingError> for ServiceError {
    /// Implement this: wrap `e` in the matching `ServiceError::Parsing`
    /// variant. This lets `?` convert a `ParsingError` into a
    /// `ServiceError` automatically inside functions that return
    /// `Result<_, ServiceError>`.
    fn from(e: ParsingError) -> Self {
        let _ = e;
        todo!()
    }
}
impl From<BusinessError> for ServiceError {
    /// Implement this: wrap `e` in the matching `ServiceError::Business`
    /// variant, symmetric to the `ParsingError` conversion above.
    fn from(e: BusinessError) -> Self {
        let _ = e;
        todo!()
    }
}
impl From<StorageError> for ServiceError {
    /// Implement this: wrap `e` in the matching `ServiceError::Storage`
    /// variant, symmetric to the conversions above.
    fn from(e: StorageError) -> Self {
        let _ = e;
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub room: String,
    pub date: String,
    pub start_hour: u32,
    pub duration_minutes: u32,
}

#[derive(Debug, PartialEq)]
pub struct Confirmation {
    pub room: String,
    pub date: String,
    pub start_hour: u32,
    pub duration_minutes: u32,
}

pub trait RoomList {
    fn exists(&self, name: &str) -> bool;
}

pub trait Storage {
    fn save(&self, request: &Request) -> Result<(), StorageError>;
}

/// Implement this: parse `input` formatted as
/// "room;YYYY-MM-DD;start-hour;duration-minutes" (semicolon-separated).
/// Split on `;` and check you get exactly 4 fields — if not, return
/// `ParsingError::WrongFieldCount(actual_count)`. Parse the start-hour and
/// duration fields with `str::parse::<u32>`, returning
/// `ParsingError::InvalidNumber` (with the offending substring) on failure;
/// use `ParsingError::MalformedDate` if you choose to validate the date
/// field's shape too. Check the tests for the exact expectations.
pub fn parse_request(input: &str) -> Result<Request, ParsingError> {
    let _ = input;
    todo!()
}

/// Implement this: enforce the business rules on `request`:
/// - `request.duration_minutes` must be in `15..=240`, otherwise return
///   `BusinessError::DurationOutOfRange(request.duration_minutes)`.
/// - `request.start_hour` must be in `8..=20`, otherwise return
///   `BusinessError::InvalidStartTime(request.start_hour)`.
/// - `rooms.exists(&request.room)` must be `true`, otherwise return
///   `BusinessError::RoomNotFound(request.room.clone())`.
/// Mind the boundaries: 15, 240, 8 and 20 are all VALID (inclusive ranges),
/// see the `business_rules` test for the exact edge cases.
pub fn validate(request: &Request, rooms: &impl RoomList) -> Result<(), BusinessError> {
    let _ = (request, rooms);
    todo!()
}

/// Implement this: traverse the three layers with `?`, letting the `From`
/// impls above convert each layer's error into `ServiceError`:
/// 1. `parse_request(input)?` to get a `Request`.
/// 2. `validate(&request, rooms)?` to enforce the business rules.
/// 3. `storage.save(&request)?` to persist it.
/// On success, build and return the matching `Confirmation` from the
/// validated `request`'s fields.
pub fn book(
    input: &str,
    rooms: &impl RoomList,
    storage: &impl Storage,
) -> Result<Confirmation, ServiceError> {
    let _ = (input, rooms, storage);
    todo!("three layers traversed with `?`")
}

/// GENERIC: works with any error. Follows source() all the way down.
/// Format: "msg" or "msg <- cause" or "msg <- cause <- cause2" ...
///
/// Implement this: start with `format!("{error}")`, then loop calling
/// `.source()` on the current error; for each `Some(cause)` found, append
/// " <- " followed by `format!("{cause}")` and keep going from `cause`.
/// Stop when `.source()` returns `None`. A
/// `while let Some(cause) = current.source() { ... current = cause; }`
/// loop is the natural shape here.
pub fn error_chain(error: &dyn Error) -> String {
    let _ = error;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeRoomList(Vec<&'static str>);
    impl RoomList for FakeRoomList {
        fn exists(&self, name: &str) -> bool {
            self.0.contains(&name)
        }
    }

    struct WorkingStorage;
    impl Storage for WorkingStorage {
        fn save(&self, _: &Request) -> Result<(), StorageError> {
            Ok(())
        }
    }

    struct BrokenStorage;
    impl Storage for BrokenStorage {
        fn save(&self, _: &Request) -> Result<(), StorageError> {
            Err(StorageError("connessione rifiutata".into()))
        }
    }

    fn rooms() -> FakeRoomList {
        FakeRoomList(vec!["andromeda", "orione"])
    }

    #[test]
    fn valid_booking() {
        let c = book("andromeda;2026-09-01;9;60", &rooms(), &WorkingStorage).unwrap();
        assert_eq!(
            c,
            Confirmation {
                room: "andromeda".into(),
                date: "2026-09-01".into(),
                start_hour: 9,
                duration_minutes: 60
            }
        );
    }

    #[test]
    fn parsing_error() {
        let e = book("solo;tre;campi", &rooms(), &WorkingStorage).unwrap_err();
        assert!(matches!(
            e,
            ServiceError::Parsing(ParsingError::WrongFieldCount(3))
        ));
    }

    #[test]
    fn business_rules() {
        let cases = [
            ("andromeda;2026-09-01;9;10", "duration too short"),
            ("andromeda;2026-09-01;9;300", "duration too long"),
            ("andromeda;2026-09-01;7;60", "start too early"),
            ("andromeda;2026-09-01;21;60", "start too late"),
            ("vega;2026-09-01;9;60", "nonexistent room"),
        ];
        for (input, case) in cases {
            let e = book(input, &rooms(), &WorkingStorage).unwrap_err();
            assert!(
                matches!(e, ServiceError::Business(_)),
                "case '{case}': expected a business error, got {e:?}"
            );
        }
        // The inclusive bounds must pass:
        assert!(book("andromeda;2026-09-01;8;15", &rooms(), &WorkingStorage).is_ok());
        assert!(book("andromeda;2026-09-01;20;240", &rooms(), &WorkingStorage).is_ok());
    }

    #[test]
    fn storage_error_is_kept_as_cause() {
        let e = book("andromeda;2026-09-01;9;60", &rooms(), &BrokenStorage).unwrap_err();
        let cause = e.source().expect("source() must expose the cause");
        assert!(format!("{cause}").contains("connessione rifiutata"));
    }

    #[test]
    fn error_chain_follows_sources() {
        let e = book("andromeda;2026-09-01;9;60", &rooms(), &BrokenStorage).unwrap_err();
        let chain = error_chain(&e);
        assert!(chain.contains(" <- "), "chain: {chain}");
        assert!(chain.contains("connessione rifiutata"), "chain: {chain}");
    }

    #[test]
    fn error_chain_is_generic() {
        // Must work with ANY error, even one with no causes:
        let simple = StorageError("boom".into());
        let chain = error_chain(&simple);
        assert!(!chain.contains(" <- "));
        assert!(chain.contains("boom"));
    }
}
