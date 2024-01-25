#![deny(missing_docs)]
//! HTS-style full-context label question parser and matcher.
//!
//! The main structure for parsing and matching is [`AllQuestion`].
//! It can parse most patterns, but it cannot parse some of them.
//! For details, please see [Condition for parsing as AllQuestion].
//!
//! [Condition for parsing as AllQuestion]: #condition-for-parsing-as-allquestion
//!
//! ```rust
//! # use std::error::Error;
//! use jlabel::Label;
//! use jlabel_question::{AllQuestion, QuestionMatcher};
//!
//! use std::str::FromStr;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let question = AllQuestion::parse(&["*/A:-??+*", "*/A:-?+*"])?;
//! let label_str = concat!(
//!     "sil^n-i+h=o",
//!     "/A:-3+1+7",
//!     "/B:xx-xx_xx",
//!     "/C:02_xx+xx",
//!     "/D:02+xx_xx",
//!     "/E:xx_xx!xx_xx-xx",
//!     "/F:7_4#0_xx@1_3|1_12",
//!     "/G:4_4%0_xx_1",
//!     "/H:xx_xx",
//!     "/I:3-12@1+2&1-8|1+41",
//!     "/J:5_29",
//!     "/K:2+8-41"
//! );
//! assert!(question.test(&label_str.parse()?));
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! ## Condition for parsing as [`AllQuestion`]
//!
//! Here is the necessary condition for the pattern to succeed in parsing as [`AllQuestion`],
//! but some questions may not succeed even if they fulfill these requirements.
//!
//! - The patterns must be valid as htsvoice question pattern.
//!   - Using `*` and `?` as wildcard, matches the entire full-context label.
//!   - The pattern that cannot match full-context label in any situation (e.g. `*/A:-?????+*`) are not allowed.
//!   - Minus sign (`-`) in numerical field can only be used in the first element of `A` (`A1`).
//! - All the patterns must be about the same position
//!   - e.g. The first pattern is about the first element of Phoneme, the second pattern is about the last element of field `J`, is *not* allowed.
//! - Each pattern must *not* have conditions on two or more positions.
//! - When the pattern is about position of numerical field (except for categorical field such as `B`, `C`, or `D`),
//!   - The pattern must be continuous.
//!
//! ## Fallback
//!
//! As [`AllQuestion`] parsing does not always suceed (even if the pattern is correct),
//! you may need to write fallback for that.
//!
//! If you just want to ignore those pattern, you can simply return `false` instead of the result of `test()`.
//!
//! If you need to successfully parse pattern which [`AllQuestion`] fails to parse,
//! [`regex::RegexQuestion`] is the best choice.
//!
//! ```rust
//! # #[cfg(feature = "regex")]
//! # {
//! use jlabel::Label;
//! use jlabel_question::{regex::RegexQuestion, AllQuestion, ParseError, QuestionMatcher};
//!
//! enum Pattern {
//!     AllQustion(AllQuestion),
//!     Regex(RegexQuestion),
//! }
//! impl Pattern {
//!     fn parse(patterns: &[&str]) -> Result<Self, ParseError> {
//!         match AllQuestion::parse(patterns) {
//!             Ok(question) => Ok(Self::AllQustion(question)),
//!             Err(_) => Ok(Self::Regex(RegexQuestion::parse(patterns)?)),
//!         }
//!     }
//!     fn test(&self, label: &Label) -> bool {
//!         match self {
//!             Self::AllQustion(question) => question.test(label),
//!             Self::Regex(question) => question.test(label),
//!         }
//!     }
//! }
//! # }
//! ```

pub mod parse_position;
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

    /// The pattern or range is empty, so jlabel-question cannot parse it.
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
    /// Failed to build regex parser from the pattern.
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

/// Parses the question, and tests it aganinst given full-context label.
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
    /// The position this question matches to.
    pub position: P,
    /// The parsed range
    pub range: Option<P::Range>,
}

impl<P: Position> Question<P> {
    /// Parse question pattern
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

    /// Check if this question matches
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
