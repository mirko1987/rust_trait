//! # Problem 3 — Product code validator
//! **Concepts:** `Result`, error enum, `match`, early return, typed domain.
//!
//! Code format: "CAT-NNNN-R"
//! - CAT: 3 uppercase ASCII letters
//! - NNNN: 4 digits, value between 0001 and 9999
//! - R: revision between A and F
//!
//! Requirements (encoded in the tests):
//! 1. `parse_code` returns a typed `ProductCode`. The revision is an enum:
//!    an invalid value is unrepresentable by construction.
//! 2. Every error variant carries the offending data.
//! 3. The error implements `Display` (clear, human-readable messages for a
//!    warehouse operator) and `std::error::Error`.
//! 4. `validate_list` returns the first error enriched with its index.
//!
//! MASTERY QUESTIONS (answer below in a comment):
//! - Three concrete advantages of an error enum over `Result<_, String>`.
//! - What does "making invalid states unrepresentable" mean, and where did
//!   you apply it here?
//
// ANSWERS:
//
// Three advantages of an error enum (`CodeError`) over `Result<_, String>`:
// 1. Exhaustive matching: the compiler forces every `match` on `CodeError`
//    to handle all four variants (or an explicit `_`), so adding a new
//    variant later surfaces every call site that needs updating. A
//    `String` gives no such guarantee — callers can only guess at the
//    possible error "shapes" from documentation or by re-parsing the text.
// 2. Structured, typed data attached to each error: `InvalidNumber(String)`
//    carries the exact offending section, `InvalidFormat(usize)` carries a
//    `usize` count, not a formatted sentence. Callers can inspect and reuse
//    that data programmatically instead of parsing an error message.
// 3. Callers can react differently per error kind (e.g. retry on
//    `InvalidFormat` but reject outright on `InvalidCategory`) via `match`,
//    without brittle string comparisons/parsing that break if the wording
//    of a message changes.
//
// "Making invalid states unrepresentable" means designing types so that
// values which shouldn't exist simply cannot be constructed, instead of
// being constructible and then checked/rejected at every use site. Applied
// here with `Revision`: instead of storing the revision as a `char` (which
// could hold any of ~1M possible values, only 6 of which are valid) and
// re-validating it every time it's read, `parse_code` validates it once at
// parse time and produces a `Revision` enum that can *only* ever be one of
// `A..=F`. From that point on, every piece of code holding a `Revision`
// is statically guaranteed to have a valid one — no runtime check needed.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Revision {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProductCode {
    pub category: String,
    pub number: u16,
    pub revision: Revision,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CodeError {
    /// Number of sections different from 3 (carries how many were found).
    InvalidFormat(usize),
    InvalidCategory(String),
    InvalidNumber(String),
    InvalidRevision(String),
}

/// TODO: implement this. `match` on `self` and write a clear, human-readable
/// message for each variant, embedding the offending value it carries, e.g.
/// something like "expected 3 sections, found {n}" for `InvalidFormat`,
/// "invalid category '{s}'" for `InvalidCategory`, and analogous messages
/// for `InvalidNumber`/`InvalidRevision`. Use `write!(f, "...", ...)` and
/// return its `fmt::Result` directly.
impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
       match self {
    CodeError::InvalidFormat(n) => write!(f, "expected 3 sections, found {n}"),
    CodeError::InvalidCategory(s) => write!(f, "invalid category '{s}'"),
    CodeError::InvalidNumber(s) => write!(f, "invalid number '{s}'"),
    CodeError::InvalidRevision(s) => write!(f, "invalid revision '{s}'"),
}
    }
}

impl std::error::Error for CodeError {}

/// TODO: implement this.
/// Steps:
/// 1. Split `input` on `-`. If the result doesn't have exactly 3 sections,
///    return `Err(CodeError::InvalidFormat(n))` where `n` is the number of
///    sections actually found (don't hardcode: count them).
/// 2. Validate the category section: exactly 3 ASCII uppercase letters
///    (check length and `char::is_ascii_uppercase` on every char), otherwise
///    `Err(CodeError::InvalidCategory(section.into()))`.
/// 3. Validate the number section: must be exactly 4 ASCII digit characters
///    (reject e.g. "042", which is only 3 digits, even though shorter
///    numeric strings still parse) and, once parsed, fall in `1..=9999`
///    (reject "0000"); otherwise `Err(CodeError::InvalidNumber(section.into()))`.
/// 4. Validate the revision section: must be exactly one uppercase letter in
///    `A..=F` (case-sensitive — lowercase "c" is invalid), otherwise
///    `Err(CodeError::InvalidRevision(section.into()))`.
/// 5. On success, build and return
///    `Ok(ProductCode { category, number, revision })`.
pub fn parse_code(input: &str) -> Result<ProductCode, CodeError> {
    let sections: Vec<&str> = input.split('-').collect();

    // 1. Deve avere esattamente 3 sezioni
    if sections.len() != 3 {
        return Err(CodeError::InvalidFormat(sections.len()));
    }
    let (cat_section, num_section, rev_section) = (sections[0], sections[1], sections[2]);

    // 2. Categoria: 3 lettere maiuscole ASCII
    if cat_section.len() != 3 || !cat_section.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(CodeError::InvalidCategory(cat_section.into()));
    }

    // 3. Numero: 4 cifre ASCII, poi valore in 1..=9999
    if num_section.len() != 4 || !num_section.chars().all(|c| c.is_ascii_digit()) {
        return Err(CodeError::InvalidNumber(num_section.into()));
    }
    let number: u16 = num_section.parse().unwrap(); // sicuro: sono 4 cifre valide
    if !(1..=9999).contains(&number) {
        return Err(CodeError::InvalidNumber(num_section.into()));
    }

    // 4. Revisione: una lettera in A..=F, maiuscola
    let mut chars = rev_section.chars();
    let revision = match (chars.next(), chars.next()) {
        (Some('A'), None) => Revision::A,
        (Some('B'), None) => Revision::B,
        (Some('C'), None) => Revision::C,
        (Some('D'), None) => Revision::D,
        (Some('E'), None) => Revision::E,
        (Some('F'), None) => Revision::F,
        _ => return Err(CodeError::InvalidRevision(rev_section.into())),
    };

    // 5. Tutto valido: costruisci il risultato
    Ok(ProductCode {
        category: cat_section.into(),
        number,
        revision,
    })
} 



/// Validates every code; on the first error, returns (index, error).
///
/// TODO: implement this. Iterate over `codes` with `.iter().enumerate()`,
/// call `parse_code` on each entry, and early-return as soon as one fails,
/// wrapping the failure as `Err((index, error))`. If every code parses
/// successfully, return `Ok(...)` with the `Vec<ProductCode>` of all parsed
/// results, in the same order as the input.
pub fn validate_list(codes: &[&str]) -> Result<Vec<ProductCode>, (usize, CodeError)> {
    let mut result = Vec::new();
    
    for(i,code) in codes.iter().enumerate(){
        match parse_code(code){
            Ok(pc)=>result.push(pc),
            Err(e)=>return Err((i,e)),
        }
    }
    Ok(result)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_code() {
        let c = parse_code("ABC-0042-C").unwrap();
        assert_eq!(c.category, "ABC");
        assert_eq!(c.number, 42);
        assert_eq!(c.revision, Revision::C);
    }

    #[test]
    fn invalid_format_counts_sections() {
        assert_eq!(parse_code("ABC-0042"), Err(CodeError::InvalidFormat(2)));
        assert_eq!(
            parse_code("A-B-C-D"),
            Err(CodeError::InvalidFormat(4))
        );
    }

    #[test]
    fn invalid_category() {
        assert_eq!(
            parse_code("AbC-0042-C"),
            Err(CodeError::InvalidCategory("AbC".into()))
        );
        assert_eq!(
            parse_code("ABCD-0042-C"),
            Err(CodeError::InvalidCategory("ABCD".into()))
        );
    }

    #[test]
    fn invalid_number() {
        // 0000 is out of range
        assert_eq!(
            parse_code("ABC-0000-C"),
            Err(CodeError::InvalidNumber("0000".into()))
        );
        // not numeric
        assert_eq!(
            parse_code("ABC-12x4-C"),
            Err(CodeError::InvalidNumber("12x4".into()))
        );
        // 3 digits: wrong numeric format
        assert_eq!(
            parse_code("ABC-042-C"),
            Err(CodeError::InvalidNumber("042".into()))
        );
    }

    #[test]
    fn invalid_revision() {
        assert_eq!(
            parse_code("ABC-0042-G"),
            Err(CodeError::InvalidRevision("G".into()))
        );
        assert_eq!(
            parse_code("ABC-0042-c"),
            Err(CodeError::InvalidRevision("c".into()))
        );
    }

    #[test]
    fn display_is_readable() {
        let msg = format!("{}", CodeError::InvalidNumber("12x4".into()));
        assert!(msg.contains("12x4"));
        assert!(!msg.is_empty());
    }

    #[test]
    fn list_reports_index() {
        let codes = ["ABC-0001-A", "XYZ-9999-F", "ABC-0000-A"];
        let err = validate_list(&codes).unwrap_err();
        assert_eq!(err.0, 2);
        assert_eq!(err.1, CodeError::InvalidNumber("0000".into()));
    }

    #[test]
    fn list_all_valid() {
        let codes = ["ABC-0001-A", "XYZ-9999-F"];
        let ok = validate_list(&codes).unwrap();
        assert_eq!(ok.len(), 2);
    }
}
