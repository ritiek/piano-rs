use std::io::{BufReader, Read, Cursor};
use std::{thread, time};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Player {
    device: rodio::Device,
    samples: HashMap<String, Vec<u8>>,
}

impl Player {
    pub fn new() -> Player {
        let device = rodio::default_output_device().unwrap();
        let mut samples = HashMap::new();

        for base in &["a", "as", "b", "c", "cs", "d", "ds", "e", "f", "fs", "g", "gs"] {
            for frequency in -1..8_i8 {
                Self::read_note(*base, frequency)
                    .map(|sample| {
                        samples.insert(format!("{}{}", base, frequency), sample);
                        Some(())
                    });
            }
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

    fn read_note(base: &str, frequency: i8) -> Option<Vec<u8>> {
        let file_path = format!("assets/{0}{1}.ogg", base, frequency);
        std::fs::File::open(file_path)
            .map(|mut file| {
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();
                data
            }).ok()
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
        let note = Player::read_note("a", 2);
        assert!(note.is_some());
    }

    #[test]
    fn read_note_none() {
        let note = Player::read_note("z", 9);
        assert!(note.is_none());
    }
}

