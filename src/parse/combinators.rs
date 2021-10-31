use {
    super::{
        error::{IResult, ParserError::*},
        pitch,
    },
    crate::{
        accidental::Accidental,
        duration::{Duration, DurationBuilder},
        key::Key,
        note::Note,
    },
    nom::{
        character::complete::{char as parse_char, i32 as parse_i32, one_of, u32 as parse_u32},
        combinator::opt,
        multi::{fold_many0, many0_count, separated_list0},
        sequence::preceded,
        Err::Failure,
    },
};

pub fn octave<'a>(default: &'a i32) -> impl 'a + FnMut(&str) -> IResult<&str, i32> {
    move |input| {
        let (input, oct) = opt(parse_i32)(input)?;
        let oct = oct.unwrap_or(*default);
        Ok((input, oct))
    }
}

pub fn accidental<'a>(
    default: &'a Accidental,
) -> impl 'a + FnMut(&str) -> IResult<&str, Accidental> {
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
            (0, 0, 0) => Ok((input, *default)),
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

pub fn duration<'a>(dur: &'a DurationBuilder) -> impl 'a + FnMut(&str) -> IResult<&str, Duration> {
    move |input| {
        if let (input, Some('/')) = opt(parse_char('/'))(input)? {
            let (input, durs) = separated_list0(parse_char('~'), |input| {
                let (input, denom) = opt(parse_u32)(input)?;
                if let Some(denom) = denom {
                    if !denom.is_power_of_two() {
                        Err(Failure(DurationError("Duration must be power of two")))
                    } else {
                        let (input, dots) = many0_count(parse_char('.'))(input)?;
                        Ok((input, dur.build(denom, dots as u32)))
                    }
                } else {
                    Ok((input, dur.build(4, 0)))
                }
            })(input)?;

            Ok((input, durs.iter().sum::<Duration>()))
        } else {
            Ok((input, dur.build(4, 0)))
        }
    }
}

pub fn note<'a>(
    key: &'a Key,
    dur: &'a DurationBuilder,
) -> impl 'a + FnMut(&str) -> IResult<&str, Note> {
    move |input| {
        let (input, pitch) = pitch(input)?;
        let (input, oct) = octave(&key.oct)(input)?;
        let (input, accidental) = accidental(key.get(&pitch))(input)?;
        let (input, duration) = duration(dur)(input)?;
        Ok((input, Note::Note(pitch, oct, accidental, duration)))
    }
}

pub fn rest<'a>(dur: &'a DurationBuilder) -> impl 'a + FnMut(&str) -> IResult<&str, Note> {
    move |input| {
        let (input, duration) = preceded(parse_char('R'), duration(dur))(input)?;
        Ok((input, Note::Rest(duration)))
    }
}
