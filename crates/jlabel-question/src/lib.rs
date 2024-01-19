mod parse_position;
pub mod position;

use std::num::ParseIntError;

use position::{
    AllPosition, BooleanPosition, CategoryPosition, PhonePosition, Position, SignedRangePosition,
    UndefinedPotision, UnsignedRangePosition,
};

use jlabel::Label;
use parse_position::{estimate_position, PositionError};

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("Position mismatch")]
    PositionMismatch,
    #[error("Invalid position")]
    InvalidPosition(PositionError),
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
    let [first, rest @ ..] = patterns else {
        return Err(ParseError::Empty);
    };
    let mut ranges = Vec::with_capacity(patterns.len());

    let (position, range) = estimate_position(first).map_err(ParseError::InvalidPosition)?;
    ranges.push(range);

    for pattern in rest {
        let (pos, range) = estimate_position(pattern).map_err(ParseError::InvalidPosition)?;
        if pos != position {
            return Err(ParseError::PositionMismatch);
        }
        ranges.push(range);
    }

    match_position!(
        position,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllQuestion {
    Phone(Question<PhonePosition>),
    SignedRange(Question<SignedRangePosition>),
    UnsignedRange(Question<UnsignedRangePosition>),
    Boolean(Question<BooleanPosition>),
    Category(Question<CategoryPosition>),
    Undefined(Question<UndefinedPotision>),
}

impl AllQuestion {
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
