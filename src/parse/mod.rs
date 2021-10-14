use {
    crate::{
        accidental::Accidental,
        duration::{Duration, DurationBuilder},
        key::{Key, Mode},
        note::{Note, Pitch},
    },
    error::{IResult, ParserError::*},
    nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char as parse_char, i32 as parse_i32, one_of, u32 as parse_u32},
        combinator::{all_consuming, map_res, opt},
        multi::{fold_many0, many0_count},
        sequence::{preceded, tuple},
        Err::Failure,
    },
    std::convert::TryFrom,
};

mod error;

fn octave<'a>(default: &'a i32) -> impl 'a + FnMut(&str) -> IResult<&str, i32> {
    move |input| {
        let (input, oct) = opt(parse_i32)(input)?;
        let oct = oct.unwrap_or(*default);
        Ok((input, oct))
    }
}

fn accidental<'a>(default: &'a Accidental) -> impl 'a + FnMut(&str) -> IResult<&str, Accidental> {
    use Accidental::*;

    move |input| {
        let (input, acc) = fold_many0(
            one_of("#bn"),
            || (0, 0, 0),
            |mut acc, c| {
                match c {
                    '#' => acc.0 += 1,
                    'b' => acc.1 += 1,
                    'n' => acc.2 += 1,
                    _ => unreachable!(),
                }
                acc
            },
        )(input)?;

        match acc {
            (0, 0, 0) => Ok((input, *default)), //TODO default accidental
            (1, 0, 0) => Ok((input, Sharp)),
            (n @ 2.., 0, 0) => Ok((input, NSharp(n as i32))),
            (0, 1, 0) => Ok((input, Flat)),
            (0, n @ 2.., 0) => Ok((input, NFlat(n as i32))),
            (0, 0, 1) => Ok((input, Natural)),
            (0, 0, 2..) => Err(Failure(AccidentalError("Multiple n Accidentals"))),
            _ => Err(Failure(AccidentalError("Mixed Accidentals"))),
        }
    }
}

fn duration<'a>(dur: &'a DurationBuilder) -> impl 'a + FnMut(&str) -> IResult<&str, Duration> {
    move |input| {
        let (input, denom) = opt(preceded(parse_char('/'), parse_u32))(input)?;
        let denom = denom.unwrap_or(4); //TODO default denom
        if !denom.is_power_of_two() {
            Err(Failure(DurationError("Duration must be power of two")))
        } else {
            let (input, dots) = many0_count(parse_char('.'))(input)?;
            Ok((input, dur.build(denom, dots as u32)))
        }
    }
}

fn note<'a>(
    oct: &'a i32,
    key: &'a Key,
    dur: &'a DurationBuilder,
) -> impl 'a + FnMut(&str) -> IResult<&str, Note> {
    move |input| {
        let (input, pitch) = pitch(input)?;
        let (input, oct) = octave(oct)(input)?;
        let (input, accidental) = accidental(key.get(&pitch))(input)?;
        let (input, duration) = duration(dur)(input)?;
        Ok((input, Note::Note(pitch, oct, accidental, duration)))
    }
}

fn pitch(input: &str) -> IResult<&str, Pitch> {
    map_res(one_of("ABCDEFG"), Pitch::try_from)(input)
}

fn rest<'a>(dur: &'a DurationBuilder) -> impl 'a + FnMut(&str) -> IResult<&str, Note> {
    move |input| {
        let (input, duration) = preceded(parse_char('R'), duration(dur))(input)?;
        Ok((input, Note::Rest(duration)))
    }
}

fn parse_key(input: &str) -> IResult<&str, Key> {
    let mode = map_res(opt(one_of("Mm")), |opt| Mode::try_from(opt.unwrap_or('M')));

    let (input, key) = map_res(
        preceded(
            tag("K:"),
            tuple((pitch, accidental(&Accidental::Natural), mode)),
        ),
        Key::try_from,
    )(input)?;

    Ok((input, key))
}

//TODO new parser format (maybe use nom?? (or other crate))
//[Pitch][Oct][Acc][/Denom][Dots]
//where Acc, Oct, and Denom could be implied.
//Pitch is one of [A,B,C,D,E,F,G,R]
//Oct is a number 0-? or some number of ^ or _ mixing is not allowed
//Acc is some number of # or b or a single n mixing is not allowed
//Denom must always be a power of 2
//Dots is a string of .
//
//[nom]/[denom]=[bmp] to set bpm
//nom=1 can be implied
//
//[nom]/[denom] for staff length (term?)
//nom=nom can be implied
//
//always check for correct staff count before allowing bpm or staff len to change
//K:[Key][M|m]
//M is major, impled by default
//m is minor
//valid keys:
//[
// C[M] = Am
// G[M] = Em
// D[M] = Bm
// A[M] = F#m
// E[M] = C#m
// B[M] = G#m
// Gb[M] = F#[M] = Ebm = D#m
// Db[M] = Bbm
// Ab[M] = Fm
// Eb[M] = Cm
// Bb[M] = Gm
// F[M] = Dm
//]
//
//TODO STAFF? | check for correct duration without changing settings

pub fn parse(txt: &str, default: Duration) -> Vec<Note> {
    let mut out = Vec::new();

    //TODO introcuce encompassing struct for defaults, eg Key
    let mut oct = 4;
    let mut key = Key::new();
    let mut dur: DurationBuilder = default.into();

    for token in txt.split_whitespace() {
        if let Ok(("", note)) = all_consuming(alt((note(&oct, &key, &dur), rest(&dur))))(token) {
            out.push(note);
            continue;
        }
        if let Ok(("", new_key)) = all_consuming(parse_key)(token) {
            key = new_key;
            continue;
        }
    }
    out
}

#[test]
fn test_parse() {
    use {Accidental::*, Pitch::*};
    let dur = DurationBuilder::from_bpm(80);
    assert_eq!(
        parse(
            include_str!("../../a_cruel_angels_thesis.txt"),
            dur.build(4, 0),
        ),
        vec![
            Note::Note(C, 4, Natural, dur.build(4, 0)),
            Note::Note(E, 4, Flat, dur.build(4, 0)),
            Note::Note(F, 4, Natural, dur.build(8, 1)),
            Note::Note(E, 4, Flat, dur.build(8, 1)),
            Note::Note(F, 4, Natural, dur.build(8, 0)),
        ]
    )
}
