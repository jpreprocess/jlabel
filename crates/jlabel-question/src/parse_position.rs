use std::ops::Range;

use crate::position::AllPosition;
use crate::position::BooleanPosition::*;
use crate::position::CategoryPosition::*;
use crate::position::PhonePosition::*;
use crate::position::SignedRangePosition::*;
use crate::position::UndefinedPotision::*;
use crate::position::UnsignedRangePosition::*;
use AllPosition::*;

fn estimate_position(input_pattern: &str) -> Option<(AllPosition, &str)> {
    // Trim asterisks
    let (pattern, asterisks) = {
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
    };

    // Find symbols
    let (prefix, suffix) = {
        // Match to the last char of prefix
        // /A:
        //   ^
        let prefix = pattern
            .bytes()
            .position(|b| "!#%&+-=@^_|:".contains(b as char));

        // Match to the first char of suffix
        // /A:
        // ^
        let suffix = pattern
            .bytes()
            .rev()
            .position(|b| "!#%&+-=@^_|/".contains(b as char))
            .map(|i| pattern.len() - i - 1);

        if let (Some(prefix), Some(suffix)) = (prefix, suffix) {
            if prefix < suffix {
                (Some(prefix), Some(suffix))
            } else if prefix == pattern.len() - 1 {
                (None, Some(suffix))
            } else {
                (Some(prefix), None)
            }
        } else {
            (prefix, suffix)
        }
    };

    let position = 'position: {
        if suffix.is_none() && !asterisks.1 {
            // no suffix and no `*` at the end of pattern
            break 'position UnsignedRange(K3);
        }

        if let Some(prefix) = prefix {
            if let Some(position) = multi_forward(pattern, prefix) {
                // A1 must be captured here; A1 is not distinguishable in combination_match
                break 'position position;
            }
            if let Some(position) = single_forward(pattern.as_bytes()[prefix] as char) {
                break 'position position;
            }
        }

        if let Some(suffix) = suffix {
            if let Some(position) = multi_reverse(pattern, suffix) {
                break 'position position;
            }
            if let Some(position) = single_reverse(pattern.as_bytes()[suffix] as char) {
                break 'position position;
            }
        }

        if let (Some(prefix), Some(suffix)) = (prefix, suffix) {
            if let Some(position) = combination_match(
                pattern.as_bytes()[prefix] as char,
                pattern.as_bytes()[suffix] as char,
            ) {
                break 'position position;
            }
        }

        return None;
    };

    // Check asterisk
    if match position {
        Phone(P1) => (false, true),
        UnsignedRange(K3) => (true, false),
        _ => (true, true),
    } != asterisks
    {
        return None;
    }

    let range = generate_range(position, pattern, prefix, suffix)?;
    if range.is_empty() {
        return None;
    }

    Some((position, &pattern[range]))
}

fn generate_range(
    position: AllPosition,
    pattern: &str,
    prefix: Option<usize>,
    suffix: Option<usize>,
) -> Option<Range<usize>> {
    let prefix = prefix.map(|i| i + 1).unwrap_or(0);
    let suffix = suffix.unwrap_or(pattern.len());

    let (rprefix, rsuffix) = reverse_hint(position);
    if !rprefix.ends_with(&pattern[..prefix]) {
        return None;
    }
    if !rsuffix.starts_with(&pattern[suffix..]) {
        return None;
    }

    Some(prefix..suffix)
}

fn multi_forward(pattern: &str, prefix: usize) -> Option<AllPosition> {
    if prefix < 1 {
        return None;
    }
    match &pattern[prefix - 1..=prefix] {
        "A:" => Some(SignedRange(A1)),
        "B:" => Some(Category(B1)),
        "C:" => Some(Category(C1)),
        "D:" => Some(Category(D1)),
        "E:" => Some(UnsignedRange(E1)),
        "F:" => Some(UnsignedRange(F1)),
        "G:" => Some(UnsignedRange(G1)),
        "H:" => Some(UnsignedRange(H1)),
        "I:" => Some(UnsignedRange(I1)),
        "J:" => Some(UnsignedRange(J1)),
        "K:" => Some(UnsignedRange(K1)),
        _ => None,
    }
}

fn multi_reverse(pattern: &str, suffix: usize) -> Option<AllPosition> {
    if pattern.len() <= suffix + 1 {
        return None;
    }
    match &pattern[suffix..=suffix + 1] {
        "/A" => Some(Phone(P5)),
        "/B" => Some(UnsignedRange(A3)),
        "/C" => Some(Category(B3)),
        "/D" => Some(Category(C3)),
        "/E" => Some(Category(D3)),
        "/F" => Some(Boolean(E5)),
        "/G" => Some(UnsignedRange(F8)),
        "/H" => Some(Boolean(G5)),
        "/I" => Some(UnsignedRange(H2)),
        "/J" => Some(UnsignedRange(I8)),
        "/K" => Some(UnsignedRange(J2)),
        _ => None,
    }
}

fn single_forward(c: char) -> Option<AllPosition> {
    match c {
        '^' => Some(Phone(P2)),
        '=' => Some(Phone(P5)),
        '!' => Some(Boolean(E3)),
        '#' => Some(Boolean(F3)),
        '%' => Some(Boolean(G3)),
        '&' => Some(UnsignedRange(I5)),
        _ => None,
    }
}
fn single_reverse(c: char) -> Option<AllPosition> {
    match c {
        '^' => Some(Phone(P1)),
        '=' => Some(Phone(P4)),
        '!' => Some(UnsignedRange(E2)),
        '#' => Some(UnsignedRange(F2)),
        '%' => Some(UnsignedRange(G2)),
        '&' => Some(UnsignedRange(I4)),
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
        position::{
            AllPosition::*, BooleanPosition::*, CategoryPosition::*, PhonePosition::*,
            SignedRangePosition::*, UnsignedRangePosition::*,
        },
        parse_position::{estimate_position, PositionError},
    };

    #[test]
    fn basic() {
        assert_eq!(estimate_position("a^*"), Some((Phone(P1), "a")));
        assert_eq!(estimate_position("*/A:-1+*"), Some((SignedRange(A1), "-1")));
        assert_eq!(
            estimate_position("*/A:-??+*"),
            Some((SignedRange(A1), "-??"))
        );
        assert_eq!(estimate_position("*|?+*"), Some((UnsignedRange(I7), "?")));
        assert_eq!(estimate_position("*-1"), Some((UnsignedRange(K3), "1")));
        assert_eq!(
            estimate_position("*_42/I:*"),
            Some((UnsignedRange(H2), "42"))
        );
        assert_eq!(estimate_position("*/B:17-*"), Some((Category(B1), "17")));

        assert_eq!(estimate_position("*"), None);
        assert_eq!(estimate_position(":*"), None);
        assert_eq!(estimate_position("*/A:*"), None);
        assert_eq!(estimate_position("*/A:0/B:*"), None);
    }

    #[test]
    fn advanced() {
        assert_eq!(estimate_position("*-1/H:*"), None);
        assert_eq!(estimate_position("*#1*"), Some((Boolean(F3), "1")));
        assert_eq!(estimate_position("*%1*"), Some((Boolean(G3), "1")));
        assert_eq!(estimate_position("*_01/C*"), Some((Category(B3), "01")));
    }
}
