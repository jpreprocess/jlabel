pub mod position;

use position::{
    position, AllPosition, BooleanPosition, CategoryPosition, PhonePosition, Position,
    SignedRangePosition, UndefinedPotision, UnsignedRangePosition,
};

fn split_pattern(pattern: &str) -> Option<(String, String, String)> {
    let mut chars = pattern.chars();
    let mut prefix = String::new();
    let mut range = String::new();
    let mut postfix = String::new();

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
        postfix.push(last);
        let second_last = chars.next_back()?;
        postfix.push(second_last);
        if second_last == ':' {
            postfix.push(chars.next_back()?); // `A`…`K`
            postfix.push(chars.next_back()?); // `/`
        }
        range.push_str(&chars.collect::<String>());
    } else {
        range.push_str(&chars.collect::<String>());
        postfix.push(last);
    }

    Some((prefix, range, postfix))
}

macro_rules! question_arm {
    ($name:ident, $position:expr, $triplets:expr) => {
        if $triplets.len() == 1 && $triplets[0].1 == "xx" {
            Some(AllQuestion::$name(Question::new_xx($position)))
        } else {
            let range = $position.range(&$triplets.iter().map(|(_, r, _)| r).collect());
            Some(AllQuestion::$name(Question::new($position, range)))
        }
    };
}

pub fn question(patterns: &[String]) -> Option<AllQuestion> {
    let mut triplets = Vec::new();
    for pattern in patterns {
        triplets.push(split_pattern(pattern)?);
    }

    let (prefix, _, postfix) = triplets.first()?;
    if !triplets
        .iter()
        .all(|(pre, _, post)| pre == prefix && post == postfix)
    {
        return None;
    }

    let position = position(prefix, postfix)?;

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
