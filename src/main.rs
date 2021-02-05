use {
    crate::{duration::DurationBuilder, note::Sound},
    parse::parse,
    rodio::{OutputStream, Sink},
    std::fs,
};

mod accidental;
mod duration;
mod note;
mod parse;

fn main() {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    //A B E b
    for n in parse(
        &fs::read_to_string("a_cruel_angels_thesis.txt").unwrap(),
        DurationBuilder::from_bpm(80).build(4, 0),
    )
    .into_iter()
    {
        sink.append::<Sound>(n.into());
    }

    sink.sleep_until_end();
}
