use soloud::{AudioExt, LoadExt, Soloud};
use std::{fs, sync::mpsc::Receiver};

pub const AUDIO_FILES_PATH: &str = "audio_files/";

pub enum AudioPlayerCommands {
    TogglePause,
    NextSong,
    PrevSong,
    SetVol(f32),
    Quit,
}

pub struct AudioPlayer {
    receiver_channel: Receiver<AudioPlayerCommands>,
    sl: Soloud,
    wav: soloud::Wav,
    song_list: Vec<String>,
    current_song_index: usize,
    pause_state: bool,
    vol_state: f32,
    channel_handle: soloud::Handle,
}

impl AudioPlayer {
    pub fn init(receiver_channel: Receiver<AudioPlayerCommands>) -> Self {
        let song_list: Vec<String> = fs::read_dir("audio_files")
            .unwrap()
            .map(|path| path.unwrap().file_name().into_string().unwrap())
            .collect();

        let sl = Soloud::default().unwrap();
        let wav = soloud::audio::Wav::default();

        let pause_state = false;
        let volume = 100.0;

        let current_song_index = 0;
        return Self {
            receiver_channel,
            channel_handle: soloud::Handle::PRIMARY,
            current_song_index,
            pause_state,
            sl,
            song_list,
            vol_state: volume,
            wav,
        };
    }
    pub async fn run(mut self) {
        self.channel_handle = self.next_song();
        self.set_volume(0.50);

        loop {
            //This should block
            let event = self.receiver_channel.recv();

            let event = match event {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{e}");
                    continue;
                }
            };

            match event {
                AudioPlayerCommands::TogglePause => self.toggle_pause_song(),
                AudioPlayerCommands::NextSong => self.channel_handle = self.next_song(),
                AudioPlayerCommands::PrevSong => self.channel_handle = self.prev_song(),
                AudioPlayerCommands::SetVol(v) => self.set_volume(v),
                AudioPlayerCommands::Quit => break,
            }
        }

        println!("Exiting");
    }
    pub fn next_song(&mut self) -> soloud::Handle {
        if self.current_song_index >= self.song_list.len() - 1 {
            self.current_song_index = 0;
        } else {
            self.current_song_index += 1
        }
        let next_song_path_str = &format!(
            "{AUDIO_FILES_PATH}{}",
            self.song_list[self.current_song_index].clone()
        );
        let next_song_path = &std::path::Path::new(&next_song_path_str);
        self.wav.load(next_song_path).expect(&format!(
            "Failed to load song with path: {}",
            next_song_path.to_str().unwrap()
        ));

        println!("Now Playing: {}", self.song_list[self.current_song_index]);
        return self.sl.play(&self.wav);
    }

    pub fn prev_song(&mut self) -> soloud::Handle {
        if self.current_song_index <= 0 {
            self.current_song_index = self.song_list.len() - 1
        } else {
            self.current_song_index -= 1
        }

        let next_song_path_str = &format!(
            "{AUDIO_FILES_PATH}{}",
            self.song_list[self.current_song_index].clone()
        );
        let next_song_path = &std::path::Path::new(&next_song_path_str);
        self.wav.load(next_song_path).expect(&format!(
            "Failed to load song with path: {}",
            next_song_path.to_str().unwrap()
        ));

        println!("Now Playing: {}", self.song_list[self.current_song_index]);
        return self.sl.play(&self.wav);
    }

    pub fn toggle_pause_song(&mut self) {
        self.sl.set_pause_all(!self.pause_state);
        self.pause_state = !self.pause_state;
    }

    pub fn set_volume(&mut self, new_vol: f32) {
        self.sl.set_volume(self.channel_handle, new_vol);
    }
}
