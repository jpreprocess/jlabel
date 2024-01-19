use crate::position::AllPosition;
use crate::position::BooleanPosition::*;
use crate::position::CategoryPosition::*;
use crate::position::PhonePosition::*;
use crate::position::SignedRangePosition::*;
use crate::position::UndefinedPotision::*;
use crate::position::UnsignedRangePosition::*;
use AllPosition::*;

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum PositionError {
    #[error("No matching position found")]
    NoMatchingPosition,
    #[error("The first character should be asterisk in this position")]
    MissingPrefixAsterisk,
    #[error("The last character should be asterisk in this position")]
    MissingSuffixAsterisk,
    #[error("Prefix has unknown sequence")]
    PrefixVerifyError,
    #[error("Suffix has unknown sequence")]
    SuffixVerifyError,
    #[error("Range is empty")]
    EmptyRange,
}

pub fn estimate_position(input_pattern: &str) -> Result<(AllPosition, &str), PositionError> {
    let (pattern, asterisks) = trim_asterisk(input_pattern);
    let (prefix, range, suffix) = find_delim_marks(pattern);
    if range.is_empty() {
        return Err(PositionError::EmptyRange);
    }

    let position = match_position(prefix, suffix, asterisks)?;

    verify(position, prefix, suffix, asterisks)?;

    Ok((position, range))
}

fn find_delim_marks(pattern: &str) -> (&str, &str, &str) {
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

    (
        &pattern[..prefix],
        &pattern[prefix..suffix],
        &pattern[suffix..],
    )
}

fn trim_asterisk(input_pattern: &str) -> (&str, (bool, bool)) {
    let mut pattern = input_pattern;
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

fn match_position(
    prefix: &str,
    suffix: &str,
    asterisks: (bool, bool),
) -> Result<AllPosition, PositionError> {
    if suffix.is_empty() && !asterisks.1 {
        // no suffix and no `*` at the end of pattern
        return Ok(UnsignedRange(K3));
    }

    if let Some(position) = prefix_match(prefix) {
        return Ok(position);
    }

    if let Some(position) = suffix_match(suffix) {
        return Ok(position);
    }

    if !prefix.is_empty() && !suffix.is_empty() {
        if let Some(position) = combination_match(
            prefix.bytes().next_back().unwrap() as char,
            suffix.bytes().next().unwrap() as char,
        ) {
            return Ok(position);
        }
    }

    Err(PositionError::NoMatchingPosition)
}

fn verify(
    position: AllPosition,
    prefix: &str,
    suffix: &str,
    asterisks: (bool, bool),
) -> Result<(), PositionError> {
    // Check asterisk
    if position != Phone(P1) && !asterisks.0 {
        return Err(PositionError::MissingPrefixAsterisk);
    }
    if position != UnsignedRange(K3) && !asterisks.1 {
        return Err(PositionError::MissingSuffixAsterisk);
    }

    // Check prefix and suffix
    let (rprefix, rsuffix) = reverse_hint(position);
    if !rprefix.ends_with(prefix) {
        return Err(PositionError::PrefixVerifyError);
    }
    if !rsuffix.starts_with(suffix) {
        return Err(PositionError::SuffixVerifyError);
    }

    Ok(())
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
fn combination_match(prefix: char, suffix: char) -> Option<AllPosition> {
    match (prefix, suffix) {
        ('^', '-') => Some(Phone(P2)),
        ('-', '+') => Some(Phone(P3)),
        ('+', '=') => Some(Phone(P4)),
        ('=', '/') => Some(Phone(P5)),

        // (':', '+') => Some(SignedRange(A1)),
        ('+', '+') => Some(UnsignedRange(A2)),
        // ('+', '/') => Some(UnsignedRange(A3)),

        // (':', '-') => Some(Category(B1)),
        ('-', '_') => Some(Category(B2)),
        // ('_', '/') => Some(Category(B3)),

        // (':', '_') => Some(Category(C1)),
        ('_', '+') => Some(Category(C2)),
        // ('+', '/') => Some(Category(C3)),

        // (':', '+') => Some(Category(D1)),
        ('+', '_') => Some(Category(D2)),
        // ('_', '/') => Some(Category(D3)),

        // (':', '_') => Some(UnsignedRange(E1)),
        ('_', '!') => Some(UnsignedRange(E2)),
        ('!', '_') => Some(Boolean(E3)),
        ('_', '-') => Some(Undefined(E4)),
        ('-', '/') => Some(Boolean(E5)),

        // (':', '_') => Some(UnsignedRange(F1)),
        ('_', '#') => Some(UnsignedRange(F2)),
        ('#', '_') => Some(Boolean(F3)),
        ('_', '@') => Some(Undefined(F4)),
        ('@', '_') => Some(UnsignedRange(F5)),
        ('_', '|') => Some(UnsignedRange(F6)),
        ('|', '_') => Some(UnsignedRange(F7)),
        // ('_', '/') => Some(UnsignedRange(F8)),

        // (':', '_') => Some(UnsignedRange(G1)),
        ('_', '%') => Some(UnsignedRange(G2)),
        ('%', '_') => Some(Boolean(G3)),
        ('_', '_') => Some(Undefined(G4)),
        // ('_', '/') => Some(Boolean(G5)),

        // (':', '_') => Some(UnsignedRange(H1)),
        // ('_', '/') => Some(UnsignedRange(H2)),

        // (':', '-') => Some(UnsignedRange(I1)),
        ('-', '@') => Some(UnsignedRange(I2)),
        ('@', '+') => Some(UnsignedRange(I3)),
        ('+', '&') => Some(UnsignedRange(I4)),
        ('&', '-') => Some(UnsignedRange(I5)),
        ('-', '|') => Some(UnsignedRange(I6)),
        ('|', '+') => Some(UnsignedRange(I7)),
        // ('+', '/') => Some(UnsignedRange(I8)),

        // (':', '_') => Some(UnsignedRange(J1)),
        // ('_', '/') => Some(UnsignedRange(J2)),

        // (':', '+') => Some(UnsignedRange(K1)),
        ('+', '-') => Some(UnsignedRange(K2)),

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
        parse_position::{estimate_position, PositionError},
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
            Err(PositionError::EmptyRange)
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