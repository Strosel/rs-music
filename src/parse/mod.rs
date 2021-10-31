use {
    crate::{
        accidental::Accidental,
        duration::{Duration, DurationBuilder, Fraction},
        key::{Key, Mode},
        note::{Note, Pitch},
    },
    combinators::{accidental, note, octave, rest},
    error::IResult,
    nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char as parse_char, one_of, u32 as parse_u32},
        combinator::{all_consuming, map_res, opt},
        sequence::{preceded, separated_pair, tuple},
    },
    std::convert::TryFrom,
};

mod combinators;
mod error;

pub fn pitch(input: &str) -> IResult<&str, Pitch> {
    map_res(one_of("ABCDEFG"), Pitch::try_from)(input)
}

pub fn parse_key(input: &str) -> IResult<&str, Key> {
    let mode = map_res(opt(one_of("Mm")), |opt| Mode::try_from(opt.unwrap_or('M')));

    let (input, key) = map_res(
        preceded(
            tag("K:"),
            tuple((pitch, octave(&4), accidental(&Accidental::Natural), mode)),
        ),
        Key::try_from,
    )(input)?;

    Ok((input, key))
}

pub fn bar_line(input: &str) -> IResult<&str, char> {
    parse_char('|')(input)
}

pub fn bpm(input: &str) -> IResult<&str, u32> {
    preceded(tag("BPM:"), parse_u32)(input)
}

pub fn parse_measure(input: &str) -> IResult<&str, Fraction> {
    let (input, f) = separated_pair(parse_u32, parse_char('/'), parse_u32)(input)?;
    Ok((input, f.into()))
}

enum Valid {
    Valid,
    Invalid(Fraction),
}

fn validate_measure(measure: &(Fraction, Vec<Note>)) -> Valid {
    if !measure.1.is_empty() {
        let f = measure
            .1
            .iter()
            .map(|&v| v.duration())
            .sum::<Duration>()
            .fraction;
        if f == measure.0 {
            Valid::Valid
        } else {
            Valid::Invalid(f)
        }
    } else {
        Valid::Valid
    }
}

macro_rules! validate_measure {
    ($out:ident, $measure:ident) => {
        if let Valid::Invalid(act) = validate_measure(&$measure) {
            panic!(
                "Invalid measure no. {}. {} ≠ {}",
                $out.len(),
                act,
                $measure.0
            );
        }
        if !$measure.1.is_empty() {
            $out.push($measure.1);
            $measure.1 = vec![];
        }
    };
}

pub fn parse(txt: &str) -> Vec<Note> {
    let mut out = Vec::new();
    let mut measure = (Fraction::new(4u32, 4u32), Vec::new());

    let mut key = Key::default();
    let mut dur = DurationBuilder::from_bpm(120);

    for token in txt.split_whitespace() {
        if let Ok(("", note)) = all_consuming(alt((note(&key, &dur), rest(&dur))))(token) {
            measure.1.push(note);
            continue;
        }
        if let Ok(("", new_key)) = all_consuming(parse_key)(token) {
            key = new_key;
            continue;
        }
        if let Ok(("", '|')) = all_consuming(bar_line)(token) {
            validate_measure!(out, measure);
            continue;
        }
        if let Ok(("", bpm)) = all_consuming(bpm)(token) {
            validate_measure!(out, measure);
            dur = DurationBuilder::from_bpm(bpm);
            continue;
        }
        if let Ok(("", signature)) = all_consuming(parse_measure)(token) {
            validate_measure!(out, measure);
            measure.0 = signature;
            continue;
        }

        panic!("Invalid token: `{}`", token);
    }

    validate_measure!(out, measure);
    out.into_iter().flatten().collect()
}

#[test]
fn test_parse() {
    parse(include_str!("../../a_cruel_angels_thesis.txt"));
}
