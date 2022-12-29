use midi_toolkit::{
    sequence::event::Delta,
    events::Event,
    io::{MIDIFile, MIDIWriter},
    pipe,
};

fn main() {
    let file = MIDIFile::open("/home/jim/Temp/test.mid", None).unwrap();

    let writer = MIDIWriter::new("./out.mid", file.ppq()).unwrap();

    let tracks = file.iter_all_tracks();

    for (n, track) in tracks.enumerate() {
        if n < 2 {
            continue;
        }
        let n = n - 2;
        let mut track_writers = Vec::new();
        for _ in 0..16 {
            track_writers.push(writer.open_next_track());
        }

        let track_ch = match n {
            0..=15 => n as u8,
            _ => (n % 16) as u8,
        };

        let mut extra_delta = 0;

        for event in track {
            let event = event.unwrap();
            let delta = event.delta;
            match event.event {
                Event::NoteOn(mut e) => {
                    let ch = e.channel as usize;
                    e.channel = track_ch;
                    let ev = Delta::new(delta + extra_delta, e);
                    track_writers[ch].write_event(ev).unwrap();
                }
                Event::NoteOff(mut e) => {
                    let ch = e.channel as usize;
                    e.channel = track_ch;
                    let ev = Delta::new(delta, e);
                    track_writers[ch].write_event(ev).unwrap();
                }
                _ => {
                    track_writers[0].write_event(event).unwrap();
                },
            }
            extra_delta += delta;
        }
    }
}
