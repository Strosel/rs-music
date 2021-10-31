pub struct Piano {
    pub attack: f32,
    pub decay: f32,
    pub release: f32,
}

impl Piano {
    pub fn apply(&self, t: f32, amp: f32, duration: f32) -> f32 {
        amp * [
            self.attack * t,
            self.decay * (t - 1.0 / self.attack) + 1.0,
            self.release * (t - duration),
        ]
        .iter()
        .map(|&v| v.clamp(0.0, 0.5))
        .fold(f32::MAX, |f, v| if f < v { f } else { v })
    }
}
