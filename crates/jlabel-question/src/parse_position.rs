//! Estimate the position from pattern

use crate::position::AllPosition;
use crate::position::BooleanPosition::*;
use crate::position::CategoryPosition::*;
use crate::position::PhonePosition::*;
use crate::position::SignedRangePosition::*;
use crate::position::UndefinedPotision::*;
use crate::position::UnsignedRangePosition::*;
use AllPosition::*;

/// Errors from position parser.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum PositionError {
    /// Could not determine the position uniquely.
    #[error("No matching position found")]
    NoMatchingPosition,
    /// The position is not `P1`, so it requires an asterisk as the first character of the pattern.
    #[error("The first character should be asterisk in this position")]
    MissingPrefixAsterisk,
    /// The position is not `K3`, so it requires an asterisk as the last character of the pattern.
    #[error("The last character should be asterisk in this position")]
    MissingSuffixAsterisk,
    /// The prefix (string before the range section) conflicts with the estimated position.
    #[error("Prefix has unknown sequence")]
    PrefixVerifyError,
    /// The suffix (string after the range section) conflicts with the estimated position.
    #[error("Suffix has unknown sequence")]
    SuffixVerifyError,
    /// Range section is empty. This pattern does not match any label.
    #[error("Range is empty")]
    EmptyRange,
}

/// Estimates the position the pattern is pointing at.
pub(crate) fn estimate_position(pattern: &str) -> Result<(AllPosition, &str), PositionError> {
    let split = PositionSplit::new(pattern);
    let position = split.match_position()?;
    split.verify(position)?;

    Ok((position, split.into_range()?))
}

struct PositionSplit<'a> {
    prefix: &'a str,
    range: &'a str,
    suffix: &'a str,
    asterisks: (bool, bool),
}

impl<'a> PositionSplit<'a> {
    pub fn new(pattern: &'a str) -> Self {
        let (pattern, asterisks) = Self::trim_asterisk(pattern);

        // Match to the next char of prefix
        // /A:
        //    ^
        let mut prefix = pattern
            .bytes()
            .position(|b| "!#%&+-=@^_|:".contains(b as char))
            .map(|i| i + 1)
            .unwrap_or(0);

        // Match to the first char of suffix
        // /A:
        // ^
        let mut suffix = pattern
            .bytes()
            .rev()
            .position(|b| "!#%&+-=@^_|/".contains(b as char))
            .map(|i| pattern.len() - i - 1)
            .unwrap_or(pattern.len());

        // If there is only one prefix/suffix delimiter:
        // /A:
        // ^s ^p
        if prefix > suffix {
            if prefix == pattern.len() {
                prefix = 0;
            } else {
                suffix = pattern.len();
            }
        }

        Self {
            prefix: &pattern[..prefix],
            range: &pattern[prefix..suffix],
            suffix: &pattern[suffix..],
            asterisks,
        }
    }

    fn trim_asterisk(mut pattern: &str) -> (&str, (bool, bool)) {
        let mut stars = (false, false);
        if pattern.starts_with('*') {
            pattern = &pattern[1..];
            stars.0 = true;
        }
        if pattern.ends_with('*') {
            pattern = &pattern[..pattern.len() - 1];
            stars.1 = true;
        }
        (pattern, stars)
    }

    pub fn match_position(&self) -> Result<AllPosition, PositionError> {
        if self.suffix.is_empty() && !self.asterisks.1 {
            // no suffix and no `*` at the end of pattern
            return Ok(UnsignedRange(K3));
        }

        if let Some(position) = prefix_match(self.prefix) {
            return Ok(position);
        }

        if let Some(position) = suffix_match(self.suffix) {
            return Ok(position);
        }

        if let (Some(pchar), Some(schar)) =
            (self.prefix.bytes().next_back(), self.suffix.bytes().next())
        {
            if let Some(position) = combination_match(pchar, schar) {
                return Ok(position);
            }
        }

        Err(PositionError::NoMatchingPosition)
    }

    pub fn verify(&self, position: AllPosition) -> Result<(), PositionError> {
        // Check asterisk
        if position != Phone(P1) && !self.asterisks.0 {
            return Err(PositionError::MissingPrefixAsterisk);
        }
        if position != UnsignedRange(K3) && !self.asterisks.1 {
            return Err(PositionError::MissingSuffixAsterisk);
        }

        // Check prefix and suffix
        let (rprefix, rsuffix) = reverse_hint(position);
        if !rprefix.ends_with(self.prefix) {
            return Err(PositionError::PrefixVerifyError);
        }
        if !rsuffix.starts_with(self.suffix) {
            return Err(PositionError::SuffixVerifyError);
        }

        Ok(())
    }

    pub fn into_range(self) -> Result<&'a str, PositionError> {
        if self.range.is_empty() {
            return Err(PositionError::EmptyRange);
        }
        Ok(self.range)
    }
}

fn prefix_match(prefix: &str) -> Option<AllPosition> {
    let mut bytes = prefix.bytes();
    match bytes.next_back()? {
        b'^' => Some(Phone(P2)),
        b'=' => Some(Phone(P5)),
        b'!' => Some(Boolean(E3)),
        b'#' => Some(Boolean(F3)),
        b'%' => Some(Boolean(G3)),
        b'&' => Some(UnsignedRange(I5)),
        b':' => match bytes.next_back()? {
            b'A' => Some(SignedRange(A1)),
            b'B' => Some(Category(B1)),
            b'C' => Some(Category(C1)),
            b'D' => Some(Category(D1)),
            b'E' => Some(UnsignedRange(E1)),
            b'F' => Some(UnsignedRange(F1)),
            b'G' => Some(UnsignedRange(G1)),
            b'H' => Some(UnsignedRange(H1)),
            b'I' => Some(UnsignedRange(I1)),
            b'J' => Some(UnsignedRange(J1)),
            b'K' => Some(UnsignedRange(K1)),
            _ => None,
        },
        _ => None,
    }
}
fn suffix_match(suffix: &str) -> Option<AllPosition> {
    let mut bytes = suffix.bytes();
    match bytes.next()? {
        b'^' => Some(Phone(P1)),
        b'=' => Some(Phone(P4)),
        b'!' => Some(UnsignedRange(E2)),
        b'#' => Some(UnsignedRange(F2)),
        b'%' => Some(UnsignedRange(G2)),
        b'&' => Some(UnsignedRange(I4)),
        b'/' => match bytes.next()? {
            b'A' => Some(Phone(P5)),
            b'B' => Some(UnsignedRange(A3)),
            b'C' => Some(Category(B3)),
            b'D' => Some(Category(C3)),
            b'E' => Some(Category(D3)),
            b'F' => Some(Boolean(E5)),
            b'G' => Some(UnsignedRange(F8)),
            b'H' => Some(Boolean(G5)),
            b'I' => Some(UnsignedRange(H2)),
            b'J' => Some(UnsignedRange(I8)),
            b'K' => Some(UnsignedRange(J2)),
            _ => None,
        },
        _ => None,
    }
}
fn combination_match(prefix: u8, suffix: u8) -> Option<AllPosition> {
    // The following conditions were removed:
    // - Conditions that are matched by prefix_match or suffix_match
    // - Conditions that cannot uniquely determine the position
    match (prefix, suffix) {
        (b'-', b'+') => Some(Phone(P3)),

        (b'+', b'+') => Some(UnsignedRange(A2)),

        (b'-', b'_') => Some(Category(B2)),

        (b'_', b'+') => Some(Category(C2)),

        (b'+', b'_') => Some(Category(D2)),

        (b'_', b'-') => Some(Undefined(E4)),
        (b'-', b'/') => Some(Boolean(E5)),

        (b'_', b'@') => Some(Undefined(F4)),
        (b'@', b'_') => Some(UnsignedRange(F5)),
        (b'_', b'|') => Some(UnsignedRange(F6)),
        (b'|', b'_') => Some(UnsignedRange(F7)),

        (b'_', b'_') => Some(Undefined(G4)),

        (b'-', b'@') => Some(UnsignedRange(I2)),
        (b'@', b'+') => Some(UnsignedRange(I3)),
        (b'-', b'|') => Some(UnsignedRange(I6)),
        (b'|', b'+') => Some(UnsignedRange(I7)),

        (b'+', b'-') => Some(UnsignedRange(K2)),

        _ => None,
    }
}

fn reverse_hint(position: AllPosition) -> (&'static str, &'static str) {
    match position {
        Phone(P1) => ("", "^"),
        Phone(P2) => ("^", "-"),
        Phone(P3) => ("-", "+"),
        Phone(P4) => ("+", "="),
        Phone(P5) => ("=", "/A:"),

        SignedRange(A1) => ("/A:", "+"),
        UnsignedRange(A2) => ("+", "+"),
        UnsignedRange(A3) => ("+", "/B:"),

        Category(B1) => ("/B:", "-"),
        Category(B2) => ("-", "_"),
        Category(B3) => ("_", "/C:"),

        Category(C1) => ("/C:", "_"),
        Category(C2) => ("_", "+"),
        Category(C3) => ("+", "/D:"),

        Category(D1) => ("/D:", "+"),
        Category(D2) => ("+", "_"),
        Category(D3) => ("_", "/E:"),

        UnsignedRange(E1) => ("/E:", "_"),
        UnsignedRange(E2) => ("_", "!"),
        Boolean(E3) => ("!", "_"),
        Undefined(E4) => ("_", "-"),
        Boolean(E5) => ("-", "/F:"),

        UnsignedRange(F1) => ("/F:", "_"),
        UnsignedRange(F2) => ("_", "#"),
        Boolean(F3) => ("#", "_"),
        Undefined(F4) => ("_", "@"),
        UnsignedRange(F5) => ("@", "_"),
        UnsignedRange(F6) => ("_", "|"),
        UnsignedRange(F7) => ("|", "_"),
        UnsignedRange(F8) => ("_", "/G:"),

        UnsignedRange(G1) => ("/G:", "_"),
        UnsignedRange(G2) => ("_", "%"),
        Boolean(G3) => ("%", "_"),
        Undefined(G4) => ("_", "_"),
        Boolean(G5) => ("_", "/H:"),

        UnsignedRange(H1) => ("/H:", "_"),
        UnsignedRange(H2) => ("_", "/I:"),

        UnsignedRange(I1) => ("/I:", "-"),
        UnsignedRange(I2) => ("-", "@"),
        UnsignedRange(I3) => ("@", "+"),
        UnsignedRange(I4) => ("+", "&"),
        UnsignedRange(I5) => ("&", "-"),
        UnsignedRange(I6) => ("-", "|"),
        UnsignedRange(I7) => ("|", "+"),
        UnsignedRange(I8) => ("+", "/J:"),

        UnsignedRange(J1) => ("/J:", "_"),
        UnsignedRange(J2) => ("_", "/K:"),

        UnsignedRange(K1) => ("/K:", "+"),
        UnsignedRange(K2) => ("+", "-"),
        UnsignedRange(K3) => ("-", ""),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_position::{PositionError, estimate_position},
        position::{
            AllPosition::*, BooleanPosition::*, CategoryPosition::*, PhonePosition::*,
            SignedRangePosition::*, UndefinedPotision::*, UnsignedRangePosition::*,
        },
    };

    #[test]
    fn basic() {
        assert_eq!(estimate_position("a^*"), Ok((Phone(P1), "a")));
        assert_eq!(estimate_position("*/A:-1+*"), Ok((SignedRange(A1), "-1")));
        assert_eq!(estimate_position("*/A:-??+*"), Ok((SignedRange(A1), "-??")));
        assert_eq!(estimate_position("*|?+*"), Ok((UnsignedRange(I7), "?")));
        assert_eq!(estimate_position("*-1"), Ok((UnsignedRange(K3), "1")));
        assert_eq!(estimate_position("*_42/I:*"), Ok((UnsignedRange(H2), "42")));
        assert_eq!(estimate_position("*/B:17-*"), Ok((Category(B1), "17")));
        assert_eq!(estimate_position("*_xx-*"), Ok((Undefined(E4), "xx")));
        assert_eq!(estimate_position("*_xx@*"), Ok((Undefined(F4), "xx")));
        assert_eq!(estimate_position("*_xx_*"), Ok((Undefined(G4), "xx")));
    }

    #[test]
    fn basic_fail() {
        assert_eq!(estimate_position("*"), Err(PositionError::EmptyRange));
        assert_eq!(
            estimate_position(":*"),
            Err(PositionError::NoMatchingPosition)
        );
        assert_eq!(estimate_position("*/A:*"), Err(PositionError::EmptyRange));
        assert_eq!(
            estimate_position("*/A:0/B:*"),
            Err(PositionError::SuffixVerifyError)
        );
        assert_eq!(
            estimate_position("*/B:0+*"),
            Err(PositionError::SuffixVerifyError)
        );

        assert_eq!(
            estimate_position("*/B :0+*"),
            Err(PositionError::NoMatchingPosition)
        );
        assert_eq!(
            estimate_position("*_0/Z:*"),
            Err(PositionError::NoMatchingPosition)
        );

        assert_eq!(
            estimate_position("a^"),
            Err(PositionError::MissingSuffixAsterisk)
        );
        assert_eq!(
            estimate_position("/B:17-*"),
            Err(PositionError::MissingPrefixAsterisk)
        );
        assert_eq!(
            // K3
            estimate_position("-1"),
            Err(PositionError::MissingPrefixAsterisk)
        );
    }

    #[test]
    fn advanced() {
        assert_eq!(estimate_position("*#1*"), Ok((Boolean(F3), "1")));
        assert_eq!(estimate_position("*%1*"), Ok((Boolean(G3), "1")));
        assert_eq!(estimate_position("*_01/C*"), Ok((Category(B3), "01")));
        assert_eq!(estimate_position("*-1/*"), Ok((Boolean(E5), "1")));

        assert_eq!(
            estimate_position("*-1/H:*"),
            Err(PositionError::PrefixVerifyError)
        );
    }
}
