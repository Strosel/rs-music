use {
    fraction::{GenericFraction, ToPrimitive},
    std::{iter::Sum, time},
};

pub type Fraction = GenericFraction<u32>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Duration {
    whole_note: time::Duration,
    pub fraction: Fraction,
}

impl Duration {
    pub fn as_secs(&self) -> f32 {
        time::Duration::from(*self).as_secs_f32()
    }
}

impl From<Duration> for time::Duration {
    fn from(d: Duration) -> Self {
        d.whole_note.mul_f32(d.fraction.to_f32().unwrap())
    }
}

impl<'a> Sum<&'a Self> for Duration {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|v| *v).sum::<Self>()
    }
}

impl Sum for Duration {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let (whole_notes, fractions) = iter
            .map(|d| (d.whole_note, d.fraction))
            .unzip::<time::Duration, Fraction, Vec<_>, Vec<_>>();
        let whole_note = whole_notes
            .into_iter()
            .reduce(|a, b| {
                if a == b {
                    a
                } else {
                    panic!("Attempted to add note durations with different time durations")
                }
            })
            .unwrap();
        Self {
            whole_note,
            fraction: fractions.into_iter().sum::<Fraction>(),
        }
    }
}

pub struct DurationBuilder(time::Duration);

impl DurationBuilder {
    pub fn new(dur: time::Duration) -> Self {
        Self(dur)
    }

    pub fn from_bpm(bpm: u32) -> Self {
        DurationBuilder(time::Duration::new(4 * 60, 0).div_f32(bpm as f32))
    }

    pub fn build(&self, nth: u32, dots: u32) -> Duration {
        Duration {
            whole_note: self.0,
            fraction: (0..=dots)
                .scan(Fraction::new(1u32, nth), |frac, _nth_dot| {
                    let old = *frac;
                    *frac = (1, frac.denom().unwrap() * 2).into();
                    Some(old)
                })
                .sum::<Fraction>(),
        }
    }
}

impl From<Duration> for DurationBuilder {
    fn from(d: Duration) -> Self {
        Self::new(d.whole_note)
    }
}

#[test]
fn test_bpm_builder() {
    let mut dur = DurationBuilder::from_bpm(120);
    assert_eq!(
        time::Duration::from(dur.build(4, 0)),
        time::Duration::from_secs_f32(0.5)
    );
    assert_eq!(
        time::Duration::from(dur.build(4, 1)),
        time::Duration::from_secs_f32(0.75)
    );
    assert_eq!(
        time::Duration::from(dur.build(1, 0)),
        time::Duration::from_secs_f32(2.)
    );
    assert_eq!(
        time::Duration::from(
            [dur.build(4, 0), dur.build(4, 1), dur.build(1, 0)]
                .iter()
                .sum::<Duration>()
        ),
        time::Duration::from_secs_f32(3.25),
    );

    dur = DurationBuilder::from_bpm(80);
    assert_eq!(
        time::Duration::from(dur.build(4, 0)),
        time::Duration::from_secs_f32(0.75)
    );
    assert_eq!(
        time::Duration::from(dur.build(1, 0)),
        time::Duration::from_secs_f32(3.)
    );
}
