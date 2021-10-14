use std::time;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Duration {
    whole_note: time::Duration,
    pub nth: u32,
    pub dots: u32,
}

impl Duration {
    pub fn as_secs(&self) -> f32 {
        time::Duration::from(*self).as_secs_f32()
    }
}

impl From<Duration> for time::Duration {
    fn from(d: Duration) -> Self {
        let dots = d.dots as i32;
        let f = (2f64.powi(dots + 1) - 1f64) / ((d.nth as f64) * 2f64.powi(dots));
        d.whole_note.mul_f64(f)
    }
}

pub struct DurationBuilder(time::Duration);

impl DurationBuilder {
    pub fn new(dur: time::Duration) -> Self {
        Self(dur)
    }

    pub fn from_bpm(bpm: u32) -> Self {
        DurationBuilder(time::Duration::new(4 * 60, 0).div_f64(bpm as f64))
    }

    pub fn build(&self, nth: u32, dots: u32) -> Duration {
        Duration {
            whole_note: self.0,
            nth,
            dots,
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
        time::Duration::from(dur.build(1, 0)),
        time::Duration::from_secs_f32(2.)
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
