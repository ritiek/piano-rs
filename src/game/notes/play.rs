use std::io::{BufReader, Read, Cursor};
use std::{thread, time};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Player {
    device: rodio::Device,
    samples: HashMap<String, Vec<u8>>,
}

impl Player {
    pub fn new() -> Player {
        Self::read_notes(None)
    }

    pub fn from(path: PathBuf) -> Player {
        Self::read_notes(Some(path))
    }

    fn read_notes(path: Option<PathBuf>) -> Player {
        let device = rodio::default_output_device().unwrap();
        let mut samples = HashMap::new();

        for base in &["a", "as", "b", "c", "cs", "d", "ds", "e", "f", "fs", "g", "gs"] {
            for frequency in -1..8_i8 {
                Self::read_note(*base, frequency, path.clone())
                    .map(|sample| {
                        samples.insert(format!("{}{}", base, frequency), sample);
                        Some(())
                    });
            }
        }

        if samples.len() == 0 {
            panic!("No sound assets found!")
        }

        Player {
            device,
            samples,
        }
    }

    fn get(&self, base: &str, frequency: i8) -> Option<BufReader<Cursor<Vec<u8>>>> {
        self.samples.get(&format!("{}{}", base, frequency))
            .map(|v| BufReader::new(Cursor::new(v.clone())))
    }

    pub fn play(&self, base: &str, frequency: i8, duration: time::Duration, volume: f32) {
        self.get(base, frequency)
            .map(|note| {
                let sink = rodio::play_once(&self.device, note).expect("Cannot play");
                sink.set_volume(volume);
                if duration == time::Duration::from_millis(0) {
                    sink.detach();
                } else {
                    thread::spawn(move || {
                        thread::sleep(duration);
                        sink.stop();
                    });
                }

                true
            });
    }

    fn read_note(base: &str, frequency: i8, path: Option<PathBuf>) -> Option<Vec<u8>> {
        let note_name = format!("{0}{1}.ogg", base, frequency);
        let possible_file_paths_by_preference = path.map_or_else(
            || vec![
                PathBuf::from("assets/"),
                home::home_dir().unwrap().join(".local/share/piano-rs/assets/"),
                PathBuf::from("/usr/local/share/piano-rs/assets/"),
                PathBuf::from("/usr/share/piano-rs/assets/"),
            ],
            |p| vec![p]
        );

        for directory in possible_file_paths_by_preference {
            let possible_file_path = directory.join(&note_name);
            if !possible_file_path.exists() {
                continue;
            }
            let file_path = possible_file_path;
            let content = std::fs::File::open(file_path)
                .map(|mut file| {
                    let mut data = Vec::new();
                    file.read_to_end(&mut data).unwrap();
                    data
                }).ok();
            return content;
        }
        None
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod test {
    use super::Player;
    use std::fs;

    #[test]
    fn load_sound_files() {
        let player = Player::new();
        println!("{:?}", player.samples.len());
        let asset_path = fs::read_dir("assets").unwrap();
        assert_eq!(player.samples.len(), asset_path.count());
    }

    #[test]
    fn get_note_some() {
        let player = Player::new();
        let note_sound = player.get("a", 2);
        assert!(note_sound.is_some());
    }

    #[test]
    fn get_note_none() {
        let player = Player::new();
        let note_sound = player.get("z", 9);
        assert!(note_sound.is_none());
    }

    #[test]
    fn read_note_some() {
        let note = Player::read_note("a", 2, None);
        assert!(note.is_some());
    }

    #[test]
    fn read_note_none() {
        let note = Player::read_note("z", 9, None);
        assert!(note.is_none());
    }
}

