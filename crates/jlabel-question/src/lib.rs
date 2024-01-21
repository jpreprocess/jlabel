pub mod fallback;
pub mod position;

use std::num::ParseIntError;

use position::{
    position, AllPosition, BooleanPosition, CategoryPosition, PhonePosition, Position,
    SignedRangePosition, UndefinedPotision, UnsignedRangePosition,
};

use jlabel::Label;

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("Failed splitting")]
    FailSplitting,
    #[error("Position mismatch")]
    PositionMismatch,
    #[error("Invalid position")]
    InvalidPosition,
    #[error("Empty patterns or range")]
    Empty,
    #[error("Incontinuous range")]
    IncontinuousRange,
    #[error("Failed wildcard: {0}")]
    FailWildcard(ParseIntError),
    #[error("Failed literal: {0}")]
    FailLiteral(ParseIntError),
    #[error("Invalid boolean: {0}")]
    InvalidBoolean(String),
    #[error("Failed regex")]
    FailRegex,
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

pub fn question(patterns: &[&str]) -> Result<AllQuestion, ParseError> {
    AllQuestion::parse(patterns)
}

pub trait QuestionMatcher
where
    Self: Sized,
{
    fn parse(patterns: &[&str]) -> Result<Self, ParseError>;
    fn test(&self, label: &Label) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllQuestion {
    Phone(Question<PhonePosition>),
    SignedRange(Question<SignedRangePosition>),
    UnsignedRange(Question<UnsignedRangePosition>),
    Boolean(Question<BooleanPosition>),
    Category(Question<CategoryPosition>),
    Undefined(Question<UndefinedPotision>),
}

impl QuestionMatcher for AllQuestion{
    fn parse(patterns: &[&str]) -> Result<Self, ParseError> {
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
