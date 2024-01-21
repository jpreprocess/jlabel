use jlabel::Label;
use regex_automata::{meta::Regex, Anchored, Input};
use regex_syntax::hir::{Dot, Hir, Repetition};

use crate::{ParseError, QuestionMatcher};

#[derive(Debug, Clone)]
pub enum RegexFallback<T: QuestionMatcher> {
    Ok(T),
    Regex(RegexQuestion),
}

impl<T: QuestionMatcher> QuestionMatcher for RegexFallback<T> {
    fn parse(patterns: &[&str]) -> Result<Self, ParseError> {
        T::parse(patterns)
            .map(Self::Ok)
            .or_else(|_| RegexQuestion::parse(patterns).map(Self::Regex))
    }
    fn test(&self, label: &Label) -> bool {
        match &self {
            Self::Ok(inner) => inner.test(label),
            Self::Regex(regex) => regex.test(label),
        }
    }
}

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
    use std::str::FromStr;

    use jlabel::Label;

    use crate::{fallback::regex::RegexFallback, AllQuestion, QuestionMatcher};

    const TEST_LABEL:&str="sil^k-o+N=n/A:-4+1+5/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5";

    #[test]
    fn ok() {
        let label = Label::from_str(TEST_LABEL).unwrap();
        assert!(RegexFallback::<AllQuestion>::parse(&["*-o+*", "*-N+*"])
            .unwrap()
            .test(&label));
    }
    #[test]
    fn regex() {
        let label = Label::from_str(TEST_LABEL).unwrap();
        assert!(RegexFallback::<AllQuestion>::parse(&["*^k-o+*"])
            .unwrap()
            .test(&label));
        assert!(!RegexFallback::<AllQuestion>::parse(&["INVALID?*"])
            .unwrap()
            .test(&label));

        assert!(!RegexFallback::<AllQuestion>::parse(&["^k-o+*"])
            .unwrap()
            .test(&label));
    }
}
