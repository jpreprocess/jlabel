use crate::{fixture::fixtures, fullcontext_label::Label};

#[test]
fn test_parse() {
    for (input, expected) in fixtures() {
        let actual = input.parse::<Label>().unwrap();
        assert_eq!(actual, expected);
    }
}
