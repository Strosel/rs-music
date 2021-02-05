use {
    crate::{accidental::Accidental, duration::Duration},
    rodio::source::Source,
    std::time,
};

const A4: f32 = 440.;
const TWELFTH_ROOT: f32 = 1.05946309436;
const SAMPLE_RATE: u32 = 44_100;

#[derive(Debug, Copy, Clone)]
pub enum Pitch {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<Pitch> for i32 {
    fn from(p: Pitch) -> Self {
        match p {
            Pitch::A => 9,
            Pitch::B => 11,
            Pitch::C => 0,
            Pitch::D => 2,
            Pitch::E => 4,
            Pitch::F => 5,
            Pitch::G => 7,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Note {
    Note(Pitch, i32, Accidental, Duration),
    Rest(Duration),
}

impl Note {
    fn freq(self) -> f32 {
        match self {
            Self::Note(p, oct, acc, _) => {
                let dist = (i32::from(p) + (oct - 4) * 12) + i32::from(acc) - 9;
                (A4 * TWELFTH_ROOT.powi(dist) * 100.).round() / 100.
            }
            Self::Rest(_) => 0f32,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sound(pub Note, usize, usize);

impl From<Note> for Sound {
    fn from(n: Note) -> Self {
        match n {
            Note::Note(_, _, _, d) | Note::Rest(d) => Sound(
                n,
                0,
                (time::Duration::from(d).as_secs_f32() * SAMPLE_RATE as f32) as usize,
            ),
        }
    }
}

impl Iterator for Sound {
    type Item = f32;
    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.1 = self.1.wrapping_add(1);
        if self.1 <= self.2 {
            let value = 2.0 * 3.14159265 * self.0.freq() * self.1 as f32 / SAMPLE_RATE as f32;
            Some(value.sin())
        } else {
            None
        }
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
        match self.0 {
            Note::Note(_, _, _, d) | Note::Rest(d) => Some(time::Duration::from(d)),
        }
    }
}

#[cfg(test)]
use crate::duration::DurationBuilder;

#[test]
fn test_freq() {
    let dur = DurationBuilder::from_bpm(120);
    assert_eq!(
        Note::Note(Pitch::A, 4, Accidental::Natural, dur.build(1, 0)).freq(),
        A4,
    );
    assert_eq!(
        Note::Note(Pitch::A, 4, Accidental::Sharp, dur.build(1, 0)).freq(),
        Note::Note(Pitch::B, 4, Accidental::Flat, dur.build(1, 0)).freq(),
    );
    assert_eq!(
        Note::Note(Pitch::A, 4, Accidental::Sharp, dur.build(1, 0)).freq(),
        466.16_f32,
    );
}
