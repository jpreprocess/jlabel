pub mod position;

use std::num::ParseIntError;

use position::{
    position, AllPosition, BooleanPosition, CategoryPosition, PhonePosition, Position,
    SignedRangePosition, UndefinedPotision, UnsignedRangePosition,
};

use crate::Label;

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
}

fn split_pattern(pattern: &str) -> Option<(&str, &str, &str)> {
    if !pattern.len() < 4 {
        return None;
    }

    let start = if pattern.starts_with("*/") {
        4
    } else if pattern.starts_with('*') {
        2
    } else {
        0
    };
    let end = if pattern.ends_with(":*") {
        pattern.len() - 4
    } else if pattern.ends_with('*') {
        pattern.len() - 2
    } else {
        pattern.len()
    };

    Some((&pattern[..start], &pattern[start..end], &pattern[end..]))
}

macro_rules! question_arm {
    ($name:ident, $position:expr, $triplets:expr) => {
        if $triplets.len() == 1 && $triplets[0].1 == "xx" {
            Ok(AllQuestion::$name(Question::new_xx($position)))
        } else {
            let range =
                $position.range(&$triplets.into_iter().map(|(_, r, _)| r).collect::<Vec<_>>())?;
            Ok(AllQuestion::$name(Question::new($position, range)))
        }
    };
}

pub fn question(patterns: &[&str]) -> Result<AllQuestion, ParseError> {
    let mut triplets = Vec::new();
    for pattern in patterns {
        triplets.push(split_pattern(pattern).ok_or(ParseError::FailSplitting)?);
    }

    let (prefix, _, suffix) = triplets.first().ok_or(ParseError::Empty)?;
    if !triplets
        .iter()
        .all(|(pre, _, post)| pre == prefix && post == suffix)
    {
        return Err(ParseError::PositionMismatch);
    }

    let position = position(prefix, suffix).ok_or(ParseError::InvalidPosition)?;

    use AllPosition::*;
    match position {
        Phone(position) => question_arm!(Phone, position, triplets),
        SignedRange(position) => question_arm!(SignedRange, position, triplets),
        UnsignedRange(position) => question_arm!(UnsignedRange, position, triplets),
        Boolean(position) => question_arm!(Boolean, position, triplets),
        Category(position) => question_arm!(Category, position, triplets),
        Undefined(position) => question_arm!(Undefined, position, triplets),
    }
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
    pub fn new(position: P, range: P::Range) -> Self {
        Self {
            position,
            range: Some(range),
        }
    }

    pub fn new_xx(position: P) -> Self {
        Self {
            position,
            range: None,
        }
    }

    pub fn test(&self, label: &Label) -> bool {
        match (&self.range, self.position.get(&label)) {
            (Some(range), Some(target)) => self.position.test(range, target),
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests;
