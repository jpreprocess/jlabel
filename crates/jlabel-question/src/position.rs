use std::{fmt::Debug, ops::Range};

use crate::Label;

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
        ("*-", "/H:*") => {
            // due to some bug in htsvoice, this arm is needed
            eprintln!("Warning: symbol before g5 should be `_` instead of `-`");
            Some(Boolean(G5))
        }

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

macro_rules! as_ref_map {
    ($label:ident.$block:ident.$prop:ident) => {
        $label.$block.as_ref().map(|b| &b.$prop)
    };
}

macro_rules! as_ref_and_then {
    ($label:ident.$block:ident.$prop:ident) => {
        $label.$block.as_ref().and_then(|b| b.$prop.as_ref())
    };
}

pub trait Position {
    type Target;
    type Range;

    fn range(&self, ranges: &[&str]) -> Result<Self::Range, ParseError>;
    fn get<'a>(&self, label: &'a Label) -> Option<&'a Self::Target>;
    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn range(&self, ranges: &[&str]) -> Result<Self::Range, ParseError> {
        Ok(ranges.iter().map(|s| s.to_string()).collect())
    }

    fn get<'a>(&self, label: &'a Label) -> Option<&'a Self::Target> {
        match self {
            Self::P1 => label.phoneme.p1.as_ref(),
            Self::P2 => label.phoneme.p2.as_ref(),
            Self::P3 => label.phoneme.c.as_ref(),
            Self::P4 => label.phoneme.n1.as_ref(),
            Self::P5 => label.phoneme.n2.as_ref(),
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignedRangePosition {
    A1,
}

impl Position for SignedRangePosition {
    type Target = i8;
    type Range = Range<i8>;

    fn range(&self, ranges: &[&str]) -> Result<Self::Range, ParseError> {
        let parsed_ranges = ranges.iter().map(range_i8).collect::<Result<Vec<_>, _>>()?;
        merge_ranges(parsed_ranges)
    }

    fn get<'a>(&self, label: &'a Label) -> Option<&'a Self::Target> {
        match self {
            Self::A1 => as_ref_map!(label.mora.relative_accent_position),
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

fn range_i8<S: AsRef<str>>(s: S) -> Result<Range<i8>, ParseError> {
    let range = match s.as_ref() {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn range(&self, ranges: &[&str]) -> Result<Self::Range, ParseError> {
        let parsed_ranges = ranges.iter().map(range_u8).collect::<Result<Vec<_>, _>>()?;
        merge_ranges(parsed_ranges)
    }

    fn get<'a>(&self, label: &'a Label) -> Option<&'a Self::Target> {
        match self {
            Self::A2 => as_ref_map!(label.mora.position_forward),
            Self::A3 => as_ref_map!(label.mora.position_backward),
            Self::E1 => as_ref_map!(label.accent_phrase_prev.mora_count),
            Self::E2 => as_ref_map!(label.accent_phrase_prev.accent_position),
            Self::F1 => as_ref_map!(label.accent_phrase_curr.mora_count),
            Self::F2 => as_ref_map!(label.accent_phrase_curr.accent_position),
            Self::F5 => as_ref_map!(label.accent_phrase_curr.accent_phrase_position_forward),
            Self::F6 => as_ref_map!(label.accent_phrase_curr.accent_phrase_position_backward),
            Self::F7 => as_ref_map!(label.accent_phrase_curr.mora_position_forward),
            Self::F8 => as_ref_map!(label.accent_phrase_curr.mora_position_backward),
            Self::G1 => as_ref_map!(label.accent_phrase_next.mora_count),
            Self::G2 => as_ref_map!(label.accent_phrase_next.accent_position),
            Self::H1 => as_ref_map!(label.breath_group_prev.accent_phrase_count),
            Self::H2 => as_ref_map!(label.breath_group_prev.mora_count),
            Self::I1 => as_ref_map!(label.breath_group_curr.accent_phrase_count),
            Self::I2 => as_ref_map!(label.breath_group_curr.mora_count),
            Self::I3 => as_ref_map!(label.breath_group_curr.breath_group_position_forward),
            Self::I4 => as_ref_map!(label.breath_group_curr.breath_group_position_backward),
            Self::I5 => as_ref_map!(label.breath_group_curr.accent_phrase_position_forward),
            Self::I6 => as_ref_map!(label.breath_group_curr.accent_phrase_position_backward),
            Self::I7 => as_ref_map!(label.breath_group_curr.mora_position_forward),
            Self::I8 => as_ref_map!(label.breath_group_curr.mora_position_backward),
            Self::J1 => as_ref_map!(label.breath_group_next.accent_phrase_count),
            Self::J2 => as_ref_map!(label.breath_group_next.mora_count),
            Self::K1 => Some(&label.utterance.breath_group_count),
            Self::K2 => Some(&label.utterance.accent_phrase_count),
            Self::K3 => Some(&label.utterance.mora_count),
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

fn range_u8<S: AsRef<str>>(s: S) -> Result<Range<u8>, ParseError> {
    let range = match s.as_ref() {
        "?" => 1..10,
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

fn merge_ranges<Idx>(mut ranges: Vec<Range<Idx>>) -> Result<Range<Idx>, ParseError>
where
    Idx: Ord + Copy,
{
    ranges.sort_unstable_by_key(|range| range.start);
    ranges
        .into_iter()
        .fold(Err(ParseError::Empty), |acc, curr| match acc {
            Err(ParseError::Empty) => Ok(curr),
            Ok(mut acc) if curr.start <= acc.end => {
                acc.start = acc.start.min(curr.start);
                acc.end = acc.end.max(curr.end);
                Ok(acc)
            }
            _ => Err(ParseError::IncontinuousRange),
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn range(&self, ranges: &[&str]) -> Result<Self::Range, ParseError> {
        let first = ranges.first().ok_or(ParseError::Empty)?;
        match *first {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(ParseError::InvalidBoolean(first.to_string())),
        }
    }

    fn get<'a>(&self, label: &'a Label) -> Option<&'a Self::Target> {
        match self {
            Self::E3 => as_ref_map!(label.accent_phrase_prev.is_interrogative),
            Self::E5 => as_ref_and_then!(label.accent_phrase_prev.is_pause_insertion),
            Self::F3 => as_ref_map!(label.accent_phrase_curr.is_interrogative),
            Self::G3 => as_ref_map!(label.accent_phrase_next.is_interrogative),
            Self::G5 => as_ref_and_then!(label.accent_phrase_next.is_pause_insertion),
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range == target
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn range(&self, ranges: &[&str]) -> Result<Self::Range, ParseError> {
        ranges
            .iter()
            .map(|s| s.parse::<u8>().map_err(ParseError::FailLiteral))
            .collect()
    }

    fn get<'a>(&self, label: &'a Label) -> Option<&'a Self::Target> {
        match self {
            Self::B1 => as_ref_and_then!(label.word_prev.pos),
            Self::B2 => as_ref_and_then!(label.word_prev.ctype),
            Self::B3 => as_ref_and_then!(label.word_prev.cform),
            Self::C1 => as_ref_and_then!(label.word_curr.pos),
            Self::C2 => as_ref_and_then!(label.word_curr.ctype),
            Self::C3 => as_ref_and_then!(label.word_curr.cform),
            Self::D1 => as_ref_and_then!(label.word_next.pos),
            Self::D2 => as_ref_and_then!(label.word_next.ctype),
            Self::D3 => as_ref_and_then!(label.word_next.cform),
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UndefinedPotision {
    E4,
    F4,
    G4,
}

impl Position for UndefinedPotision {
    type Target = ();
    type Range = ();

    fn range(&self, _: &[&str]) -> Result<Self::Range, ParseError> {
        Ok(())
    }

    fn get<'a>(&self, _: &'a Label) -> Option<&'a Self::Target> {
        None
    }

    fn test(&self, _: &Self::Range, _: &Self::Target) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(range_u8("?"), Ok(1..10));
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
    fn merge_ranges_1() {
        assert_eq!(merge_ranges(vec![0..1]), Ok(0..1));
        assert_eq!(merge_ranges(vec![0..1, 1..3]), Ok(0..3));
        assert_eq!(merge_ranges(vec![1..3, 0..1]), Ok(0..3));
        assert_eq!(merge_ranges(vec![0..2, 1..3]), Ok(0..3));
        assert_eq!(merge_ranges(vec![-6..7, 1..3]), Ok(-6..7));

        assert_eq!(
            merge_ranges(vec![0..1, 5..6]),
            Err(ParseError::IncontinuousRange)
        );
        assert_eq!(
            merge_ranges(vec![3..6, -1..2]),
            Err(ParseError::IncontinuousRange)
        );
    }
}
