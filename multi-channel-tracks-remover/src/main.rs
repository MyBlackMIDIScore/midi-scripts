use std::io;
use std::io::BufRead;

fn main() {
    // Read MIDI path
    let input = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut output = input.clone();
    output.push_str(".chrm.mid");

    // Load the MIDI file from the given path
    let midi = MIDIFile::open_in_ram(input, None).unwrap();
    let writer = MIDIWriter::new(output, midi.ppq()).unwrap();

    // Collect events
    let converted = file.iter_all_tracks();
    let merged = pipe!(converted|>to_vec()|>merge_events_array()|>unwrap_items());

    println!("{}||{}", input, output);
}
