use crate::{
    question::{
        position::{
            BooleanPosition, CategoryPosition, PhonePosition, SignedRangePosition,
            UndefinedPotision, UnsignedRangePosition,
        },
        question, AllQuestion, Question,
    },
    Label, Mora, Phoneme, Utterance, Word,
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

    assert!(!question(&["*=i/A:*"]).unwrap().test(&label));

    assert!(!question(&["*/A:-??+*", "*/A:-9+*"]).unwrap().test(&label));
    assert!(question(&["*/A:-6+*"]).unwrap().test(&label));

    assert!(question(&["*+8/B:*"]).unwrap().test(&label));

    assert!(question(&["*-xx_*"]).unwrap().test(&label));
    assert!(question(&["*/C:01_*"]).unwrap().test(&label));
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
        let q = AllQuestion::Phone(Question {
            position,
            range: None,
        });
        assert!(q.test(&label));
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
        let q = Question {
            position,
            range: None,
        };
        assert!(q.test(&label));
    }

    let q = Question {
        position: SignedRangePosition::A1,
        range: None,
    };
    assert!(q.test(&label));

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
        let q = Question {
            position: position.clone(),
            range: None,
        };
        assert!(q.test(&label));
        let q = Question {
            position: position.clone(),
            range: Some(0..1),
        };
        assert!(!q.test(&label));
    }

    for position in [
        BooleanPosition::E3,
        BooleanPosition::E5,
        BooleanPosition::F3,
        BooleanPosition::G3,
        BooleanPosition::G5,
    ] {
        let q = Question {
            position,
            range: None,
        };
        assert!(q.test(&label));
    }

    let q = Question {
        position: UnsignedRangePosition::K1,
        range: Some(3..4),
    };
    assert!(q.test(&label));
    let q = Question {
        position: UnsignedRangePosition::K2,
        range: Some(6..7),
    };
    assert!(q.test(&label));
    let q = Question {
        position: UnsignedRangePosition::K3,
        range: Some(5..11),
    };
    assert!(q.test(&label));

    let q = Question {
        position: UndefinedPotision::E4,
        range: None,
    };
    assert!(q.test(&label));
}
