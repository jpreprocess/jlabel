use jlabel::Label;

use crate::{ParseError, QuestionMatcher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoopFallback<T: QuestionMatcher> {
    Ok(T),
    Noop,
}

impl<T: QuestionMatcher> QuestionMatcher for NoopFallback<T> {
    fn parse(patterns: &[&str]) -> Result<Self, ParseError> {
        T::parse(patterns).map(Self::Ok).or(Ok(Self::Noop))
    }
    fn test(&self, label: &Label) -> bool {
        match &self {
            Self::Ok(inner) => inner.test(label),
            Self::Noop => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use jlabel::Label;

    use crate::{fallback::noop::NoopFallback, AllQuestion, QuestionMatcher};

    const TEST_LABEL:&str="sil^k-o+N=n/A:-4+1+5/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5";

    #[test]
    fn ok() {
        let label = Label::from_str(TEST_LABEL).unwrap();
        assert!(NoopFallback::<AllQuestion>::parse(&["*-o+*", "*-N+*"])
            .unwrap()
            .test(&label));
    }
    #[test]
    fn noop() {
        let label = Label::from_str(TEST_LABEL).unwrap();
        assert!(!NoopFallback::<AllQuestion>::parse(&["INVALID?*"])
            .unwrap()
            .test(&label));
    }
}
