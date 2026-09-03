//! # Problem 7 — Plugin-based processing pipeline
//! **Concepts:** trait objects, `Box<dyn Trait>`, object safety, cloneability.
//!
//! Runtime-configurable transformations, applied in sequence:
//! - trim: collapses redundant whitespace (runs of spaces → a single one, and trims the edges)
//! - upper: all uppercase
//! - censor:WORD — replaces every occurrence with asterisks (same length)
//! - truncate:N — cuts to N characters
//!
//! Requirements (encoded in the tests):
//! 1. The transformations live together in the SAME collection (heterogeneous).
//! 2. `process` returns (result, report) where report = names of the
//!    transformations applied, in order, separated by " -> ".
//! 3. `parse_pipeline` builds the pipeline from a config string like
//!    "trim,upper,censura:segreto,tronca:50" with typed errors.
//! 4. The whole pipeline must be cloneable (to save "presets").
//!    The obvious route (`Clone` as a supertrait) does NOT compile: try it,
//!    read the error, explain why it violates object safety, and use the
//!    `clone_box` workaround already present in the trait signature.
//! 5. Final comment: compare with Problem 4 (performance, runtime
//!    flexibility, binary size).
//!
//! MASTERY QUESTIONS (answer below in a comment):
//! - The main object-safety rules: which one would you have violated in point 4?
//! - What is a "fat pointer" and what does it contain in the case of `&dyn Transformation`?

use std::fmt;

pub trait Transformation {
    fn name(&self) -> &str;
    fn apply(&self, text: &str) -> String;
    /// Idiomatic workaround for making trait objects cloneable.
    fn clone_box(&self) -> Box<dyn Transformation>;
}

// This impl makes Box<dyn Transformation> cloneable, and therefore
// Vec<Box<dyn Transformation>> too: complete the body.
//
/// TODO: implement this by delegating to `clone_box`: every concrete type
/// behind the trait object implements `clone_box` (see below), so the body
/// here is simply `self.clone_box()`. This is the standard workaround for
/// the fact that `Clone` cannot be added as a supertrait of an
/// object-safe trait (its `fn clone(&self) -> Self` returns `Self` by
/// value, which is not object-safe — the compiler cannot know the size of
/// the concrete type behind a `dyn Trait` at compile time).
impl Clone for Box<dyn Transformation> {
    fn clone(&self) -> Self {
       self.clone_box()
    }
}

#[derive(Clone)]
pub struct Trim;

#[derive(Clone)]
pub struct Upper;

#[derive(Clone)]
pub struct Censor {
    pub word: String,
}

#[derive(Clone)]
pub struct Truncate {
    pub max: usize,
}

impl Transformation for Trim {
    fn name(&self) -> &str {
        "trim"
    }
    /// TODO: implement this. Collapse every run of one-or-more whitespace
    /// characters into a single space, and trim leading/trailing whitespace
    /// from the result. Hint: `text.split_whitespace()` already skips runs
    /// of whitespace and empty tokens, so
    /// `text.split_whitespace().collect::<Vec<_>>().join(" ")` gets you
    /// there directly — no manual character-by-character loop needed.
    /// Edge case: an empty or all-whitespace input should produce an empty
    /// string.
    fn apply(&self, text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }
    fn clone_box(&self) -> Box<dyn Transformation> {
        Box::new(self.clone())
    }
}

impl Transformation for Upper {
    fn name(&self) -> &str {
        "upper"
    }
    /// TODO: implement this. Return the uppercase version of `text`.
    /// Hint: `str::to_uppercase()` handles this (careful: `to_ascii_uppercase`
    /// would also work for plain ASCII input, but `to_uppercase` is the more
    /// general/correct choice for Unicode).
    fn apply(&self, text: &str) -> String {
        text.to_uppercase()
    }
    fn clone_box(&self) -> Box<dyn Transformation> {
        Box::new(self.clone())
    }
}

impl Transformation for Censor {
    fn name(&self) -> &str {
        "censura"
    }
    /// TODO: implement this. Replace every occurrence of `self.word` inside
    /// `text` with a run of asterisks of the SAME LENGTH as the matched
    /// word (see the test: "il segreto è qui" with word "segreto" becomes
    /// "il ******* è qui" — 7 letters, 7 asterisks). Hint:
    /// `text.replace(&self.word, &"*".repeat(self.word.len()))` does
    /// exactly this in one call. Edge case: an empty `word` should leave
    /// `text` unchanged (make sure your approach doesn't loop forever or
    /// insert asterisks between every character in that case).
    fn apply(&self, text: &str) -> String {
        if self.word.is_empty() {
            return text.to_string();
        }
        text.replace(&self.word, &"*".repeat(self.word.len()))
    }
    fn clone_box(&self) -> Box<dyn Transformation> {
        Box::new(self.clone())
    }
}

impl Transformation for Truncate {
    fn name(&self) -> &str {
        "tronca"
    }
    /// TODO: implement this. Cut `text` to at most `self.max` characters.
    /// Edge cases: if `text` is already shorter than or equal to
    /// `self.max`, return it unchanged; if `self.max` is `0`, return an
    /// empty string. Careful: byte-slicing `&text[..self.max]` is only
    /// safe for ASCII — for correctness with multi-byte UTF-8 characters,
    /// prefer `text.chars().take(self.max).collect::<String>()`, which
    /// truncates by character count rather than by byte offset.
    fn apply(&self, text: &str) -> String {
        text.chars().take(self.max).collect()
    }
    fn clone_box(&self) -> Box<dyn Transformation> {
        Box::new(self.clone())
    }
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    UnknownTransformation(String),
    MissingParameter(String),
    InvalidParameter { transformation: String, value: String },
}

impl fmt::Display for ConfigError {
    /// TODO: implement this. Match on `self` and `write!` a human-readable
    /// message to `f` for each variant, e.g. something like:
    /// - `UnknownTransformation(name)` → "unknown transformation: {name}"
    /// - `MissingParameter(name)` → "missing parameter for: {name}"
    /// - `InvalidParameter { transformation, value }` → "invalid parameter
    ///   '{value}' for {transformation}"
    /// The exact wording is up to you (the tests only check the typed
    /// variants via `PartialEq`, not the `Display` text), but every arm
    /// must return the `fmt::Result` produced by `write!`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            ConfigError::UnknownTransformation(name)=>{
                write!(f,"unknown trasformation:{name}")
            }
            ConfigError::MissingParameter(name)=>{
                write!(f,"missing parameter for:{name}")
            }
             ConfigError::InvalidParameter { transformation, value } => {
            write!(f, "invalid parameter '{value}' for {transformation}")
        }
        }
    }
}

impl std::error::Error for ConfigError {}

/// TODO: implement this. Apply each transformation in `pipeline` to `text`
/// in order (each one consuming the previous one's output), and build a
/// report string of the applied transformation names joined by " -> ".
/// Hint: fold over `pipeline` carrying `(String, Vec<&str>)` (or use two
/// separate folds/loops) — start with `text.to_string()` and, for each
/// transformation, replace the running text with `t.apply(&text)` and push
/// `t.name()` onto the names list; at the end join the names with " -> ".
/// Edge case: an empty pipeline must return
/// `(text.to_string(), String::new())` (see the `empty_pipeline` test).
pub fn process(text: &str, pipeline: &[Box<dyn Transformation>]) -> (String, String) {
    let (final_text, names): (String, Vec<&str>) = pipeline.iter().fold(
        (text.to_string(), Vec::new()),
        |(text, mut names), t| {
            let text = t.apply(&text);
            names.push(t.name());
            (text, names)
        },
    );

    (final_text, names.join(" -> "))
}

/// TODO: implement this. Split `config` on `,` to get one spec per
/// transformation; for each spec, split on the first `:` to separate the
/// name from an optional parameter. Build the right transformation:
/// - "trim" → `Trim`
/// - "upper" → `Upper`
/// - "censura:WORD" → `Censor { word: WORD.to_string() }` (missing WORD →
///   `ConfigError::MissingParameter("censura".into())`)
/// - "tronca:N" → `Truncate { max: N }`, parsing N with
///   `str::parse::<usize>()` (missing N → `ConfigError::MissingParameter
///   ("tronca".into())`; present but not a valid `usize` →
///   `ConfigError::InvalidParameter { transformation: "tronca".into(),
///   value: N.into() }`)
/// - anything else → `ConfigError::UnknownTransformation(name.into())`
/// Box each transformation (`Box::new(...) as Box<dyn Transformation>`) and
/// collect them into a `Vec`, short-circuiting on the first error (a `for`
/// loop with early `return Err(...)`, or
/// `.map(...).collect::<Result<Vec<_>, _>>()`, both work).
pub fn parse_pipeline(config: &str) -> Result<Vec<Box<dyn Transformation>>, ConfigError> {
    config
        .split(',')
        .map(|c| -> Result<Box<dyn Transformation>, ConfigError> {
            let (nome, parametro) = match c.split_once(':') {
                Some((n, p)) => (n, Some(p)),
                None => (c, None),
            };

            match nome {
                "trim" => Ok(Box::new(Trim)),
                "upper" => Ok(Box::new(Upper)),
                "censura" => {
                    let word = parametro
                        .ok_or_else(|| ConfigError::MissingParameter(nome.to_string()))?;
                    Ok(Box::new(Censor { word: word.to_string() }))
                }
                "tronca" => {
                    let value = parametro
                        .ok_or_else(|| ConfigError::MissingParameter(nome.to_string()))?;
                    let max: usize = value.parse().map_err(|_| ConfigError::InvalidParameter {
                        transformation: nome.to_string(),
                        value: value.to_string(),
                    })?;
                    Ok(Box::new(Truncate { max }))
                }
                _ => Err(ConfigError::UnknownTransformation(nome.to_string())),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_transformations() {
        assert_eq!(Trim.apply("  ciao   mondo  "), "ciao mondo");
        assert_eq!(Upper.apply("osu"), "OSU");
        assert_eq!(
            Censor { word: "segreto".into() }.apply("il segreto è qui"),
            "il ******* è qui"
        );
        assert_eq!(Truncate { max: 4 }.apply("kyokushin"), "kyok");
        assert_eq!(Truncate { max: 20 }.apply("kata"), "kata");
    }

    #[test]
    fn heterogeneous_collection_in_sequence() {
        let pipeline: Vec<Box<dyn Transformation>> = vec![
            Box::new(Trim),
            Box::new(Upper),
            Box::new(Truncate { max: 7 }),
        ];
        let (out, report) = process("  hello   world  ", &pipeline);
        assert_eq!(out, "HELLO W");
        assert_eq!(report, "trim -> upper -> tronca");
    }

    #[test]
    fn empty_pipeline() {
        let pipeline: Vec<Box<dyn Transformation>> = vec![];
        let (out, report) = process("intatto", &pipeline);
        assert_eq!(out, "intatto");
        assert_eq!(report, "");
    }

    #[test]
    fn parse_valid_configuration() {
        let p = parse_pipeline("trim,upper,censura:segreto,tronca:50").unwrap();
        assert_eq!(p.len(), 4);
        let names: Vec<&str> = p.iter().map(|t| t.name()).collect();
        assert_eq!(names, ["trim", "upper", "censura", "tronca"]);
    }

    #[test]
    fn parse_errors() {
        assert_eq!(
            parse_pipeline("trim,esplodi").err(),
            Some(ConfigError::UnknownTransformation("esplodi".into()))
        );
        assert_eq!(
            parse_pipeline("censura").err(),
            Some(ConfigError::MissingParameter("censura".into()))
        );
        assert_eq!(
            parse_pipeline("tronca:abc").err(),
            Some(ConfigError::InvalidParameter {
                transformation: "tronca".into(),
                value: "abc".into()
            })
        );
    }

    #[test]
    fn cloneable_preset() {
        let preset = parse_pipeline("trim,upper").unwrap();
        let copy = preset.clone();
        let (a, _) = process("  x  y ", &preset);
        let (b, _) = process("  x  y ", &copy);
        assert_eq!(a, b);
        assert_eq!(a, "X Y");
    }
}
