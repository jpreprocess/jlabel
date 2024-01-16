use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

use jlabel::{question, AllQuestion};

#[test]
fn parse_all_questions() {
    let file = File::open("tests/questions.hed").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let mut s = line.split_ascii_whitespace();
        s.next().unwrap(); // "QS"
        let name = s.next().unwrap();
        let patterns = s.next().unwrap().trim();

        let split: Vec<_> = patterns[1..patterns.len() - 1].split(',').collect();
        let question = question(&split).unwrap();
        match question {
            AllQuestion::SignedRange(r) => {
                let range = parse_range_from_name_i8(name);
                assert_eq!(r.range, range);
            }
            AllQuestion::UnsignedRange(r) => {
                let range = parse_range_from_name_u8(name);
                assert_eq!(r.range, range);
            }
            _ => {}
        }
    }
}

fn parse_range_from_name_u8(name: &str) -> Option<Range<u8>> {
    if let Some(leq) = name.find("<=") {
        let n: u8 = name[leq + 2..name.len() - 1].parse().ok()?;
        Some(1..n + 1)
    } else if let Some(eq) = name.find("=") {
        let n = name[eq + 1..name.len() - 1].parse().ok()?;
        Some(n..n + 1)
    } else {
        None
    }
}

fn parse_range_from_name_i8(name: &str) -> Option<Range<i8>> {
    if let Some(leq) = name.find("<=") {
        let n: i8 = name[leq + 2..name.len() - 1].parse().ok()?;
        Some(-99..n + 1)
    } else if let Some(eq) = name.find("=") {
        let n = name[eq + 1..name.len() - 1].parse().ok()?;
        Some(n..n + 1)
    } else {
        None
    }
}
