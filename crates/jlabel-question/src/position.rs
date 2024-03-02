//! Structures for position

use std::{fmt::Debug, ops::Range};

use crate::Label;

use super::ParseError;

/// Enum that represent all positions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllPosition {
    /// Phone fields
    Phone(PhonePosition),
    /// Signed integer fields
    SignedRange(SignedRangePosition),
    /// Unsigned integer fields
    UnsignedRange(UnsignedRangePosition),
    /// Boolean fields
    Boolean(BooleanPosition),
    /// Numerical categorical fields
    Category(CategoryPosition),
    /// Undefined (always `xx`) fields
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

/// The trait that Position requires to implement
pub trait Position {
    /// The type of match target
    type Target;
    /// The type of range
    type Range;

    /// Parse range strings
    fn range(&self, ranges: &[&str]) -> Result<Self::Range, ParseError>;
    /// Get part of [`Label`] this position matches to.
    fn get<'a>(&self, label: &'a Label) -> Option<&'a Self::Target>;
    /// Check if the range matches target
    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool;
}

/// Positions of phone fields
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
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
            Self::P1 => label.phoneme.p2.as_ref(),
            Self::P2 => label.phoneme.p1.as_ref(),
            Self::P3 => label.phoneme.c.as_ref(),
            Self::P4 => label.phoneme.n1.as_ref(),
            Self::P5 => label.phoneme.n2.as_ref(),
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

/// Positions with signed integer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
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

/// Positions with unsigned integer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
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
    let merged = ranges
        .into_iter()
        .try_fold(None, |acc: Option<Range<Idx>>, curr| match acc {
            // By sorting, always acc.start <= curr.start
            // Only need to check curr's start is continuous with acc's end
            Some(mut acc) if curr.start <= acc.end => {
                acc.end = acc.end.max(curr.end);
                Ok(Some(acc))
            }
            None => Ok(Some(curr)),
            _ => Err(ParseError::IncontinuousRange),
        })?;
    merged.ok_or(ParseError::Empty)
}

/// Positions with boolean type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
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
        // E5/G5's logics are inverted
        let field_false = matches!(self, Self::E5 | Self::G5);
        match *first {
            "0" => Ok(field_false),
            "1" => Ok(!field_false),
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

/// Positions with numerical representations of categorical value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
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

/// Positions that are always `xx`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
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
    #[allow(clippy::single_range_in_vec_init)]
    fn merge_ranges_1() {
        assert_eq!(merge_ranges(vec![0..1]), Ok(0..1));
        assert_eq!(merge_ranges(vec![0..1, 1..3]), Ok(0..3));
        assert_eq!(merge_ranges(vec![1..3, 0..1]), Ok(0..3));
        assert_eq!(merge_ranges(vec![0..2, 1..3]), Ok(0..3));
        assert_eq!(merge_ranges(vec![-6..7, 1..3]), Ok(-6..7));
        assert_eq!(
            merge_ranges(vec![-6..7, 1..3, 2..6, -8..-7, -8..0]),
            Ok(-8..7)
        );

        assert_eq!(merge_ranges::<u8>(vec![]), Err(ParseError::Empty));
        assert_eq!(
            merge_ranges(vec![0..1, 5..6]),
            Err(ParseError::IncontinuousRange)
        );
        assert_eq!(
            merge_ranges(vec![3..6, -1..2]),
            Err(ParseError::IncontinuousRange)
        );
        assert_eq!(
            merge_ranges(vec![-6..7, 1..3, 2..6, -8..-7]),
            Err(ParseError::IncontinuousRange)
        );
    }
}
