# multi-channel-tracks-remover

This script will split the notes from a MIDI will multi-channel tracks.

## How it works

All notes from each track that have a channel different than the track will be split in separate tracks using the same channel as the origin track.
The graph below visualizes exactly how notes are split:
<img src="https://github.com/MyBlackMIDIScore/midi-scripts/raw/master/multi-channel-tracks-remover/expl.svg" width="512"/>

## License

GNU General Public License v2.0
