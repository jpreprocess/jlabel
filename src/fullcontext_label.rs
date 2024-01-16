use crate::{
    question::position::{
        BooleanPosition, CategoryPosition, PhonePosition, SignedRangePosition,
        UnsignedRangePosition,
    },
    question::AllQuestion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub phoneme: Phoneme,
    pub mora: Option<Mora>,
    pub word_prev: Option<Word>,
    pub word_curr: Option<Word>,
    pub word_next: Option<Word>,
    pub accent_phrase_prev: Option<AccentPhrasePrevNext>,
    pub accent_phrase_curr: Option<AccentPhraseCurrent>,
    pub accent_phrase_next: Option<AccentPhrasePrevNext>,
    pub breath_group_prev: Option<BreathGroupPrevNext>,
    pub breath_group_curr: Option<BreathGroupCurrent>,
    pub breath_group_next: Option<BreathGroupPrevNext>,
    pub utterance: Utterance,
}

impl Label {
    pub fn satisfies(&self, question: &AllQuestion) -> bool {
        use AllQuestion::*;
        match question {
            Phone(question) => {
                use PhonePosition::*;
                match question.position {
                    P1 => question.test(&self.phoneme.p1),
                    P2 => question.test(&self.phoneme.p2),
                    P3 => question.test(&self.phoneme.c),
                    P4 => question.test(&self.phoneme.n1),
                    P5 => question.test(&self.phoneme.n2),
                }
            }
            SignedRange(question) => {
                use SignedRangePosition::*;
                match question.position {
                    A1 => question.test(&self.mora.as_ref().map(|m| m.relative_accent_position)),
                }
            }
            UnsignedRange(question) => {
                use UnsignedRangePosition::*;
                match question.position {
                    A2 => question.test(&self.mora.as_ref().map(|m| m.position_forward)),
                    A3 => question.test(&self.mora.as_ref().map(|m| m.position_backward)),
                    E1 => question.test(&self.accent_phrase_prev.as_ref().map(|a| a.mora_count)),
                    E2 => {
                        question.test(&self.accent_phrase_prev.as_ref().map(|a| a.accent_position))
                    }
                    F1 => question.test(&self.accent_phrase_curr.as_ref().map(|a| a.mora_count)),
                    F2 => {
                        question.test(&self.accent_phrase_curr.as_ref().map(|a| a.accent_position))
                    }
                    F5 => question.test(
                        &self
                            .accent_phrase_curr
                            .as_ref()
                            .map(|a| a.accent_phrase_position_forward),
                    ),
                    F6 => question.test(
                        &self
                            .accent_phrase_curr
                            .as_ref()
                            .map(|a| a.accent_phrase_position_backward),
                    ),
                    F7 => question.test(
                        &self
                            .accent_phrase_curr
                            .as_ref()
                            .map(|a| a.mora_position_forward),
                    ),
                    F8 => question.test(
                        &self
                            .accent_phrase_curr
                            .as_ref()
                            .map(|a| a.mora_position_backward),
                    ),
                    G1 => question.test(&self.accent_phrase_next.as_ref().map(|a| a.mora_count)),
                    G2 => {
                        question.test(&self.accent_phrase_next.as_ref().map(|a| a.accent_position))
                    }
                    H1 => question.test(
                        &self
                            .breath_group_prev
                            .as_ref()
                            .map(|b| b.accent_phrase_count),
                    ),
                    H2 => question.test(&self.breath_group_prev.as_ref().map(|b| b.mora_count)),
                    I1 => question.test(
                        &self
                            .breath_group_curr
                            .as_ref()
                            .map(|b| b.accent_phrase_count),
                    ),
                    I2 => question.test(&self.breath_group_curr.as_ref().map(|b| b.mora_count)),
                    I3 => question.test(
                        &self
                            .breath_group_curr
                            .as_ref()
                            .map(|b| b.breath_group_position_forward),
                    ),
                    I4 => question.test(
                        &self
                            .breath_group_curr
                            .as_ref()
                            .map(|b| b.breath_group_position_backward),
                    ),
                    I5 => question.test(
                        &self
                            .breath_group_curr
                            .as_ref()
                            .map(|b| b.accent_phrase_position_forward),
                    ),
                    I6 => question.test(
                        &self
                            .breath_group_curr
                            .as_ref()
                            .map(|b| b.accent_phrase_position_backward),
                    ),
                    I7 => question.test(
                        &self
                            .breath_group_curr
                            .as_ref()
                            .map(|b| b.mora_position_forward),
                    ),
                    I8 => question.test(
                        &self
                            .breath_group_curr
                            .as_ref()
                            .map(|b| b.mora_position_backward),
                    ),
                    J1 => question.test(
                        &self
                            .breath_group_next
                            .as_ref()
                            .map(|b| b.accent_phrase_count),
                    ),
                    J2 => question.test(&self.breath_group_next.as_ref().map(|b| b.mora_count)),
                    K1 => question.test(&Some(self.utterance.breath_group_count)),
                    K2 => question.test(&Some(self.utterance.accent_phrase_count)),
                    K3 => question.test(&Some(self.utterance.mora_count)),
                }
            }
            Boolean(question) => {
                use BooleanPosition::*;
                match question.position {
                    E3 => {
                        question.test(&self.accent_phrase_prev.as_ref().map(|a| a.is_interrogative))
                    }
                    E5 => question.test(
                        &self
                            .accent_phrase_prev
                            .as_ref()
                            .and_then(|a| a.is_pause_insertion),
                    ),
                    F3 => {
                        question.test(&self.accent_phrase_curr.as_ref().map(|a| a.is_interrogative))
                    }
                    G3 => {
                        question.test(&self.accent_phrase_next.as_ref().map(|a| a.is_interrogative))
                    }
                    G5 => question.test(
                        &self
                            .accent_phrase_next
                            .as_ref()
                            .and_then(|a| a.is_pause_insertion),
                    ),
                }
            }
            Category(question) => {
                use CategoryPosition::*;
                match question.position {
                    B1 => question.test(&self.word_prev.as_ref().and_then(|w| w.pos)),
                    B2 => question.test(&self.word_prev.as_ref().and_then(|w| w.ctype)),
                    B3 => question.test(&self.word_prev.as_ref().and_then(|w| w.cform)),
                    C1 => question.test(&self.word_curr.as_ref().and_then(|w| w.pos)),
                    C2 => question.test(&self.word_curr.as_ref().and_then(|w| w.ctype)),
                    C3 => question.test(&self.word_curr.as_ref().and_then(|w| w.cform)),
                    D1 => question.test(&self.word_next.as_ref().and_then(|w| w.pos)),
                    D2 => question.test(&self.word_next.as_ref().and_then(|w| w.ctype)),
                    D3 => question.test(&self.word_next.as_ref().and_then(|w| w.cform)),
                }
            }
            Undefined(_) => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phoneme {
    pub p2: Option<String>,
    pub p1: Option<String>,
    pub c: Option<String>,
    pub n1: Option<String>,
    pub n2: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mora {
    /// the difference between accent type and position of the current mora identity
    pub relative_accent_position: i8,
    /// position of the current mora identity in the current accent phrase (forward)
    pub position_forward: u8,
    /// position of the current mora identity in the current accent phrase (backward)
    pub position_backward: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    /// pos (part-of-speech) of the word
    pub pos: Option<u8>,
    /// conjugation type of the word
    pub ctype: Option<u8>,
    /// inflected forms of the word
    pub cform: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccentPhraseCurrent {
    /// the number of moras in the current accent phrase
    pub mora_count: u8,
    /// accent type in the current accent phrase
    pub accent_position: u8,
    /// whether the current accent phrase interrogative or not
    pub is_interrogative: bool,
    /// position of the current accent phrase identity in the current breath group by the accent phrase (forward)
    pub accent_phrase_position_forward: u8,
    /// position of the current accent phrase identity in the current breath group by the accent phrase (backward)
    pub accent_phrase_position_backward: u8,
    /// position of the current accent phrase identity in the current breath group by the mora (forward)
    pub mora_position_forward: u8,
    /// position of the current accent phrase identity in the current breath group by the mora (backward)
    pub mora_position_backward: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccentPhrasePrevNext {
    /// the number of moras in the accent phrase
    pub mora_count: u8,
    /// accent type in the accent phrase
    pub accent_position: u8,
    /// whether the accent phrase interrogative or not
    pub is_interrogative: bool,
    /// whether pause insertion or not in between the accent phrase and the current accent phrase
    pub is_pause_insertion: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreathGroupCurrent {
    /// the number of accent phrases in the current breath group
    pub accent_phrase_count: u8,
    /// the number of moras in the current breath group
    pub mora_count: u8,
    /// position of the current breath group identity by breath group (forward)
    pub breath_group_position_forward: u8,
    /// position of the current breath group identity by breath group (backward)
    pub breath_group_position_backward: u8,
    /// position of the current breath group identity by accent phrase (forward)
    pub accent_phrase_position_forward: u8,
    /// position of the current breath group identity by accent phrase (backward)
    pub accent_phrase_position_backward: u8,
    /// position of the current breath group identity by mora (forward)
    pub mora_position_forward: u8,
    /// position of the current breath group identity by mora (backward)
    pub mora_position_backward: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreathGroupPrevNext {
    /// the number of accent phrases in the breath group
    pub accent_phrase_count: u8,
    /// the number of moras in the breath group
    pub mora_count: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utterance {
    /// the number of breath groups in this utterance
    pub breath_group_count: u8,
    /// the number of accent phrases in this utterance
    pub accent_phrase_count: u8,
    /// the number of moras in this utterance
    pub mora_count: u8,
}

#[cfg(test)]
mod tests {
    use crate::{
        fullcontext_label::{Mora, Word},
        question::{question, AllQuestion, Question},
        Label,
    };

    use super::{Phoneme, Utterance};

    #[test]
    fn query() {
        // Note: this Label is created randomly, and is invalid.
        let label = Label {
            phoneme: Phoneme {
                p2: Some("b".to_string()),
                p1: Some("o".to_string()),
                c: Some("N".to_string()),
                n1: Some("s".to_string()),
                n2: Some("a".to_string()),
            },
            mora: Some(Mora {
                relative_accent_position: -6,
                position_forward: 2,
                position_backward: 8,
            }),
            word_prev: None,
            word_curr: Some(Word {
                pos: Some(1),
                ctype: None,
                cform: None,
            }),
            word_next: None,
            accent_phrase_prev: None,
            accent_phrase_curr: None,
            accent_phrase_next: None,
            breath_group_prev: None,
            breath_group_curr: None,
            breath_group_next: None,
            utterance: Utterance {
                breath_group_count: 3,
                accent_phrase_count: 6,
                mora_count: 10,
            },
        };

        assert_eq!(
            label.satisfies(&question(&["*=i/A:*".to_string()]).unwrap()),
            false
        );

        assert_eq!(
            label.satisfies(&question(&["*/A:-??+*".to_string(), "*/A:-9+*".to_string()]).unwrap()),
            false
        );
        assert_eq!(
            label.satisfies(&question(&["*/A:-6+*".to_string()]).unwrap()),
            true
        );

        assert_eq!(
            label.satisfies(&question(&["*+8/B:*".to_string()]).unwrap()),
            true
        );

        assert_eq!(
            label.satisfies(&question(&["*-xx_*".to_string()]).unwrap()),
            true
        );
        assert_eq!(
            label.satisfies(&question(&["*/C:01_*".to_string()]).unwrap()),
            true
        );
    }

    #[test]
    fn all_query() {
        let label = Label {
            phoneme: Phoneme {
                p2: None,
                p1: None,
                c: None,
                n1: None,
                n2: None,
            },
            mora: None,
            word_prev: None,
            word_curr: None,
            word_next: None,
            accent_phrase_prev: None,
            accent_phrase_curr: None,
            accent_phrase_next: None,
            breath_group_prev: None,
            breath_group_curr: None,
            breath_group_next: None,
            utterance: Utterance {
                breath_group_count: 3,
                accent_phrase_count: 6,
                mora_count: 10,
            },
        };

        use crate::question::position::*;

        for position in [
            PhonePosition::P1,
            PhonePosition::P2,
            PhonePosition::P3,
            PhonePosition::P4,
            PhonePosition::P5,
        ] {
            assert!(label.satisfies(&AllQuestion::Phone(Question {
                position,
                range: None,
            })));
        }

        for position in [
            CategoryPosition::B1,
            CategoryPosition::B2,
            CategoryPosition::B3,
            CategoryPosition::C1,
            CategoryPosition::C2,
            CategoryPosition::C3,
            CategoryPosition::D1,
            CategoryPosition::D2,
            CategoryPosition::D3,
        ] {
            assert!(label.satisfies(&AllQuestion::Category(Question {
                position,
                range: None,
            })));
        }

        assert!(label.satisfies(&AllQuestion::SignedRange(Question {
            position: SignedRangePosition::A1,
            range: None,
        })));

        for position in [
            UnsignedRangePosition::A2,
            UnsignedRangePosition::A3,
            UnsignedRangePosition::E1,
            UnsignedRangePosition::E2,
            UnsignedRangePosition::F1,
            UnsignedRangePosition::F2,
            UnsignedRangePosition::F5,
            UnsignedRangePosition::F6,
            UnsignedRangePosition::F7,
            UnsignedRangePosition::F8,
            UnsignedRangePosition::G1,
            UnsignedRangePosition::G2,
            UnsignedRangePosition::H1,
            UnsignedRangePosition::H2,
            UnsignedRangePosition::I1,
            UnsignedRangePosition::I2,
            UnsignedRangePosition::I3,
            UnsignedRangePosition::I4,
            UnsignedRangePosition::I5,
            UnsignedRangePosition::I6,
            UnsignedRangePosition::I7,
            UnsignedRangePosition::I8,
            UnsignedRangePosition::J1,
            UnsignedRangePosition::J2,
        ] {
            assert!(label.satisfies(&AllQuestion::UnsignedRange(Question {
                position: position.clone(),
                range: None,
            })));
            assert!(!label.satisfies(&AllQuestion::UnsignedRange(Question {
                position: position.clone(),
                range: Some(1..2),
            })));
        }

        for position in [
            BooleanPosition::E3,
            BooleanPosition::E5,
            BooleanPosition::F3,
            BooleanPosition::G3,
            BooleanPosition::G5,
        ] {
            assert!(label.satisfies(&AllQuestion::Boolean(Question {
                position,
                range: None,
            })));
        }

        assert!(label.satisfies(&AllQuestion::UnsignedRange(Question {
            position: UnsignedRangePosition::K1,
            range: Some(3..4),
        })));
        assert!(label.satisfies(&AllQuestion::UnsignedRange(Question {
            position: UnsignedRangePosition::K2,
            range: Some(6..7),
        })));
        assert!(label.satisfies(&AllQuestion::UnsignedRange(Question {
            position: UnsignedRangePosition::K3,
            range: Some(5..11),
        })));

        assert!(label.satisfies(&AllQuestion::Undefined(Question {
            position: UndefinedPotision::E4,
            range: None,
        })));
    }
}
