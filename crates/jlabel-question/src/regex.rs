use jlabel::Label;
use regex_automata::{meta::Regex, Anchored, Input};
use regex_syntax::hir::{Dot, Hir, Repetition};

use crate::{ParseError, QuestionMatcher};

#[derive(Debug, Clone)]
pub struct RegexQuestion(Regex);

impl RegexQuestion {
    fn parse_wildcard<S: AsRef<str>>(pattern: S) -> Hir {
        Hir::concat(
            pattern
                .as_ref()
                .bytes()
                .map(|c| match c {
                    b'*' => Hir::repetition(Repetition {
                        min: 0,
                        max: None,
                        greedy: true,
                        sub: Box::new(Hir::dot(Dot::AnyByteExceptLF)),
                    }),
                    b'?' => Hir::dot(Dot::AnyByteExceptLF),
                    c => Hir::literal([c]),
                })
                .collect(),
        )
    }
}

impl QuestionMatcher for RegexQuestion {
    fn parse(patterns: &[&str]) -> Result<Self, ParseError> {
        let regex = Regex::builder()
            .build_from_hir(&Hir::alternation(
                patterns.iter().map(Self::parse_wildcard).collect(),
            ))
            .or(Err(ParseError::FailRegex))?;
        Ok(Self(regex))
    }
    fn test(&self, label: &Label) -> bool {
        self.0
            .is_match(Input::new(&label.to_string()).anchored(Anchored::Yes))
    }
}

#[cfg(test)]
mod tests {
    use super::RegexQuestion;

    #[test]
    fn regex() {
        const TEST_LABEL:&str="sil^k-o+N=n/A:-4+1+5/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5";

        use crate::QuestionMatcher;
        use jlabel::Label;
        use std::str::FromStr;

        let label = Label::from_str(TEST_LABEL).unwrap();

        assert!(RegexQuestion::parse(&["*^k-o+*"]).unwrap().test(&label));
        assert!(!RegexQuestion::parse(&["INVALID?*"]).unwrap().test(&label));

        assert!(!RegexQuestion::parse(&["^k-o+*"]).unwrap().test(&label));
    }
    #[test]
    fn wildcard() {
        use regex_syntax::hir::*;
        assert_eq!(
            RegexQuestion::parse_wildcard("*?^k-?o+*"),
            Hir::concat(vec![
                Hir::repetition(Repetition {
                    min: 0,
                    max: None,
                    greedy: true,
                    sub: Box::new(Hir::dot(Dot::AnyByteExceptLF)),
                }),
                Hir::dot(Dot::AnyByteExceptLF),
                Hir::literal(*b"^k-"),
                Hir::dot(Dot::AnyByteExceptLF),
                Hir::literal(*b"o+"),
                Hir::repetition(Repetition {
                    min: 0,
                    max: None,
                    greedy: true,
                    sub: Box::new(Hir::dot(Dot::AnyByteExceptLF)),
                })
            ])
        );
    }
}
