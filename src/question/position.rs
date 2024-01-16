use std::{fmt::Debug, ops::Range};

use super::ParseError;

pub fn position(prefix: &str, suffix: &str) -> Option<AllPosition> {
    use AllPosition::*;
    use BooleanPosition::*;
    use CategoryPosition::*;
    use PhonePosition::*;
    use SignedRangePosition::*;
    use UndefinedPotision::*;
    use UnsignedRangePosition::*;

    match (prefix, suffix) {
        ("", "^*") => Some(Phone(P1)),
        ("*^", "-*") => Some(Phone(P2)),
        ("*-", "+*") => Some(Phone(P3)),
        ("*+", "=*") => Some(Phone(P4)),
        ("*=", "/A:*") => Some(Phone(P5)),

        ("*/A:", "+*") => Some(SignedRange(A1)),
        ("*+", "+*") => Some(UnsignedRange(A2)),
        ("*+", "/B:*") => Some(UnsignedRange(A3)),

        ("*/B:", "-*") => Some(Category(B1)),
        ("*-", "_*") => Some(Category(B2)),
        ("*_", "/C:*") => Some(Category(B3)),

        ("*/C:", "_*") => Some(Category(C1)),
        ("*_", "+*") => Some(Category(C2)),
        ("*+", "/D:*") => Some(Category(C3)),

        ("*/D:", "+*") => Some(Category(D1)),
        ("*+", "_*") => Some(Category(D2)),
        ("*_", "/E:*") => Some(Category(D3)),

        ("*/E:", "_*") => Some(UnsignedRange(E1)),
        ("*_", "!*") => Some(UnsignedRange(E2)),
        ("*!", "_*") => Some(Boolean(E3)),
        ("*_", "-*") => Some(Undefined(E4)),
        ("*-", "/F:*") => Some(Boolean(E5)),

        ("*/F:", "_*") => Some(UnsignedRange(F1)),
        ("*_", "#*") => Some(UnsignedRange(F2)),
        ("*#", "_*") => Some(Boolean(F3)),
        ("*_", "@*") => Some(Undefined(F4)),
        ("*@", "_*") => Some(UnsignedRange(F5)),
        ("*_", "|*") => Some(UnsignedRange(F6)),
        ("*|", "_*") => Some(UnsignedRange(F7)),
        ("*_", "/G:*") => Some(UnsignedRange(F8)),

        ("*/G:", "_*") => Some(UnsignedRange(G1)),
        ("*_", "%*") => Some(UnsignedRange(G2)),
        ("*%", "_*") => Some(Boolean(G3)),
        ("*_", "_*") => Some(Undefined(G4)),
        ("*_", "/H:*") => Some(Boolean(G5)),

        ("*/H:", "_*") => Some(UnsignedRange(H1)),
        ("*_", "/I:*") => Some(UnsignedRange(H2)),

        ("*/I:", "-*") => Some(UnsignedRange(I1)),
        ("*-", "@*") => Some(UnsignedRange(I2)),
        ("*@", "+*") => Some(UnsignedRange(I3)),
        ("*+", "&*") => Some(UnsignedRange(I4)),
        ("*&", "-*") => Some(UnsignedRange(I5)),
        ("*-", "|*") => Some(UnsignedRange(I6)),
        ("*|", "+*") => Some(UnsignedRange(I7)),
        ("*+", "/J:*") => Some(UnsignedRange(I8)),

        ("*/J:", "_*") => Some(UnsignedRange(J1)),
        ("*_", "/K:*") => Some(UnsignedRange(J2)),

        ("*/K:", "+*") => Some(UnsignedRange(K1)),
        ("*+", "-*") => Some(UnsignedRange(K2)),
        ("*-", "") => Some(UnsignedRange(K3)),

        _ => None,
    }
}

pub enum AllPosition {
    Phone(PhonePosition),
    SignedRange(SignedRangePosition),
    UnsignedRange(UnsignedRangePosition),
    Boolean(BooleanPosition),
    Category(CategoryPosition),
    Undefined(UndefinedPotision),
}

pub trait Position {
    type Target;
    type Range;

    fn range(&self, ranges: &[&String]) -> Result<Self::Range, ParseError>;
    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhonePosition {
    P1,
    P2,
    P3,
    P4,
    P5,
}

impl Position for PhonePosition {
    type Target = String;
    type Range = Vec<String>;

    fn range(&self, ranges: &[&String]) -> Result<Self::Range, ParseError> {
        Ok(ranges.iter().map(|s| s.to_string()).collect())
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignedRangePosition {
    A1,
}

impl Position for SignedRangePosition {
    type Target = i8;
    type Range = Range<i8>;

    fn range(&self, ranges: &[&String]) -> Result<Self::Range, ParseError> {
        let first = ranges.first().ok_or(ParseError::Empty)?;
        let mut range = range_i8(first)?;
        for r in ranges[1..].iter() {
            let r = range_i8(r)?;
            extend_range(&mut range, r)?;
        }
        Ok(range)
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

fn range_i8(s: &str) -> Result<Range<i8>, ParseError> {
    let range = match s {
        "-??" => -99..-9,
        "-?" => -9..0,
        "?" => 0..10,
        s if s.ends_with('?') => {
            let d = s[..s.len() - 1]
                .parse::<i8>()
                .map_err(ParseError::FailWildcard)?;
            debug_assert!(d >= 0);
            d * 10..(d + 1) * 10
        }
        s => {
            let d = s.parse::<i8>().map_err(ParseError::FailLiteral)?;
            d..d + 1
        }
    };
    Ok(range)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsignedRangePosition {
    A2,
    A3,

    E1,
    E2,

    F1,
    F2,
    F5,
    F6,
    F7,
    F8,

    G1,
    G2,

    H1,
    H2,

    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,

    J1,
    J2,

    K1,
    K2,
    K3,
}

impl Position for UnsignedRangePosition {
    type Target = u8;
    type Range = Range<u8>;

    fn range(&self, ranges: &[&String]) -> Result<Self::Range, ParseError> {
        let first = ranges.first().ok_or(ParseError::Empty)?;
        let mut range = range_u8(first)?;
        for r in ranges[1..].iter() {
            let r = range_u8(r)?;
            extend_range(&mut range, r)?;
        }
        Ok(range)
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

fn range_u8(s: &str) -> Result<Range<u8>, ParseError> {
    let range = match s {
        "?" => 0..10,
        s if s.ends_with('?') => {
            let d = s[..s.len() - 1]
                .parse::<u8>()
                .map_err(ParseError::FailWildcard)?;
            d * 10..(d + 1) * 10
        }
        s => {
            let d = s.parse::<u8>().map_err(ParseError::FailLiteral)?;
            d..d + 1
        }
    };
    Ok(range)
}

fn extend_range<Idx>(
    target: &mut Range<Idx>,
    Range { start, end }: Range<Idx>,
) -> Result<(), ParseError>
where
    Idx: Eq,
{
    let ok = target.end == start;
    if ok {
        target.end = end;
        Ok(())
    } else {
        Err(ParseError::IncontinuousRange)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanPosition {
    E3,
    E5,

    F3,

    G3,
    G5,
}

impl Position for BooleanPosition {
    type Target = bool;
    type Range = bool;

    fn range(&self, ranges: &[&String]) -> Result<Self::Range, ParseError> {
        let first = ranges.first().ok_or(ParseError::Empty)?;
        match first.as_str() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(ParseError::InvalidBoolean(first.to_string())),
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range == target
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CategoryPosition {
    B1,
    B2,
    B3,
    C1,
    C2,
    C3,
    D1,
    D2,
    D3,
}

impl Position for CategoryPosition {
    type Target = u8;
    type Range = Vec<u8>;

    fn range(&self, ranges: &[&String]) -> Result<Self::Range, ParseError> {
        ranges
            .iter()
            .map(|s| s.parse::<u8>().map_err(ParseError::FailLiteral))
            .collect()
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UndefinedPotision {
    E4,
    F4,
    G4,
}

impl Position for UndefinedPotision {
    type Target = ();
    type Range = ();

    fn range(&self, _: &[&String]) -> Result<Self::Range, ParseError> {
        Ok(())
    }

    fn test(&self, _: &Self::Range, _: &Self::Target) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::question::{
        position::{extend_range, range_u8},
        ParseError,
    };

    use super::range_i8;

    #[test]
    fn parse_i8_range() {
        assert_eq!(range_i8("12"), Ok(12..13));
        assert_eq!(range_i8("1?"), Ok(10..20));
        assert_eq!(range_i8("?"), Ok(0..10));

        assert_eq!(range_i8("-12"), Ok(-12..-11));
        assert_eq!(range_i8("-?"), Ok(-9..0));
        assert_eq!(range_i8("-??"), Ok(-99..-9));

        // assert_eq!(range_i8("-1?"), Ok(-19..-9));
    }

    #[test]
    fn parse_u8_range() {
        assert_eq!(range_u8("12"), Ok(12..13));
        assert_eq!(range_u8("1?"), Ok(10..20));
        assert_eq!(range_u8("12?"), Ok(120..130));
        assert_eq!(range_u8("?"), Ok(0..10));
    }

    #[test]
    fn range_fail() {
        use std::num::IntErrorKind;
        assert!(matches!(
            range_u8("?2"),
            Err(ParseError::FailLiteral(e)) if *e.kind() == IntErrorKind::InvalidDigit
        ));
        assert!(matches!(
            range_i8("?2"),
            Err(ParseError::FailLiteral(e)) if *e.kind() == IntErrorKind::InvalidDigit
        ));

        assert!(matches!(
            range_u8("???"),
            Err(ParseError::FailWildcard(e)) if *e.kind() == IntErrorKind::InvalidDigit
        ));
        assert!(matches!(
            range_i8("???"),
            Err(ParseError::FailWildcard(e)) if *e.kind() == IntErrorKind::InvalidDigit
        ));
    }

    #[test]
    fn extend_range_1() {
        let mut range = -9..-9;
        extend_range(&mut range, -9..-6).unwrap();
        assert_eq!(range, -9..-6);
        extend_range(&mut range, -6..-3).unwrap();
        assert_eq!(range, -9..-3);
        extend_range(&mut range, -3..2).unwrap();
        assert_eq!(range, -9..2);

        assert_eq!(
            extend_range(&mut range, -16..-10),
            Err(ParseError::IncontinuousRange)
        );
        assert_eq!(
            extend_range(&mut range, 1..3),
            Err(ParseError::IncontinuousRange)
        );
    }
}
