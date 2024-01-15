pub mod position;

use position::{
    position, AllPosition, BooleanPosition, CategoryPosition, PhonePosition, Position,
    SignedRangePosition, UndefinedPotision, UnsignedRangePosition,
};

fn split_pattern(pattern: &str) -> Option<(String, String, String)> {
    let mut chars = pattern.chars();
    let mut prefix = String::new();
    let mut range = String::new();
    let mut suffix = String::new();

    let first = chars.next()?;
    if first == '*' {
        prefix.push(first);
        let second = chars.next()?;
        prefix.push(second);
        if second == '/' {
            prefix.push(chars.next()?); // `A`…`K`
            prefix.push(chars.next()?); // `:`
        }
    } else {
        range.push(first);
    }

    let last = chars.next_back()?;
    if last == '*' {
        suffix.push(last);
        let second_last = chars.next_back()?;
        suffix.push(second_last);
        if second_last == ':' {
            suffix.push(chars.next_back()?); // `A`…`K`
            suffix.push(chars.next_back()?); // `/`
        }
        range.push_str(&chars.collect::<String>());
    } else {
        range.push_str(&chars.collect::<String>());
        range.push(last);
    }

    suffix = suffix.chars().rev().collect();

    Some((prefix, range, suffix))
}

macro_rules! question_arm {
    ($name:ident, $position:expr, $triplets:expr) => {
        if $triplets.len() == 1 && $triplets[0].1 == "xx" {
            Some(AllQuestion::$name(Question::new_xx($position)))
        } else {
            let range = $position.range(&$triplets.iter().map(|(_, r, _)| r).collect::<Vec<&String>>());
            Some(AllQuestion::$name(Question::new($position, range)))
        }
    };
}

pub fn question(patterns: &[String]) -> Option<AllQuestion> {
    let mut triplets = Vec::new();
    for pattern in patterns {
        triplets.push(split_pattern(pattern)?);
    }

    let (prefix, _, suffix) = triplets.first()?;
    if !triplets
        .iter()
        .all(|(pre, _, post)| pre == prefix && post == suffix)
    {
        return None;
    }

    let position = position(prefix, suffix)?;

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
    pub fn new(position: P, range: Option<P::Range>) -> Self {
        Self { position, range }
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
        position::{PhonePosition, SignedRangePosition},
        question, AllQuestion, Question,
    };

    use super::split_pattern;

    #[test]
    fn splitter() {
        assert_eq!(
            split_pattern("a^*"),
            Some(("".to_string(), "a".to_string(), "^*".to_string()))
        );
        assert_eq!(
            split_pattern("*/A:-??+*"),
            Some(("*/A:".to_string(), "-??".to_string(), "+*".to_string()))
        );
        assert_eq!(
            split_pattern("*|?+*"),
            Some(("*|".to_string(), "?".to_string(), "+*".to_string()))
        );
        assert_eq!(
            split_pattern("*-1"),
            Some(("*-".to_string(), "1".to_string(), "".to_string()))
        );
    }

    #[test]
    fn parse_question() {
        assert_eq!(
            question(&["a^*".to_string(), "A^*".to_string()]),
            Some(AllQuestion::Phone(Question {
                position: PhonePosition::P1,
                range: Some(vec!["a".to_string(), "A".to_string()])
            }))
        );
        assert_eq!(
            question(&["*/A:-??+*".to_string(), "*/A:-9+*".to_string()]),
            Some(AllQuestion::SignedRange(Question {
                position: SignedRangePosition::A1,
                range: Some(-128..-9)
            }))
        );
    }
}
