use std::time;

//duration whole note, denom of fractional note, number of dots
#[derive(Debug, Copy, Clone)]
pub struct Duration(pub time::Duration, pub u32, pub u32);

impl From<Duration> for time::Duration {
    fn from(d: Duration) -> Self {
        let dots = d.2 as i32;
        let f = (2f64.powi(dots + 1) - 1f64) / ((d.1 as f64) * 2f64.powi(dots));
        d.0.mul_f64(f)
    }
}

pub struct DurationBuilder(pub time::Duration);

impl DurationBuilder {
    pub fn from_bpm(bpm: u32) -> Self {
        DurationBuilder(time::Duration::new(4 * 60, 0).div_f64(bpm as f64))
    }

    pub fn build(&self, denom: u32, dots: u32) -> Duration {
        Duration(self.0, denom, dots)
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
