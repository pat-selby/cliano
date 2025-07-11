# Cliano 🎹

A command-line piano application written in Rust that lets you play piano sounds using your keyboard. Transform your computer keyboard into a musical instrument!

## Features

- **Interactive Piano**: Play piano notes using your keyboard
- **Real-time Audio**: Immediate audio feedback using the Rodio audio library
- **Simple Controls**: Intuitive key mapping that mimics piano layout
- **High-quality Sounds**: Professional piano samples from the University of Iowa Electronic Music Studio

## Key Mapping

### White Keys (Natural Notes)

```
a  s  d  f  g  h  j  k
C4 D4 E4 F4 G4 A4 B4 C5
```

### Black Keys (Sharp/Flat Notes)

```
w  e     t  y  u
Cb4 Db4   Fb4 Gb4 Ab4
```

_Note: Cb4 and Fb4 samples were not available, so these keys are currently unmapped._

## Installation

### Prerequisites

- Rust and Cargo installed on your system
- Audio output device

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd cliano

# Build the project
cargo build --release

# Run the application
cargo run
```

## Usage

1. Run the application:

   ```bash
   cargo run
   ```

2. Use the mapped keys to play piano notes:

   - Press `a` to play C4
   - Press `s` to play D4
   - Press `d` to play E4
   - And so on...

3. Try playing some songs! Here are a few examples:

### Twinkle Twinkle Little Star

```
a a g g h h g
f f d d s s a
g g f f d d s
g g f f d d s
a a g g h h g
f f d d s s a
```

### Mary Had a Little Lamb

```
d s a s d d d
s s s
d g g
d s a s d d d
s s d s a
```

## Technical Details

### Architecture

- **Audio Backend**: [Rodio](https://github.com/RustAudio/rodio) for cross-platform audio playback
- **Command Line Interface**: [Clap](https://github.com/clap-rs/clap) for argument parsing
- **Audio Format**: WAV files for high-quality sound reproduction

### Project Structure

```
cliano/
├── src/
│   └── main.rs          # Main application code
├── sounds/              # Piano sound samples
│   ├── A4.wav
│   ├── Ab4.wav
│   ├── B4.wav
│   ├── Bb4.wav
│   ├── C4.wav
│   ├── C5.wav
│   ├── D4.wav
│   ├── Db4.wav
│   ├── E4.wav
│   ├── Eb4.wav
│   ├── F4.wav
│   ├── G4.wav
│   └── Gb4.wav
├── Cargo.toml           # Dependencies and project configuration
├── details.txt          # Key mappings and song examples
└── README.md
```

### Dependencies

- `rodio` - Audio playback library
- `clap` - Command line argument parser
- Standard library modules for file I/O and threading

## Audio Sources

The piano samples used in this project are sourced from the [University of Iowa Electronic Music Studio](https://theremin.music.uiowa.edu/MISpiano.html), which provides high-quality, professionally recorded piano samples for educational and personal use.

## Demo

[![Cliano Demo](https://img.youtube.com/vi/Fppt16VJ108/0.jpg)](https://youtu.be/Fppt16VJ108)

_Click the image above to watch a demonstration of Cliano in action_

### Sample Performance

Try playing these popular melodies:

1. **Twinkle Twinkle Little Star**: `a a g g h h g f f d d s s a`
2. **Mary Had a Little Lamb**: `d s a s d d d s s s d g g`
3. **Happy Birthday**: `a a s a f d` (partial)

## Contributing

Contributions are welcome! Here are some ways you can help:

- Add support for more octaves
- Implement chord support
- Add recording/playback functionality
- Create a GUI version
- Add more song examples

## License

This project is open source. Please check the audio samples' licensing terms from the University of Iowa Electronic Music Studio for commercial use.

## Troubleshooting

### Common Issues

1. **No audio output**: Ensure your system has audio drivers installed and speakers/headphones connected
2. **Build errors**: Make sure you have the latest Rust toolchain installed
3. **Missing sound files**: Verify that all WAV files are present in the `sounds/` directory

### Performance Tips

- The application loads all sound files at startup for better performance
- Use headphones for better audio quality
- Close other audio applications to avoid conflicts

## Future Enhancements

- [ ] Add volume control
- [ ] Implement sustain pedal simulation
- [ ] Add metronome functionality
- [ ] Support for MIDI input
- [ ] Visual feedback for pressed keys
- [ ] Song recording and playback
- [ ] Multiple instrument sounds

---

_Made with ❤️ and Rust_
