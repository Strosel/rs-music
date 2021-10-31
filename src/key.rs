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

pub struct Key {
    pub oct: i32,
    map: HashMap<Pitch, Accidental>,
}

impl Key {
    pub fn get(&self, pitch: &Pitch) -> &Accidental {
        match self.map.get(pitch) {
            Some(acc) => acc,
            None => &Accidental::Natural,
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            oct: 4,
            map: HashMap::new(),
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

impl TryFrom<(Pitch, i32, Accidental, Mode)> for Key {
    type Error = &'static str;

    fn try_from(key: (Pitch, i32, Accidental, Mode)) -> Result<Self, Self::Error> {
        use {Mode::*, Pitch::*};
        let (key, oct) = ((key.0, key.2, key.3), key.1);
        Ok(Self {
            oct,
            map: match key {
                key!(C, Major) | key!(A, Minor) => HashMap::new(),
                key!(G, Major) | key!(E, Minor) => key![# F],
                key!(D, Major) | key!(B, Minor) => key![# F, C],
                key!(A, Major) | key!(F, #, Minor) => key![# F, C, G],
                key!(E, Major) | key!(C, #, Minor) => key![# F, C, G, D],
                key!(B, Major) | key!(G, #, Minor) => key![# F, C, G, D, A],
                key!(F, #, Major) | key!(D, #, Minor) => key![# F, C, G, D, A, E],
                key!(C, #, Major) | key!(A, #, Minor) => key![# F, C, G, D, A, E, B],
                key!(F, Major) | key!(D, Minor) => key![b B],
                key!(B, b, Major) | key!(G, Minor) => key![b B, E],
                key!(E, b, Major) | key!(C, Minor) => key![b B, E, A],
                key!(A, b, Major) | key!(F, Minor) => key![b B, E, A, D],
                key!(D, b, Major) | key!(B, b, Minor) => key![b B, E, A, D, G],
                key!(G, b, Major) | key!(E, b, Minor) => key![b B, E, A, D, G, C],
                key!(C, b, Major) | key!(A, b, Minor) => key![b B, E, A, D, G, C, F],
                _ => Err("Invalid Key")?,
            },
        })
    }
}
