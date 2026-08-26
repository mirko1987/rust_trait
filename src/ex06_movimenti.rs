//! # Problem 6 — Bank transaction importer
//! **Concepts:** `?` operator, `From` between errors, conversion chains.
//!
//! Line format: "date;description;amount"  e.g. "2026-08-15;palestra;-45.90"
//! Date: YYYY-MM-DD, simplified validation (month 1-12, day 1-31).
//!
//! Requirements (encoded in the tests):
//! 1. Two levels of error: field-level (`FieldError`) and line-level
//!    (`LineError`).
//! 2. `parse_transaction` propagates ONLY with `?`: no `match`/`map_err` on
//!    errors in the body. All conversion logic lives in the `From` impls.
//! 3. The standard library's `ParseFloatError` must bubble all the way up
//!    to `LineError` through `FieldError`: solve the double-conversion
//!    problem (`?` applies only ONE) and document the rule.
//! 4. `import_all`: everything or the first error, body = a SINGLE iterator
//!    expression (find out what `collect` can do with `Result`s).
//! 5. `import_resilient`: (valid ones, errors with 0-based line number).
//!
//! MASTERY QUESTIONS (answer below in a comment):
//! - How many `From` conversions does `?` apply? How did you verify it?
//! - In `import_all`, what happens when `collect` encounters the first
//!   `Err`?

use std::num::ParseFloatError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub date: Date,
    pub description: String,
    pub amount: f64,
}

#[derive(Debug, PartialEq)]
pub enum FieldError {
    MalformedDate(String),
    NonNumericAmount(String),
    EmptyDescription,
}

#[derive(Debug, PartialEq)]
pub enum LineError {
    WrongFieldCount(usize),
    Field(FieldError),
}

/// Wraps a field-level error into a line-level error.
///
/// TODO: implement this. This is the "inner" conversion the `?` operator
/// applies when a `FieldError` needs to become a `LineError` — simply wrap
/// the incoming value in `LineError::Field`. There's no branching or edge
/// case here, just a straightforward variant wrap.
impl From<FieldError> for LineError {
    fn from(e: FieldError) -> Self {
        let _ = e;
        todo!()
    }
}

// At least one more `From` impl is missing here: find out which one by
// reading the compiler errors when you write `parse_transaction` using
// only `?`.
// Note: `ParseFloatError` does not carry the original string — you'll need
// to decide how to get the offending input into the error.
/// Converts a failed float parse into a field-level error.
///
/// TODO: implement this. Tricky part: `ParseFloatError` does not retain the
/// original string that failed to parse, so wrapping it directly loses the
/// information `FieldError::NonNumericAmount(String)` needs. You'll have to
/// rethink how the offending text reaches this conversion — for example by
/// not relying on `?` doing a direct `ParseFloatError -> FieldError` jump
/// at the call site, but instead handling the parse failure where the
/// original string is still in scope (see the double-conversion mastery
/// question in the module doc comment: `?` performs only ONE `From`
/// conversion per `?`, so chaining `ParseFloatError -> FieldError ->
/// LineError` only works automatically if each step is a proper `From`
/// impl AND you still have access to the string at the point where you
/// construct the error).
impl From<ParseFloatError> for FieldError {
    fn from(e: ParseFloatError) -> Self {
        let _ = e;
        todo!("careful: you don't have access to the original string here...")
    }
}

/// Parses a `YYYY-MM-DD` date string with simplified validation (month
/// 1-12, day 1-31 regardless of the actual month length or leap years).
///
/// TODO: implement this. Split on `-` and check you get exactly 3 parts;
/// parse each part as the right integer type (`u16` for year, `u8` for
/// month/day) and validate the ranges. On ANY failure — wrong number of
/// parts, a part that doesn't parse as an integer, or a month/day out of
/// range — return `FieldError::MalformedDate(field.to_string())` with the
/// ORIGINAL (untouched) input string, not a partially parsed value: the
/// tests check the error carries back exactly what was passed in.
pub fn parse_date(field: &str) -> Result<Date, FieldError> {
    let _ = field;
    todo!()
}

/// Parses one line of the form `"date;description;amount"` into a
/// [`Transaction`].
///
/// TODO: implement this using ONLY the `?` operator to propagate errors —
/// no `match`/`map_err` in this body; all the conversion logic must live in
/// the `From` impls above. Steps:
/// 1. Split the line on `;`. If you don't get exactly 3 fields, return
///    `LineError::WrongFieldCount(n)` directly (this is a line-level
///    concern, not a field-level one, so it bypasses the `From` chain).
/// 2. Parse the date field with `parse_date` (propagates as `FieldError`,
///    then via `?` into `LineError`).
/// 3. Validate the description is non-empty, else
///    `FieldError::EmptyDescription`.
/// 4. Parse the amount with `str::parse::<f64>()`; its `ParseFloatError`
///    needs to reach `LineError` through your `From<ParseFloatError> for
///    FieldError` impl — remember `?` only performs ONE `From` conversion
///    per `?`, so a direct `ParseFloatError -> LineError` jump does NOT
///    happen automatically just because both intermediate `From` impls
///    exist; check how the types actually chain.
pub fn parse_transaction(line: &str) -> Result<Transaction, LineError> {
    let _ = line;
    todo!("only `?` to propagate field-level errors")
}

/// Everything or the first error. Body: a single iterator expression.
///
/// TODO: implement this. Split `statement` into lines, map each through
/// `parse_transaction`, and `collect()` into a `Result<Vec<Transaction>,
/// LineError>` — find out what `FromIterator` impl `Result` provides: it
/// short-circuits on the first `Err`, which is exactly requirement 4. Do
/// not write a manual loop; this should be one iterator chain ending in
/// `.collect()`.
pub fn import_all(statement: &str) -> Result<Vec<Transaction>, LineError> {
    let _ = statement;
    todo!()
}

/// Doesn't stop on errors: returns (valid transactions, errors paired with
/// their 0-based line index).
///
/// TODO: implement this. Iterate over `statement.lines()` with
/// `.enumerate()` so you have the 0-based line index, call
/// `parse_transaction` on each line, and partition the results into two
/// `Vec`s: one collecting the `Ok` values, the other collecting `(index,
/// error)` pairs for the `Err` values. Unlike `import_all`, this must
/// process every line regardless of earlier failures — no early return, no
/// `?`.
pub fn import_resilient(statement: &str) -> (Vec<Transaction>, Vec<(usize, LineError)>) {
    let _ = statement;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_line() {
        let tx = parse_transaction("2026-08-15;palestra;-45.90").unwrap();
        assert_eq!(tx.date, Date { year: 2026, month: 8, day: 15 });
        assert_eq!(tx.description, "palestra");
        assert_eq!(tx.amount, -45.9);
    }

    #[test]
    fn wrong_field_count() {
        assert_eq!(
            parse_transaction("solo;due"),
            Err(LineError::WrongFieldCount(2))
        );
        assert_eq!(
            parse_transaction("a;b;c;d"),
            Err(LineError::WrongFieldCount(4))
        );
    }

    #[test]
    fn malformed_date() {
        for input in ["2026/08/15", "2026-13-01", "2026-00-10", "2026-01-32", "abcd-01-01"] {
            let line = format!("{input};x;1.0");
            match parse_transaction(&line) {
                Err(LineError::Field(FieldError::MalformedDate(s))) => {
                    assert_eq!(s, input)
                }
                other => panic!("expected MalformedDate for {input}, got {other:?}"),
            }
        }
    }

    #[test]
    fn empty_description() {
        assert_eq!(
            parse_transaction("2026-08-15;;1.0"),
            Err(LineError::Field(FieldError::EmptyDescription))
        );
    }

    #[test]
    fn non_numeric_amount() {
        assert!(matches!(
            parse_transaction("2026-08-15;bar;dodici"),
            Err(LineError::Field(FieldError::NonNumericAmount(_)))
        ));
    }

    #[test]
    fn import_stops_at_first_error() {
        let statement = "2026-08-15;a;1.0\nrotta\n2026-08-16;b;2.0";
        assert_eq!(import_all(statement), Err(LineError::WrongFieldCount(1)));
    }

    #[test]
    fn import_all_valid() {
        let statement = "2026-08-15;a;1.0\n2026-08-16;b;2.0";
        let txs = import_all(statement).unwrap();
        assert_eq!(txs.len(), 2);
        assert_eq!(txs[1].amount, 2.0);
    }

    #[test]
    fn resilient_collects_everything() {
        let statement = "2026-08-15;a;1.0\nrotta\n2026-08-16;;2.0\n2026-08-17;c;3.0";
        let (valid, errors) = import_resilient(statement);
        assert_eq!(valid.len(), 2);
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].0, 1);
        assert_eq!(errors[1].0, 2);
        assert_eq!(
            errors[1].1,
            LineError::Field(FieldError::EmptyDescription)
        );
    }
}
