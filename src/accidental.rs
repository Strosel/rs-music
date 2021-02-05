use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone)]
pub enum Accidental {
    NFlat(i32),
    Flat,
    Natural,
    Sharp,
    NSharp(i32),
}

impl From<Accidental> for i32 {
    fn from(a: Accidental) -> i32 {
        match a {
            Accidental::NFlat(n) => -(n.abs()),
            Accidental::Flat => -1,
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::NSharp(n) => n.abs(),
        }
    }
}

impl Add for Accidental {
    type Output = Accidental;

    fn add(self, rhs: Accidental) -> Self::Output {
        match i32::from(self) + i32::from(rhs) {
            -1 => Self::Flat,
            0 => Self::Natural,
            1 => Self::Sharp,
            n => {
                if n < 0 {
                    Self::NFlat(n.abs())
                } else {
                    Self::NSharp(n.abs())
                }
            }
        }
    }
}

impl AddAssign for Accidental {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
