// Here-in lies the code for cliano.
// Cliano is a terminal based minimalistic piano app with visual feedback.

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufReader, Read},
    thread,
    time::Duration,
};

use clap::{ArgMatches, Command};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

static SOUNDS_DIR: &str = "./sounds";

fn configure_cli() -> Command {
    Command::new("Cliano")
        .about("Cliano is a minimal rust based cli piano.")
        .author("Ojalla")
        .version("0.0.1")
}

struct Piano {
    audio_sink: Sink,
    output_stream: OutputStream,
    output_stream_handle: OutputStreamHandle,
    is_playing: bool,
    note_key_map: Option<HashMap<char, Vec<u8>>>,
}

impl Piano {
    fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().expect("Couldn't get audio device");
        let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");
        Self {
            output_stream: stream,
            output_stream_handle: stream_handle,
            audio_sink: sink,
            note_key_map: None,
            is_playing: false,
        }
    }

    fn run(&mut self) {
        draw_keyboard(None);
        loop {
            if poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().expect("Couldn't read key") {
                    if event.kind == crossterm::event::KeyEventKind::Press {
                        match event.code {
                            KeyCode::Char(key) => self.handle_keys(key),
                            KeyCode::Esc => break,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn load_keys(&mut self, dir_name: &str) {
        let mut sounds: HashMap<char, Vec<u8>> = HashMap::new();
        let file_map = file_to_key_map();

        for entry in fs::read_dir(dir_name).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();

            if let Some(&key) = file_map.get(filename) {
                let file = File::open(path).unwrap();
                let mut buffer = Vec::new();
                BufReader::new(file).read_to_end(&mut buffer).unwrap();
                sounds.insert(key, buffer);
            }
        }

        self.note_key_map = Some(sounds);
    }

    fn handle_keys(&mut self, key: char) {
        draw_keyboard(Some(key));

        if let Some(ref map) = self.note_key_map {
            if let Some(source) = map.get(&key) {
                let buffer = io::Cursor::new(source.clone());
                let buffer = BufReader::new(buffer);
                let source = Decoder::new(buffer)
                    .expect("Source Error")
                    .take_duration(Duration::from_secs(5));

                if self.is_playing {
                    self.audio_sink.stop();
                    self.is_playing = false;
                }

                self.audio_sink = Sink::try_new(&self.output_stream_handle).unwrap();
                self.audio_sink.append(source);
                self.is_playing = true;
            }
        }

        thread::sleep(Duration::from_millis(120));
        draw_keyboard(None);
    }
}

fn draw_keyboard(pressed: Option<char>) {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

    let keys = [
        ('a', "C "), ('w', "Db"), ('s', "D "), ('e', "Eb"), ('d', "E "),
        ('f', "F "), ('t', "Gb"), ('g', "G "), ('y', "Ab"), ('h', "A "),
        ('u', "Bb"), ('j', "B "), ('k', "C5"),
    ];

    println!("🎹 Cliano Virtual Piano 🎹");
    println!("        w   e       t   y   u      ");
    println!("        |   |       |   |   |      ");
    print!("     ");

    for (key, _) in keys.iter() {
        if Some(*key) == pressed {
            execute!(
                stdout,
                SetForegroundColor(Color::Blue),
                Print(format!("({}) ", key)),
                ResetColor
            ).unwrap();
        } else {
            print!(" {}  ", key);
        }
    }

    println!("\nPress keys to play notes. Press ESC to exit.");
}

fn file_to_key_map() -> HashMap<&'static str, char> {
    HashMap::from([
        ("C4.wav", 'a'),
        ("Cb4.wav", 'w'),
        ("D4.wav", 's'),
        ("Db4.wav", 'e'),
        ("E4.wav", 'd'),
        ("F4.wav", 'f'),
        ("Fb4.wav", 't'),
        ("G4.wav", 'g'),
        ("Gb4.wav", 'y'),
        ("A4.wav", 'h'),
        ("Ab4.wav", 'u'),
        ("B4.wav", 'j'),
        ("C5.wav", 'k'),
    ])
}

fn main() {
    let _matches: ArgMatches = configure_cli().get_matches();
    let mut app = Piano::new();
    app.load_keys(SOUNDS_DIR);
    app.run();
}
