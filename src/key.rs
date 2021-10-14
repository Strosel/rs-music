use {
    crate::{accidental::Accidental, note::Pitch},
    std::{collections::HashMap, convert::TryFrom},
};

pub enum Mode {
    Major,
    Minor,
}
impl TryFrom<char> for Mode {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'M' => Ok(Self::Major),
            'm' => Ok(Self::Minor),
            _ => Err("Invalid Mode"),
        }
    }
}

pub struct Key(HashMap<Pitch, Accidental>);

impl Key {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, pitch: &Pitch) -> &Accidental {
        match self.0.get(pitch) {
            Some(acc) => acc,
            None => &Accidental::Natural,
        }
    }
}

macro_rules! key {
    ($p:pat, $m:pat) => {
        ($p, Accidental::Natural, $m)
    };

    ($p:pat, #, $m:pat) => {
        ($p, Accidental::Sharp, $m)
    };

    ($p:pat, b, $m:pat) => {
        ($p, Accidental::Flat, $m)
    };

    (# $($p:expr),+) => {
        vec![$(($p, Accidental::Sharp)),*].into_iter().collect()
    };

    (b $($p:expr),+) => {
        vec![$(($p, Accidental::Flat)),*].into_iter().collect()
    };
}

impl TryFrom<(Pitch, Accidental, Mode)> for Key {
    type Error = &'static str;

    fn try_from(key: (Pitch, Accidental, Mode)) -> Result<Self, Self::Error> {
        use {Mode::*, Pitch::*};
        match key {
            key!(C, Major) | key!(A, Minor) => Ok(Self(HashMap::new())),
            key!(G, Major) | key!(E, Minor) => Ok(Self(key![# F])),
            key!(D, Major) | key!(B, Minor) => Ok(Self(key![# F, C])),
            key!(A, Major) | key!(F, #, Minor) => Ok(Self(key![# F, C, G])),
            key!(E, Major) | key!(C, #, Minor) => Ok(Self(key![# F, C, G, D])),
            key!(B, Major) | key!(G, #, Minor) => Ok(Self(key![# F, C, G, D, A])),
            key!(F, #, Major) | key!(D, #, Minor) => Ok(Self(key![# F, C, G, D, A, E])),
            key!(C, #, Major) | key!(A, #, Minor) => Ok(Self(key![# F, C, G, D, A, E, B])),
            key!(F, Major) | key!(D, Minor) => Ok(Self(key![b B])),
            key!(B, b, Major) | key!(G, Minor) => Ok(Self(key![b B, E])),
            key!(E, b, Major) | key!(C, Minor) => Ok(Self(key![b B, E, A])),
            key!(A, b, Major) | key!(F, Minor) => Ok(Self(key![b B, E, A, D])),
            key!(D, b, Major) | key!(B, b, Minor) => Ok(Self(key![b B, E, A, D, G])),
            key!(G, b, Major) | key!(E, b, Minor) => Ok(Self(key![b B, E, A, D, G, C])),
            key!(C, b, Major) | key!(A, b, Minor) => Ok(Self(key![b B, E, A, D, G, C, F])),
            _ => Err("Invalid Key"),
        }
    }
}
