pub mod position;

use std::num::ParseIntError;

use position::{
    position, AllPosition, BooleanPosition, CategoryPosition, PhonePosition, Position,
    SignedRangePosition, UndefinedPotision, UnsignedRangePosition,
};

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

    pub fn test(&self, target: &Option<P::Target>) -> bool {
        match (&self.range, target) {
            (Some(range), Some(target)) => self.position.test(range, target),
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::question::{
        position::{
            BooleanPosition, CategoryPosition, PhonePosition, SignedRangePosition,
            UndefinedPotision, UnsignedRangePosition,
        },
        question, AllQuestion, Question,
    };

    use super::split_pattern;

    #[test]
    fn splitter() {
        assert_eq!(split_pattern("a^*"), Some(("", "a", "^*")));
        assert_eq!(split_pattern("*/A:-??+*"), Some(("*/A:", "-??", "+*")));
        assert_eq!(split_pattern("*|?+*"), Some(("*|", "?", "+*")));
        assert_eq!(split_pattern("*-1"), Some(("*-", "1", "")));
    }

    #[test]
    fn parse_question() {
        assert_eq!(
            question(&["a^*", "A^*"]).unwrap(),
            AllQuestion::Phone(Question {
                position: PhonePosition::P1,
                range: Some(vec!["a".to_string(), "A".to_string()])
            })
        );
        assert_eq!(
            question(&["*/A:-3+*"]).unwrap(),
            AllQuestion::SignedRange(Question {
                position: SignedRangePosition::A1,
                range: Some(-3..-2)
            })
        );
        assert_eq!(
            question(&["*/A:-??+*", "*/A:-?+*", "*/A:?+*", "*/A:10+*", "*/A:11+*",]).unwrap(),
            AllQuestion::SignedRange(Question {
                position: SignedRangePosition::A1,
                range: Some(-99..12)
            })
        );
        assert_eq!(
            question(&["*_42/I:*"]).unwrap(),
            AllQuestion::UnsignedRange(Question {
                position: UnsignedRangePosition::H2,
                range: Some(42..43)
            })
        );
        assert_eq!(
            question(&["*_?/I:*", "*_1?/I:*", "*_2?/I:*", "*_30/I:*", "*_31/I:*",]).unwrap(),
            AllQuestion::UnsignedRange(Question {
                position: UnsignedRangePosition::H2,
                range: Some(1..32)
            })
        );
        assert_eq!(
            question(&["*%1_*"]).unwrap(),
            AllQuestion::Boolean(Question {
                position: BooleanPosition::G3,
                range: Some(true)
            })
        );
        assert_eq!(
            question(&["*/B:17-*", "*/B:20-*"]).unwrap(),
            AllQuestion::Category(Question {
                position: CategoryPosition::B1,
                range: Some(vec![17, 20])
            })
        );
        assert_eq!(
            question(&["*_xx_*"]).unwrap(),
            AllQuestion::Undefined(Question {
                position: UndefinedPotision::G4,
                range: None
            })
        );
    }
}
