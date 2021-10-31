use {
    crate::{accidental::Accidental, duration::Duration, envelope::Piano},
    rodio::source::Source,
    std::{convert::TryFrom, f32::consts::PI, ops::Sub, time},
};

type Frequency = f32;

const A4: Frequency = 440.;
const SAMPLE_RATE: u32 = 44_100;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Pitch {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Pitch {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'E' => Ok(Self::E),
            'F' => Ok(Self::F),
            'G' => Ok(Self::G),
            _ => Err("Invalid Pitch"),
        }
    }
}

macro_rules! reflex {
    ($x:pat) => {
        ($x, $x)
    };
    ($x:pat, $y:pat) => {
        ($x, $y) | ($y, $x)
    };
    ($($x:pat)|*) => {
        $(reflex!($x))|*
    }
}

impl Sub for Pitch {
    type Output = i32;

    ///the number of half steps between two pitches
    fn sub(self, rhs: Self) -> Self::Output {
        use Pitch::*;
        match (self, rhs) {
            reflex![A | B | C | D | E | F | G] => 0,
            (E, F) => 1,
            (F, E) => -1,
            (C, D) | (D, E) | (F, G) | (G, A) | (A, B) => 2,
            (D, C) | (E, D) | (G, F) | (A, G) | (B, A) => -2,
            (D, F) | (E, G) => 3,
            (F, D) | (G, E) => -3,
            (C, E) | (F, A) | (G, B) => 4,
            (E, C) | (A, F) | (B, G) => -4,
            (C, F) | (D, G) | (E, A) => 5,
            (F, C) | (G, D) | (A, E) => -5,
            (F, B) => 6,
            (B, F) => -6,
            (C, G) | (D, A) | (E, B) => 7,
            (G, C) | (A, D) | (B, E) => -7,
            (C, A) | (D, B) => 9,
            (A, C) | (B, D) => -9,
            (C, B) => 11,
            (B, C) => -11,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Note {
    Note(Pitch, i32, Accidental, Duration),
    Rest(Duration),
}

impl Note {
    pub fn duration(&self) -> Duration {
        match *self {
            Self::Note(_, _, _, d) => d,
            Self::Rest(d) => d,
        }
    }
}

impl IntoIterator for Note {
    type Item = f32;
    type IntoIter = std::iter::FromFn<Box<dyn Send + FnMut() -> Option<Self::Item>>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut sample: usize = 0;
        let sample_duration = (self.duration().as_secs() * SAMPLE_RATE as f32) as usize;
        let envelope_duration = sample_duration as f32 * 1.0;
        let freq = Frequency::from(self);
        let envelope = Piano {
            attack: time::Duration::from_millis(1).as_secs_f32() * SAMPLE_RATE as f32,
            decay: -1e-6,
            release: -1e-4,
        };

        std::iter::from_fn(Box::new(move || {
            if sample <= sample_duration {
                let value = 2.0 * PI * freq * sample as f32 / SAMPLE_RATE as f32;
                sample = sample.wrapping_add(1);
                Some(envelope.apply(value, value.sin(), envelope_duration))
            } else {
                None
            }
        }))
    }
}

impl From<Note> for Frequency {
    fn from(n: Note) -> Frequency {
        match n {
            Note::Note(p, oct, acc, _) => {
                //<https://pages.mtu.edu/~suits/NoteFreqCalcs.html>
                let n = (Pitch::A - p + (oct - 4) * 12) + i32::from(acc);
                (A4 * (2_f32).powf(n as f32 / 12.0) * 100.).round() / 100.
            }
            Note::Rest(_) => 0f32,
        }
    }
}

pub struct Sound(<Note as IntoIterator>::IntoIter, Duration);

impl From<Note> for Sound {
    fn from(note: Note) -> Self {
        let dur = note.duration();
        Self(note.into_iter(), dur)
    }
}

impl Iterator for Sound {
    type Item = <Note as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Source for Sound {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    #[inline]
    fn total_duration(&self) -> Option<time::Duration> {
        Some(self.1.into())
    }
}

#[cfg(test)]
use {
    crate::{duration::DurationBuilder, parse::parse},
    Pitch::*,
};

#[test]
fn test_freq() {
    let dur = DurationBuilder::from_bpm(120);
    assert_eq!(
        Frequency::from(Note::Note(A, 4, Accidental::Natural, dur.build(1, 0))),
        A4,
    );
    assert_eq!(
        Frequency::from(Note::Note(A, 4, Accidental::Sharp, dur.build(1, 0))),
        Frequency::from(Note::Note(B, 4, Accidental::Flat, dur.build(1, 0))),
    );
    assert_eq!(
        Frequency::from(Note::Note(A, 4, Accidental::Sharp, dur.build(1, 0))),
        466.16_f32,
    );
    for (p, f) in [
        (C, 261.63),
        (D, 293.66),
        (E, 329.63),
        (F, 349.23),
        (G, 392.00),
        (A, 440.00),
        (B, 493.88),
    ] {
        assert_eq!(
            Frequency::from(Note::Note(p, 4, Accidental::Natural, dur.build(1, 0))),
            f,
            "Scale failed at {:?}",
            p
        );
    }
}
