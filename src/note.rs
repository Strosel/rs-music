use {
    crate::{accidental::Accidental, duration::Duration},
    std::time,
};

const A4: f64 = 440.;
const twelfth_root: f64 = 1.05946309436;

pub struct Note(pub char, pub i32, pub Accidental, pub Duration);

impl Note {
    pub fn freq(self) -> f64 {
        let hs: i32 = match self.0 {
            'a' | 'A' => 9,
            'b' | 'B' => 11,
            'c' | 'C' => 0,
            'd' | 'D' => 2,
            'e' | 'E' => 4,
            'f' | 'F' => 5,
            'g' | 'G' => 7,
            'r' | 'R' => return 0.0,
            _ => panic!(format!("Impossible note {}", self.0)),
        };

        let dist = (hs + (self.1 - 4) * 12) + self.2.i32();

        (A4 * twelfth_root.powi(dist - 9) * 100.).round() / 100.
    }

    pub fn duration(self, meter: time::Duration) -> time::Duration {
        self.3.duration(meter)
    }
}

pub fn parse(txt: Vec<char>) -> Result<Vec<Note>, String> {}

#[test]
fn test_freq() {
    assert_eq!(Note('A', 4, Accidental::Natural, Duration(1, 0)).freq(), A4,);
    assert_eq!(
        Note('A', 4, Accidental::Sharp, Duration(1, 0)).freq(),
        Note('B', 4, Accidental::Flat, Duration(1, 0)).freq(),
    );
    assert_eq!(
        Note('A', 4, Accidental::Sharp, Duration(1, 0)).freq(),
        466.16_f64,
    );
}
