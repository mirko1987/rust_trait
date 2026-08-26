//! # Problem 1 — Notification system
//! **Concepts:** traits, default methods, overriding.
//!
//! An application sends notifications over different channels: email, SMS,
//! console. Each channel has a recipient and its own way of formatting, but
//! the base behavior is shared.
//!
//! Requirements (encoded in the tests):
//! 1. The `Channel` trait exposes the recipient and the formatted message.
//! 2. Standard format `"[{recipient}] {text}"`, defined ONCE
//!    (in the trait's default method, not repeated in the impls).
//! 3. SMS: if the formatted message exceeds 160 characters, it must be
//!    truncated to 157 and end with "..." (final length: exactly 160).
//! 4. Email: prepends `"Oggetto: notifica\n"` to the standard format.
//! 5. Console: uses the standard behavior unmodified.
//!
//! MASTERY QUESTIONS (answer below in a comment):
//! - What happens if a type implements the trait without defining the
//!   required method? And without defining the default one?
//! - Why can the default method call `self.recipient()` even though that
//!   method has no body in the trait?

pub trait Channel {
    /// Recipient identifier (e.g. email address, phone number, "console").
    fn recipient(&self) -> String;

    /// Standard format: "[{recipient}] {text}".
    /// This is the right place to define it once.
    ///
    /// Note: this method is never overridden by the impls below, so `Email`
    /// and `Sms` can safely call it from their own `format` override.
    /// Calling it through `Channel::format(self, ...)` instead would NOT
    /// work: once a type overrides `format`, there is no syntax in Rust to
    /// reach back into the trait's default body for that method — the call
    /// always resolves to the override, causing infinite recursion.
    fn standard_format(&self, text: &str) -> String {
        format!("[{}] {}", self.recipient(), text)
    }

    fn format(&self, text: &str) -> String {
        self.standard_format(text)
    }
}

pub struct Email {
    pub address: String,
}

pub struct Sms {
    pub number: String,
}

pub struct Console;

impl Channel for Email {
    fn recipient(&self) -> String {
        self.address.clone()
    }

    fn format(&self, text: &str) -> String {
        format!("Oggetto: notifica\n{}", self.standard_format(text))
    }
}

impl Channel for Sms {
    fn recipient(&self) -> String {
        self.number.clone()
    }

    fn format(&self, text: &str) -> String {
        let base = self.standard_format(text);
        if base.len() > 160 {
            format!("{}...", &base[..157])
        } else {
            base
        }
    }
}

impl Channel for Console {
    fn recipient(&self) -> String {
        "console".to_string()
    }
    // No override of `format`: requirement 5.
}

/// Takes a text and a list of channels of THE SAME TYPE and returns all
/// formatted messages, in the same order.
pub fn send_to_all<C: Channel>(text: &str, channels: &[C]) -> Vec<String> {
    channels.iter().map(|c| c.format(text)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn console_uses_the_standard_format() {
        let c = Console;
        assert_eq!(c.recipient(), "console");
        assert_eq!(c.format("build fallita"), "[console] build fallita");
    }

    #[test]
    fn email_prepends_subject() {
        let e = Email { address: "ops@example.com".into() };
        assert_eq!(
            e.format("deploy ok"),
            "Oggetto: notifica\n[ops@example.com] deploy ok"
        );
    }

    #[test]
    fn short_sms_is_not_touched() {
        let s = Sms { number: "+34600000000".into() };
        assert_eq!(s.format("ciao"), "[+34600000000] ciao");
    }

    #[test]
    fn long_sms_is_truncated_to_160() {
        let s = Sms { number: "+34600000000".into() };
        let long_text = "x".repeat(300);
        let msg = s.format(&long_text);
        assert_eq!(msg.len(), 160);
        assert!(msg.ends_with("..."));
        assert!(msg.starts_with("[+34600000000] "));
    }

    #[test]
    fn sms_at_exact_limit_is_not_truncated() {
        let s = Sms { number: "+34600000000".into() };
        // "[+34600000000] " takes up 15 characters: 145 chars of text => exactly 160
        let text = "y".repeat(145);
        let msg = s.format(&text);
        assert_eq!(msg.len(), 160);
        assert!(!msg.ends_with("..."));
    }

    #[test]
    fn send_to_all_preserves_order() {
        let channels = vec![
            Sms { number: "111".into() },
            Sms { number: "222".into() },
        ];
        let out = send_to_all("test", &channels);
        assert_eq!(out, vec!["[111] test", "[222] test"]);
    }
}
