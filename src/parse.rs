use {
    crate::{
        accidental::Accidental,
        duration::{Duration, DurationBuilder},
        note::{Note, Pitch},
    },
    regex::Regex,
};

fn count_char(s: &str, c: &str) -> usize {
    s.matches(c).count()
}

pub fn parse(txt: &str, default: Duration) -> Vec<Note> {
    let dur = DurationBuilder(default.0);
    let mut out = Vec::new();
    let mut oct = 4;
    let re = Regex::new(r"\s*((?P<oct>\d+)|(?P<note>[\^_\#b\+-\.~]*[A-GR]))\s*").unwrap();

    for s in re.captures_iter(txt) {
        match s.name("oct") {
            Some(o) => oct = o.as_str().parse().unwrap(),
            None => {}
        }
        match s.name("note") {
            Some(n) => {
                let c = n.as_str();
                let note_oct = oct + count_char(c, "^") as i32 - count_char(c, "_") as i32;
                let note_acc = Accidental::NSharp(count_char(c, "#") as i32)
                    + Accidental::NFlat(count_char(c, "b") as i32);
                let note_dur = dur.build(
                    {
                        let diff = count_char(c, "+") as i32 - count_char(c, "-") as i32;
                        if diff > 0 {
                            default.1 / 2u32.pow(diff as u32)
                        } else {
                            default.1 * 2u32.pow(diff.abs() as u32)
                        }
                    },
                    default.2 + count_char(c, ".") as u32,
                );
                out.push(match c.chars().last() {
                    Some(k) => match k {
                        'A' => Note::Note(Pitch::A, note_oct, note_acc, note_dur),
                        'B' => Note::Note(Pitch::B, note_oct, note_acc, note_dur),
                        'C' => Note::Note(Pitch::C, note_oct, note_acc, note_dur),
                        'D' => Note::Note(Pitch::D, note_oct, note_acc, note_dur),
                        'E' => Note::Note(Pitch::E, note_oct, note_acc, note_dur),
                        'F' => Note::Note(Pitch::F, note_oct, note_acc, note_dur),
                        'G' => Note::Note(Pitch::G, note_oct, note_acc, note_dur),
                        'R' => Note::Rest(note_dur),
                        _ => panic!("Parser error"),
                    },
                    None => panic!("Parser error"),
                });
            }
            None => {}
        }
    }

    out
}
