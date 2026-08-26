//! # Problem 10 — Final project: configurable rule engine
//! **Concepts:** it all comes together — associated error types, `Infallible`,
//! blanket impl, trait objects, `try_fold`, configuration errors vs
//! evaluation errors.
//!
//! An engine evaluates an amount through a chain of rules:
//! - max:THRESHOLD — blocks if the amount exceeds the threshold
//! - commissione:PERCENT — adds PERCENT% (cannot fail: the type says so)
//! - arrotonda — rounds to the nearest cent (cannot fail)
//! - blacklist:AMOUNT — blocks that exact amount
//!
//! Requirements (encoded in the tests):
//! 1. `Rule` has an ASSOCIATED error type. Infallible rules use
//!    `std::convert::Infallible`.
//! 2. Rules with different associated errors cannot live in the same
//!    trait-object collection: the `DynRule` trait unifies the error, and
//!    a SINGLE blanket impl bridges the two worlds (every `Rule` becomes
//!    automatically a `DynRule`, zero code per rule).
//! 3. `evaluate` uses `try_fold` and, when a rule blocks, also reports the
//!    NAME of the rule that blocked (`EvaluationError`).
//! 4. `parse_chain` builds the chain from the configuration, with typed
//!    errors DISTINCT from evaluation errors.
//! 5. The `block_stops_the_chain` test OBSERVABLY DEMONSTRATES that rules
//!    after the block are not executed (a spy rule using `Cell`).
//! 6. Written closing (half a page): the journey of a value through the
//!    system, naming static/dynamic dispatch, monomorphization, error
//!    conversions, object safety, and where they appear in YOUR code.
//!
//! MASTERY QUESTIONS (answer below in a comment):
//! - Why does the blanket impl require `R::Error: 'static`?
//! - What does `Infallible` communicate in the signature, and what
//!   advantage does it give over a "normal" error type that's simply never
//!   used?

use std::convert::Infallible;
use std::error::Error;
use std::fmt;

pub trait Rule {
    type Error: Error;
    fn name(&self) -> &str;
    fn apply(&self, amount: f64) -> Result<f64, Self::Error>;
}

// ---- Per-rule errors ----

#[derive(Debug, PartialEq)]
pub struct ThresholdExceeded {
    pub amount: f64,
    pub threshold: f64,
}

impl fmt::Display for ThresholdExceeded {
    /// TODO: implement this. Write a human-readable message using
    /// `self.amount` and `self.threshold` via `write!(f, ...)`, e.g.
    /// "amount 150 exceeds threshold 100". This is the message shown when
    /// the error is boxed as `Box<dyn Error>` and displayed — see test
    /// `error_reports_the_rule_name`, which only checks the formatted
    /// string is non-empty, so any reasonable message text works.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!()
    }
}
impl Error for ThresholdExceeded {}

#[derive(Debug, PartialEq)]
pub struct BlacklistedAmount(pub f64);

impl fmt::Display for BlacklistedAmount {
    /// TODO: implement this. Write a human-readable message using `self.0`
    /// (the blacklisted amount) via `write!(f, ...)`, e.g. "amount 666 is
    /// blacklisted". Same note as above: only non-emptiness of the output
    /// is checked by the tests.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!()
    }
}
impl Error for BlacklistedAmount {}

// ---- The rules ----

pub struct MaxLimit {
    pub threshold: f64,
}

pub struct Fee {
    pub percent: f64,
}

pub struct Round;

pub struct Blacklist {
    pub amounts: Vec<f64>,
}

impl Rule for MaxLimit {
    type Error = ThresholdExceeded;
    fn name(&self) -> &str {
        "max"
    }
    /// TODO: implement this. If `amount > self.threshold`, return
    /// `Err(ThresholdExceeded { amount, threshold: self.threshold })`;
    /// otherwise return `Ok(amount)` unchanged. Note the boundary: test
    /// `single_rules` checks that `amount == threshold` is ALLOWED (only
    /// strictly greater is blocked).
    fn apply(&self, amount: f64) -> Result<f64, ThresholdExceeded> {
        let _ = amount;
        todo!()
    }
}

impl Rule for Fee {
    type Error = Infallible;
    fn name(&self) -> &str {
        "commissione"
    }
    /// TODO: implement this. Cannot fail (`Infallible`), so always return
    /// `Ok(...)`. Add `self.percent` percent on top of `amount`, i.e.
    /// `amount * (1.0 + self.percent / 100.0)`.
    fn apply(&self, amount: f64) -> Result<f64, Infallible> {
        let _ = amount;
        todo!()
    }
}

impl Rule for Round {
    type Error = Infallible;
    fn name(&self) -> &str {
        "arrotonda"
    }
    /// TODO: implement this. Cannot fail (`Infallible`). Round `amount` to
    /// the nearest cent (2 decimal places). Hint:
    /// `(amount * 100.0).round() / 100.0`.
    fn apply(&self, amount: f64) -> Result<f64, Infallible> {
        let _ = amount;
        todo!("to the cent: 10.567 -> 10.57")
    }
}

impl Rule for Blacklist {
    type Error = BlacklistedAmount;
    fn name(&self) -> &str {
        "blacklist"
    }
    /// TODO: implement this. If `self.amounts` contains `amount` (exact
    /// match), return `Err(BlacklistedAmount(amount))`; otherwise
    /// `Ok(amount)`. Hint: `self.amounts.contains(&amount)` — comparing
    /// `f64` with `==` is fine here since the tests only use exact literal
    /// values.
    fn apply(&self, amount: f64) -> Result<f64, BlacklistedAmount> {
        let _ = amount;
        todo!()
    }
}

// ---- The object-safe bridge ----

pub trait DynRule {
    fn name_dyn(&self) -> &str;
    fn apply_dyn(&self, amount: f64) -> Result<f64, Box<dyn Error>>;
}

/// THE blanket impl: every `Rule` is automatically a `DynRule`.
/// Think about why the `'static` bound is needed (mastery question).
impl<R: Rule> DynRule for R
where
    R::Error: 'static,
{
    /// TODO: implement this. Simply forward to `Rule::name` (`self.name()`).
    fn name_dyn(&self) -> &str {
        todo!()
    }
    /// TODO: implement this. Call `self.apply(amount)` and convert the
    /// `Result<f64, R::Error>` into `Result<f64, Box<dyn Error>>` via
    /// `.map_err(|e| Box::new(e) as Box<dyn Error>)` — this needs
    /// `R::Error: 'static`, which is exactly the bound on this impl, since
    /// `Box<dyn Error>` requires its inner type to be `'static`.
    fn apply_dyn(&self, amount: f64) -> Result<f64, Box<dyn Error>> {
        let _ = amount;
        todo!()
    }
}

// ---- Evaluation ----

#[derive(Debug)]
pub struct EvaluationError {
    pub rule: String,
    pub cause: Box<dyn Error>,
}

/// Body: a single expression based on `try_fold`.
///
/// TODO: implement this as a `try_fold` over `chain`, starting from
/// `Ok(amount)`. For each rule, call `rule.apply_dyn(current)`, and on
/// error wrap it into `Err(EvaluationError { rule:
/// rule.name_dyn().to_string(), cause: err })` — returning that `Err` from
/// the closure stops the fold immediately, which is what test
/// `block_stops_the_chain` observes. On an empty chain, this must return
/// `Ok(amount)` unchanged.
pub fn evaluate(
    amount: f64,
    chain: &[Box<dyn DynRule>],
) -> Result<f64, EvaluationError> {
    let _ = (amount, chain);
    todo!()
}

// ---- Configuration ----

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    UnknownRule(String),
    MissingParameter(String),
    InvalidParameter { rule: String, value: String },
}

/// TODO: implement this. Split `config` on `,` into rule specs; each spec
/// is either a bare keyword (`"arrotonda"`) or `"keyword:param"` (e.g.
/// `"max:10000"`, `"commissione:1.5"`, `"blacklist:666.00"`). For an
/// unknown keyword, return `Err(ConfigError::UnknownRule(keyword.to_string()))`.
/// For a keyword that requires a parameter but has none, return
/// `Err(ConfigError::MissingParameter(keyword.to_string()))`. For a
/// parameter that fails to parse as `f64`, return
/// `Err(ConfigError::InvalidParameter { rule: keyword.to_string(), value:
/// param.to_string() })`. Build the corresponding rule struct (`MaxLimit`
/// for `"max"`, `Fee` for `"commissione"`, `Round` for `"arrotonda"`,
/// `Blacklist` for `"blacklist"`) for each spec, box it, and collect into a
/// `Vec<Box<dyn DynRule>>` in the same order as the input. Note:
/// `Blacklist` takes a `Vec<f64>` of amounts — with this config format,
/// each `blacklist:X` spec produces its own `Blacklist` rule with a
/// single-element vector.
pub fn parse_chain(config: &str) -> Result<Vec<Box<dyn DynRule>>, ConfigError> {
    let _ = config;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn empty_chain_leaves_unchanged() {
        let chain: Vec<Box<dyn DynRule>> = vec![];
        assert_eq!(evaluate(123.45, &chain).unwrap(), 123.45);
    }

    #[test]
    fn single_rules() {
        assert_eq!(MaxLimit { threshold: 100.0 }.apply(100.0), Ok(100.0));
        assert!(MaxLimit { threshold: 100.0 }.apply(100.01).is_err());
        let c = Fee { percent: 10.0 }.apply(200.0).unwrap();
        assert!((c - 220.0).abs() < 1e-9, "expected ~220.0, got {c}");
        assert_eq!(Round.apply(10.567), Ok(10.57));
        assert_eq!(Round.apply(10.0), Ok(10.0));
        assert!(Blacklist { amounts: vec![666.0] }.apply(666.0).is_err());
        assert_eq!(Blacklist { amounts: vec![666.0] }.apply(667.0), Ok(667.0));
    }

    #[test]
    fn full_chain() {
        let chain: Vec<Box<dyn DynRule>> = vec![
            Box::new(MaxLimit { threshold: 10_000.0 }),
            Box::new(Fee { percent: 1.5 }),
            Box::new(Round),
        ];
        // 100 -> passes the limit -> 101.5 -> rounded 101.5
        assert_eq!(evaluate(100.0, &chain).unwrap(), 101.5);
        // 333.33 -> 338.32995 -> 338.33
        assert_eq!(evaluate(333.33, &chain).unwrap(), 338.33);
    }

    #[test]
    fn error_reports_the_rule_name() {
        let chain: Vec<Box<dyn DynRule>> = vec![
            Box::new(Fee { percent: 0.0 }),
            Box::new(MaxLimit { threshold: 50.0 }),
        ];
        let e = evaluate(100.0, &chain).unwrap_err();
        assert_eq!(e.rule, "max");
        assert!(format!("{}", e.cause).len() > 0);
    }

    /// Spy rule: counts how many times it gets executed.
    struct Spy {
        executed: Cell<u32>,
    }

    impl Rule for Spy {
        type Error = Infallible;
        fn name(&self) -> &str {
            "spia"
        }
        fn apply(&self, amount: f64) -> Result<f64, Infallible> {
            self.executed.set(self.executed.get() + 1);
            Ok(amount)
        }
    }

    #[test]
    fn block_stops_the_chain() {
        // OBSERVABLE check that rules after a block do not run:
        // the spy uses Cell to count executions despite taking &self.
        let spy = Spy { executed: Cell::new(0) };
        let blocker = MaxLimit { threshold: 10.0 };

        let chain: Vec<&dyn DynRule> = vec![&blocker, &spy];
        assert!(evaluate_ref(999.0, &chain).is_err());
        assert_eq!(
            spy.executed.get(),
            0,
            "the rule after the block must NOT be executed"
        );

        // Without a block, the spy runs exactly once:
        let chain_ok: Vec<&dyn DynRule> = vec![&spy];
        assert!(evaluate_ref(5.0, &chain_ok).is_ok());
        assert_eq!(spy.executed.get(), 1);
    }

    #[test]
    fn parse_configuration() {
        let chain = parse_chain("max:10000,commissione:1.5,arrotonda,blacklist:666.00").unwrap();
        assert_eq!(chain.len(), 4);
        let names: Vec<&str> = chain.iter().map(|r| r.name_dyn()).collect();
        assert_eq!(names, ["max", "commissione", "arrotonda", "blacklist"]);
        // The parsed chain must also work:
        assert_eq!(evaluate(100.0, &chain).unwrap(), 101.5);
        // The blacklist compares the value that REACHES it, not the initial input
        // (666 on input becomes 675.99 after the fee):
        let blacklist_only = parse_chain("blacklist:666").unwrap();
        assert!(evaluate(666.0, &blacklist_only).is_err());
        assert!(evaluate(667.0, &blacklist_only).is_ok());
    }

    #[test]
    fn parse_configuration_errors() {
        assert_eq!(
            parse_chain("boom").err(),
            Some(ConfigError::UnknownRule("boom".into()))
        );
        assert_eq!(
            parse_chain("max").err(),
            Some(ConfigError::MissingParameter("max".into()))
        );
        assert_eq!(
            parse_chain("commissione:tanto").err(),
            Some(ConfigError::InvalidParameter {
                rule: "commissione".into(),
                value: "tanto".into()
            })
        );
    }
}

/// A variant of `evaluate` over references, used by the spy test.
///
/// TODO: implement this, reusing the same logic as `evaluate` (either by
/// extracting a shared helper generic over the rule-reference type, or by
/// duplicating the `try_fold` here with `&dyn DynRule` items instead of
/// `Box<dyn DynRule>`).
pub fn evaluate_ref(
    amount: f64,
    chain: &[&dyn DynRule],
) -> Result<f64, EvaluationError> {
    let _ = (amount, chain);
    todo!()
}
