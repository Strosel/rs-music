use std::ops::{Add, AddAssign};

#[derive(Clone, Copy)]
pub enum Accidental {
    NFlat(i32),
    Flat,
    Natural,
    Sharp,
    NSharp(i32),
}

impl Accidental {
    pub fn i32(self) -> i32 {
        match self {
            Self::NFlat(n) => -(n.abs()),
            Self::Flat => -1,
            Self::Natural => 0,
            Self::Sharp => 1,
            Self::NSharp(n) => n.abs(),
        }
    }
}

impl Add for Accidental {
    type Output = Accidental;

    fn add(self, rhs: Accidental) -> Self::Output {
        match self.i32() + rhs.i32() {
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
