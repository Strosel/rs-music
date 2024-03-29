use {
    crate::note::Sound,
    parse::parse,
    rodio::{OutputStream, Sink},
};

mod accidental;
mod duration;
mod envelope;
mod key;
mod note;
mod parse;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for n in parse(include_str!("../a_cruel_angels_thesis.txt")).into_iter() {
        sink.append::<Sound>(n.into());
    }

    sink.sleep_until_end();
}
