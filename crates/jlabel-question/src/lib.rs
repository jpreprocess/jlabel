//! Here is the necessary condition for the pattern to succeed in parsing as [`AllQuestion`],
//! but some questions may not succeed even if they fulfill these requirements.
//!
//! - The patterns must be valid as htsvoice question pattern.
//!   - Using `*` and `?` as wildcard, matches the entire full-context label.
//!   - The pattern that cannot match full-context label in any situation (e.g. `*/A:-?????+*`) are not allowed.
//! - All the patterns must be about the same position (e.g. the first element of Phoneme, the last element of field `J`, etc.).
//! - Each pattern must *not* have conditions on two or more positions.
//! - When the pattern is about position of numerical field (except for categorical field such as `B`, `C`, or `D`),
//!   - The pattern must be continuous.
//!   - The pattern must be arranged in ascending order.
//!   - Minus sign (`-`) can only be used in the first element of `A`.
//!
//! Because this function cannot parse all of the valid htsvoice question patterns,
//! we recommend falling back to string&wildcard matching in case of failed parsing.

mod parse_position;
pub mod position;

#[cfg(feature = "regex")]
pub mod regex;

use std::num::ParseIntError;

use position::{
    AllPosition, BooleanPosition, CategoryPosition, PhonePosition, Position, SignedRangePosition,
    UndefinedPotision, UnsignedRangePosition,
};

use jlabel::Label;
use parse_position::{estimate_position, PositionError};

/// Errors from jlabel-question.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ParseError {
    /// Some patterns are pointing at position different from
    /// which the first pattern is pointing at.
    #[error("Position mismatch")]
    PositionMismatch,

    /// The pattern failed to parse.
    #[error("Invalid position")]
    InvalidPosition(#[from] PositionError),

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

    #[cfg(feature = "regex")]
    /// Failed to build regex parser from the provided pattern.
    #[error("Failed regex")]
    FailRegex,
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

pub trait QuestionMatcher
where
    Self: Sized,
{
    /// Parses question patterns in string, and if succeeds, returns the parsed question.
    fn parse(patterns: &[&str]) -> Result<Self, ParseError>;

    /// Checks if the full-context label matches the question.
    ///
    /// If you want to `test` on string label, parse it using `Label::from_str()` beforehand.
    fn test(&self, label: &Label) -> bool;
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

impl QuestionMatcher for AllQuestion {
    fn parse(patterns: &[&str]) -> Result<Self, ParseError> {
        let mut position = None;
        let mut ranges = Vec::with_capacity(patterns.len());

        for pattern in patterns {
            let (pos, range) = estimate_position(pattern)?;

            if let Some(position) = position {
                if pos != position {
                    return Err(ParseError::PositionMismatch);
                }
            } else {
                position = Some(pos);
            }

            ranges.push(range);
        }

        match_position!(
            position.ok_or(ParseError::Empty)?,
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
    fn test(&self, label: &Label) -> bool {
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
