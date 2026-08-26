//! # Problem 4 — Shipping rate engine
//! **Concepts:** traits + generics, `impl Trait`, static dispatch.
//!
//! Calculation rules:
//! - Standard: weight × rate per kg
//! - Express: flat fee + weight × surcharged rate
//! - Store pickup: always €0, but NOT eligible beyond 20 kg
//!
//! Requirements (encoded in the tests):
//! 1. `Rate::cost` can fail: the signature reflects that.
//! 2. `quote` accepts "something that implements Rate" (static dispatch)
//!    and formats `"Costo: X.XX€"` with two decimals.
//! 3. `best_rate` picks the cheapest among the ELIGIBLE ones in a
//!    homogeneous slice; if none is eligible → `NoneEligible`.
//! 4. In a comment: why can't you mix Standard and Express in the same
//!    call? What are the two solutions? (one is in Problem 7)
//!
//! MASTERY QUESTIONS:
//! - What does the compiler generate when you call `quote` with two
//!   different types? What is this mechanism called, and what is its
//!   cost/benefit?
//! - `fn f(t: &impl Rate)` vs `fn f<T: Rate>(t: &T)`: when are they NOT
//!   interchangeable?

#[derive(Debug, PartialEq)]
pub enum RateError {
    NotEligible { weight: f64 },
    NoneEligible,
}

pub trait Rate {
    fn cost(&self, weight_kg: f64) -> Result<f64, RateError>;
}

pub struct Standard {
    pub rate_per_kg: f64,
}

pub struct Express {
    pub flat_fee: f64,
    pub rate_per_kg: f64,
}

pub struct StorePickup;

impl Rate for Standard {
    /// Standard rate: weight times `rate_per_kg`. This never actually
    /// fails, but the signature still returns a `Result` because the
    /// trait requires it.
    ///
    /// TODO: implement this — return `Ok(weight_kg * self.rate_per_kg)`.
    fn cost(&self, weight_kg: f64) -> Result<f64, RateError> {
        Ok(weight_kg * self.rate_per_kg)
    }
}

impl Rate for Express {
    /// Express rate: a flat fee plus weight times `rate_per_kg`.
    ///
    /// TODO: implement this — return
    /// `Ok(self.flat_fee + weight_kg * self.rate_per_kg)`.
    fn cost(&self, weight_kg: f64) -> Result<f64, RateError> {
       Ok(self.flat_free + weight_kg *self.rate_per_kg)
    }
}

impl Rate for StorePickup {
    /// Store pickup: free (`0.0`) for any weight up to and including
    /// 20 kg; beyond that threshold it is not eligible.
    ///
    /// TODO: implement this. Boundary case: exactly `20.0` must still
    /// return `Ok(0.0)` (see test `basic_costs`), while anything above
    /// must return `Err(RateError::NotEligible { weight: weight_kg })`
    /// (see test `pickup_beyond_threshold`, using `20.5`). A single
    /// `if weight_kg <= 20.0 { Ok(0.0) } else { Err(...) }` is enough.
    fn cost(&self, weight_kg: f64) -> Result<f64, RateError> {
        if weight_kg<=20.0{
            Ok(0.0)
        }
        else{
            Err(RateError::NotEligible{weight:weight_kg})
        }
    }
}

/// Static dispatch: the signature must accept any type implementing `Rate`.
///
/// TODO: implement this. Call `rate.cost(weight_kg)`, propagate the error
/// with `?`, then format the `Ok` value with exactly two decimals as
/// `"Costo: X.XX€"` (e.g. `format!("Costo: {:.2}€", value)`).
pub fn quote(weight_kg: f64, rate: &impl Rate) -> Result<String, RateError> {
    let value = rate.cost(weight_kg)?
    Ok(("Costo: {:.2}€", value))
}

/// The cheapest among the eligible rates, over a homogeneous slice.
///
/// TODO: implement this. Iterate over `rates`, call `.cost(weight_kg)` on
/// each, keep only the `Ok` results (ignore the `Err`s — an ineligible
/// rate simply doesn't compete, it doesn't fail the whole computation),
/// and return the smallest one. If the slice is empty or every rate is
/// ineligible, return `Err(RateError::NoneEligible)`. A `filter_map` +
/// `fold`/`min_by` over the `Ok` values works well here.
pub fn best_rate<T: Rate>(weight_kg: f64, rates: &[T]) -> Result<f64, RateError> {
    rates.iter()
    .filter_map(|r| r.cost(weight_kg).ok())
    .fold(None,|acc,cost| match acc{
        None=>Some(cost),
        Some(m) if cost< m=>Some(cost),
        Some(m)=>Some(m),
    })
    .ok_or(RateError::NoneEligible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_costs() {
        assert_eq!(Standard { rate_per_kg: 2.0 }.cost(3.0), Ok(6.0));
        assert_eq!(
            Express { flat_fee: 5.0, rate_per_kg: 3.0 }.cost(2.0),
            Ok(11.0)
        );
        assert_eq!(StorePickup.cost(10.0), Ok(0.0));
        assert_eq!(StorePickup.cost(20.0), Ok(0.0));
    }

    #[test]
    fn pickup_beyond_threshold() {
        assert_eq!(
            StorePickup.cost(20.5),
            Err(RateError::NotEligible { weight: 20.5 })
        );
    }

    #[test]
    fn quote_formats() {
        let t = Standard { rate_per_kg: 2.5 };
        assert_eq!(quote(3.0, &t), Ok("Costo: 7.50€".to_string()));
        // static dispatch: same function, different concrete type
        let e = Express { flat_fee: 4.0, rate_per_kg: 1.0 };
        assert_eq!(quote(1.0, &e), Ok("Costo: 5.00€".to_string()));
    }

    #[test]
    fn quote_propagates_error() {
        assert_eq!(
            quote(25.0, &StorePickup),
            Err(RateError::NotEligible { weight: 25.0 })
        );
    }

    #[test]
    fn best_among_homogeneous() {
        let rates = [
            Standard { rate_per_kg: 3.0 },
            Standard { rate_per_kg: 1.5 },
            Standard { rate_per_kg: 2.0 },
        ];
        assert_eq!(best_rate(2.0, &rates), Ok(3.0)); // 2.0 * 1.5
    }

    #[test]
    fn none_eligible() {
        let rates: [StorePickup; 2] = [StorePickup, StorePickup];
        assert_eq!(
            best_rate(30.0, &rates),
            Err(RateError::NoneEligible)
        );
        let empty: [Standard; 0] = [];
        assert_eq!(
            best_rate(1.0, &empty),
            Err(RateError::NoneEligible)
        );
    }

    #[test]
    fn ignores_ineligible_ones() {
        // At 30 kg, pickup is not eligible, but that shouldn't make
        // everything fail: here the slice is homogeneous StorePickup, so
        // it does fail; you'll tackle the mixed case in Problem 7 — for
        // now just verify the filter:
        let rates = [StorePickup];
        assert_eq!(best_rate(10.0, &rates), Ok(0.0));
    }
}
