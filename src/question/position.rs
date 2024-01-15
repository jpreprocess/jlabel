use std::ops::Range;

pub fn position(prefix: &str, suffix: &str) -> Option<AllPosition> {
    use AllPosition::*;
    use BooleanPosition::*;
    use CategoryPosition::*;
    use PhonePosition::*;
    use SignedRangePosition::*;
    use UndefinedPotision::*;
    use UnsignedRangePosition::*;

    match (prefix, suffix) {
        ("", "^*") => Some(Phone(P1)),
        ("*^", "-*") => Some(Phone(P2)),
        ("*-", "+*") => Some(Phone(P3)),
        ("*+", "=*") => Some(Phone(P4)),
        ("*=", "/A:*") => Some(Phone(P5)),

        ("*/A:", "+*") => Some(SignedRange(A1)),
        ("*+", "+*") => Some(UnsignedRange(A2)),
        ("*+", "/B:*") => Some(UnsignedRange(A3)),

        ("*/B:", "-*") => Some(Category(B1)),
        ("*-", "_*") => Some(Category(B2)),
        ("*_", "/C:*") => Some(Category(B3)),

        ("*/C:", "_*") => Some(Category(C1)),
        ("*_", "+*") => Some(Category(C2)),
        ("*+", "/D:*") => Some(Category(C3)),

        ("*/D:", "+*") => Some(Category(D1)),
        ("*+", "_*") => Some(Category(D2)),
        ("*_", "/E:*") => Some(Category(D3)),

        ("*/E:", "_*") => Some(UnsignedRange(E1)),
        ("*_", "!*") => Some(UnsignedRange(E2)),
        ("*!", "_*") => Some(Boolean(E3)),
        ("*_", "-*") => Some(Undefined(E4)),
        ("*-", "/F:*") => Some(Boolean(E5)),

        ("*/F:", "_*") => Some(UnsignedRange(F1)),
        ("*_", "#*") => Some(UnsignedRange(F2)),
        ("*#", "_*") => Some(Boolean(F3)),
        ("*_", "@*") => Some(Undefined(F4)),
        ("*@", "_*") => Some(UnsignedRange(F5)),
        ("*_", "|*") => Some(UnsignedRange(F6)),
        ("*|", "_*") => Some(UnsignedRange(F7)),
        ("*_", "/G:*") => Some(UnsignedRange(F8)),

        ("*/G:", "_*") => Some(UnsignedRange(G1)),
        ("*_", "%*") => Some(UnsignedRange(G2)),
        ("*%", "_*") => Some(Boolean(G3)),
        ("*_", "_*") => Some(Undefined(G4)),
        ("*_", "/H:*") => Some(Boolean(G5)),

        ("*/H:", "_*") => Some(UnsignedRange(H1)),
        ("*_", "/I:*") => Some(UnsignedRange(H2)),

        ("*/I:", "-*") => Some(UnsignedRange(I1)),
        ("*-", "@*") => Some(UnsignedRange(I2)),
        ("*@", "+*") => Some(UnsignedRange(I3)),
        ("*+", "&*") => Some(UnsignedRange(I4)),
        ("*&", "-*") => Some(UnsignedRange(I5)),
        ("*-", "|*") => Some(UnsignedRange(I6)),
        ("*|", "+*") => Some(UnsignedRange(I7)),
        ("*+", "/J:*") => Some(UnsignedRange(I8)),

        ("*/J:", "_*") => Some(UnsignedRange(J1)),
        ("*_", "/K:*") => Some(UnsignedRange(J2)),

        ("*/K:", "+*") => Some(UnsignedRange(K1)),
        ("*+", "-*") => Some(UnsignedRange(K2)),
        ("*-", "") => Some(UnsignedRange(K3)),

        _ => None,
    }
}

pub enum AllPosition {
    Phone(PhonePosition),
    SignedRange(SignedRangePosition),
    UnsignedRange(UnsignedRangePosition),
    Boolean(BooleanPosition),
    Category(CategoryPosition),
    Undefined(UndefinedPotision),
}

pub trait Position {
    type Target;
    type Range;

    fn range(&self, ranges: &[&String]) -> Option<Self::Range>;
    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhonePosition {
    P1,
    P2,
    P3,
    P4,
    P5,
}

impl Position for PhonePosition {
    type Target = String;
    type Range = Vec<String>;

    fn range(&self, ranges: &[&String]) -> Option<Self::Range> {
        Some(ranges.iter().map(|s| s.to_string()).collect())
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignedRangePosition {
    A1,
}

impl Position for SignedRangePosition {
    type Target = i8;
    type Range = Range<i8>;

    fn range(&self, ranges: &[&String]) -> Option<Self::Range> {
        let first = ranges.first()?;
        if ranges.len() == 1 {
            first.parse::<i8>().ok().map(|i| i..i + 1)
        } else {
            if first != &"-??" {
                return None;
            }

            let mut range: Range<i8> = -128..-10;
            for r in ranges {
                if r.ends_with('?') {
                    let (from, to) = match r.chars().next()? {
                        '-' => {
                            let from = r[1..].replace('?', "0").parse::<i8>().ok()?;
                            let to = r[1..].replace('?', "9").parse::<i8>().ok()?;
                            (from, to)
                        }
                        _ => {
                            let from = r.replace('?', "0").parse::<i8>().ok()?;
                            let to = r.replace('?', "9").parse::<i8>().ok()?;
                            (from, to)
                        }
                    };
                    if from == range.end {
                        range = range.start..to + 1;
                    } else {
                        return None;
                    }
                } else {
                    let i = r.parse::<i8>().ok()?;
                    if i == range.end {
                        range = range.start..i + 1;
                    } else {
                        return None;
                    }
                }
            }

            Some(range)
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsignedRangePosition {
    A2,
    A3,

    E1,
    E2,

    F1,
    F2,
    F5,
    F6,
    F7,
    F8,

    G1,
    G2,

    H1,
    H2,

    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,

    J1,
    J2,

    K1,
    K2,
    K3,
}

impl Position for UnsignedRangePosition {
    type Target = u8;
    type Range = Range<u8>;

    fn range(&self, ranges: &[&String]) -> Option<Self::Range> {
        let first = ranges.first()?;
        if ranges.len() == 1 {
            first.parse::<u8>().ok().map(|i| i..i + 1)
        } else {
            if first != &"???" {
                return None;
            }

            let mut range: Range<u8> = 0..10;
            for r in ranges {
                if r.ends_with('?') {
                    let from = r[1..].replace('?', "0").parse::<u8>().ok()?;
                    let to = r[1..].replace('?', "9").parse::<u8>().ok()?;
                    if from == range.end {
                        range = range.start..to + 1;
                    } else {
                        return None;
                    }
                } else {
                    let i = r.parse::<u8>().ok()?;
                    if i == range.end {
                        range = range.start..i + 1;
                    } else {
                        return None;
                    }
                }
            }

            Some(range)
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanPosition {
    E3,
    E5,

    F3,

    G3,
    G5,
}

impl Position for BooleanPosition {
    type Target = bool;
    type Range = bool;

    fn range(&self, ranges: &[&String]) -> Option<Self::Range> {
        let first = ranges.first()?;
        match first.as_str() {
            "0" => Some(false),
            "1" => Some(true),
            _ => None,
        }
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range == target
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CategoryPosition {
    B1,
    B2,
    B3,
    C1,
    C2,
    C3,
    D1,
    D2,
    D3,
}

impl Position for CategoryPosition {
    type Target = u8;
    type Range = Vec<u8>;

    fn range(&self, ranges: &[&String]) -> Option<Self::Range> {
        let mut range = Vec::new();
        for r in ranges {
            let i = r.parse::<u8>().ok()?;
            range.push(i);
        }
        Some(range)
    }

    fn test(&self, range: &Self::Range, target: &Self::Target) -> bool {
        range.contains(target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UndefinedPotision {
    E4,
    F4,
    G4,
}

impl Position for UndefinedPotision {
    type Target = ();
    type Range = ();

    fn range(&self, _: &[&String]) -> Option<Self::Range> {
        Some(())
    }

    fn test(&self, _: &Self::Range, _: &Self::Target) -> bool {
        true
    }
}
