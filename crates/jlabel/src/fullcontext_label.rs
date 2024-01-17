/// The structure representing a single line of HTS-style full-context label.
///
/// The parser from str, and the serializer to String are both implemented.
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

/// `Phoneme` field of full-context label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phoneme {
    pub p2: Option<String>,
    pub p1: Option<String>,
    pub c: Option<String>,
    pub n1: Option<String>,
    pub n2: Option<String>,
}

/// `Mora` field of full-context label (`A` field).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mora {
    /// the difference between accent type and position of the current mora identity
    pub relative_accent_position: i8,
    /// position of the current mora identity in the current accent phrase (forward)
    pub position_forward: u8,
    /// position of the current mora identity in the current accent phrase (backward)
    pub position_backward: u8,
}

/// `Word` field of full-context label (`B`, `C`, and `D` field).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    /// pos (part-of-speech) of the word
    pub pos: Option<u8>,
    /// conjugation type of the word
    pub ctype: Option<u8>,
    /// inflected forms of the word
    pub cform: Option<u8>,
}

/// `AccentPhrase` field of full-context label for current accent phrase (`F` field).
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

/// `AccentPhrase` field of full-context label for previous or next accent phrase (`E` and `G` field).
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

/// `BreathGroup` field of full-context label for current breath group (`I` field).
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

/// `BreathGroup` field of full-context label for previous or next breath group (`H` and `J` field).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreathGroupPrevNext {
    /// the number of accent phrases in the breath group
    pub accent_phrase_count: u8,
    /// the number of moras in the breath group
    pub mora_count: u8,
}

/// `Utterance` field of full-context label (`K` field).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utterance {
    /// the number of breath groups in this utterance
    pub breath_group_count: u8,
    /// the number of accent phrases in this utterance
    pub accent_phrase_count: u8,
    /// the number of moras in this utterance
    pub mora_count: u8,
}
