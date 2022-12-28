use midi_toolkit::{
    sequence::event::Delta,
    events::Event,
    io::{MIDIFile, MIDIWriter},
};

fn main() {
    let file = MIDIFile::open("/home/jim/Black MIDIs/MIDI Files/Z-Doc/DDC - Shining Needle Castle Sinking in the Air Revision 1.mid", None).unwrap();

    let writer = MIDIWriter::new("./out.mid", file.ppq()).unwrap();
    let mut track_writer = writer.open_track(0);

    let merged = file.iter_all_events_merged();

    for e in merged {
        let e = e.unwrap();
        let delta = e.delta;
        match e.event {
            // Set all notes to channel / track 1
            Event::NoteOn(mut ev) => {
                ev.channel = 1;
                let event = Delta::new(delta, ev);
                track_writer.write_event(event).unwrap();
            }
            Event::NoteOff(mut ev) => {
                ev.channel = 1;
                let event = Delta::new(delta, ev);
                track_writer.write_event(event).unwrap();
            }

            // Write everything else
            _ => {
                track_writer.write_event(e).unwrap();
            }
        }
    }
}
