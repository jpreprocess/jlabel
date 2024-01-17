pub mod position;

use std::num::ParseIntError;

use position::{
    position, AllPosition, BooleanPosition, CategoryPosition, PhonePosition, Position,
    SignedRangePosition, UndefinedPotision, UnsignedRangePosition,
};

use jlabel::Label;

/// Errors from jlabel-question.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ParseError {
    /// Failed to split given pattern to prefix,target, and suffix
    #[error("Failed splitting")]
    FailSplitting,

    /// Some patterns are pointing at position different from
    /// which the first pattern is pointing at.
    #[error("Position mismatch")]
    PositionMismatch,

    /// The extracted position prefix/suffix did not match
    /// any of the possible patterns.
    #[error("Invalid position")]
    InvalidPosition,

    /// The provided pattern or range is empty, so jlabel-question cannot parse it.
    #[error("Empty patterns or range")]
    Empty,

    /// The range is incontinuous or not arranged in ascending order.
    #[error("Incontinuous range")]
    IncontinuousRange,

    /// Failed to parse integer field in a pattern containing wildcard.
    /// This might result from incorrect number of wildcards.
    #[error("Failed wildcard: {0}")]
    FailWildcard(ParseIntError),

    /// Failed to parse integer field in a pattern without wildcard.
    /// This might result from incorrect position of wildcard such as `1?2`.
    #[error("Failed literal: {0}")]
    FailLiteral(ParseIntError),

    /// Failed to parse boolean field.
    /// Boolean fields must be either `0` or `1` (except for `xx` which means empty).
    #[error("Invalid boolean: {0}")]
    InvalidBoolean(String),
}

fn split_pattern(pattern: &str) -> Option<(&str, &str, &str)> {
    let start = if pattern.starts_with("*/") {
        4
    } else if pattern.starts_with('*') {
        2
    } else {
        0
    };
    let end = if pattern.ends_with(":*") {
        pattern.len().checked_sub(4)?
    } else if pattern.ends_with('*') {
        pattern.len().checked_sub(2)?
    } else {
        pattern.len()
    };
    if start > end {
        return None;
    }

    Some((&pattern[..start], &pattern[start..end], &pattern[end..]))
}

macro_rules! match_position {
    ($position:expr, $ranges:expr, [$($name:ident),*]) => {
        match $position {
            $(
                AllPosition::$name(position) => Ok(AllQuestion::$name(Question::new(position, $ranges)?)),
            )*
        }
    };
}

/// Parses question patterns in string, and if succeeds, returns the parsed question ([`AllQuestion`]).
///
/// Here is the necessary condition for the pattern to succeed in parsing,
/// but some questions may not succeed even if they fulfill these requirements.
///
/// - The patterns must be valid as htsvoice question pattern.
///   - Using `*` and `?` as wildcard, matches the entire full-context label.
///   - The pattern that cannot match full-context label in any situation (e.g. `*/A:-?????+*`) are not allowed.
/// - All the patterns must be about the same position (e.g. the first element of Phoneme, the last element of field `J`, etc.).
/// - Each pattern must *not* have conditions on two or more positions.
/// - When the pattern is about position of numerical field (except for categorical field such as `B`, `C`, or `D`),
///   - The pattern must be continuous.
///   - The pattern must be arranged in ascending order.
///   - Minus sign (`-`) can only be used in the first element of `A`.
///
/// Because this function cannot parse all of the valid htsvoice question patterns,
/// we recommend falling back to string&wildcard matching in case of failed parsing.
pub fn question(patterns: &[&str]) -> Result<AllQuestion, ParseError> {
    let [first, rest @ ..] = patterns else {
        return Err(ParseError::Empty);
    };
    let (prefix, range, suffix) = split_pattern(first).ok_or(ParseError::FailSplitting)?;

    let mut ranges = Vec::with_capacity(patterns.len());
    ranges.push(range);

    for pattern in rest {
        let (pre, range, suf) = split_pattern(pattern).ok_or(ParseError::FailSplitting)?;
        if pre != prefix || suf != suffix {
            return Err(ParseError::PositionMismatch);
        }
        ranges.push(range);
    }

    match_position!(
        position(prefix, suffix).ok_or(ParseError::InvalidPosition)?,
        &ranges,
        [
            Phone,
            SignedRange,
            UnsignedRange,
            Boolean,
            Category,
            Undefined
        ]
    )
}

/// A main structure representing question.
///
/// This can be created from slice of strings using [`question`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllQuestion {
    /// Question about phone fields of full-context label
    Phone(Question<PhonePosition>),
    /// Question about signed integer fields of full-context label
    SignedRange(Question<SignedRangePosition>),
    /// Question about unsigned integer fields of full-context label
    UnsignedRange(Question<UnsignedRangePosition>),
    /// Question about boolean fields of full-context label
    Boolean(Question<BooleanPosition>),
    /// Question about numerical categorical fields of full-context label
    Category(Question<CategoryPosition>),
    /// Question about undefined (always `xx`) fields of full-context label
    Undefined(Question<UndefinedPotision>),
}

impl AllQuestion {
    /// Checks if the full-context label matches the question.
    ///
    /// If you want to `test` on string label, parse it using `Label::from_str()` beforehand.
    pub fn test(&self, label: &Label) -> bool {
        match self {
            Self::Phone(q) => q.test(label),
            Self::SignedRange(q) => q.test(label),
            Self::UnsignedRange(q) => q.test(label),
            Self::Boolean(q) => q.test(label),
            Self::Category(q) => q.test(label),
            Self::Undefined(q) => q.test(label),
        }
    }
}

/// An inner structure representing a pair of position and range.
///
/// Used in variants of [`AllQuestion`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question<P: Position> {
    pub position: P,
    pub range: Option<P::Range>,
}

impl<P: Position> Question<P> {
    pub fn new(position: P, ranges: &[&str]) -> Result<Self, ParseError> {
        match ranges {
            ["xx"] => Ok(Self {
                range: None,
                position,
            }),
            ranges => Ok(Self {
                range: Some(position.range(ranges)?),
                position,
            }),
        }
    }

    pub fn test(&self, label: &Label) -> bool {
        match (&self.range, self.position.get(label)) {
            (Some(range), Some(target)) => self.position.test(range, target),
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests;
