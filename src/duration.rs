use std::time;

//denom of fraction of meter, number of dots
pub struct Duration(pub u32, pub u32);

impl Duration {
    pub fn duration(self, meter: time::Duration) -> time::Duration {
        let n = self.1 as i32;
        let f = (2f64.powi(n + 1) - 1f64) / ((self.0 as f64) * 2f64.powi(n));
        meter.mul_f64(f)
    }
}
