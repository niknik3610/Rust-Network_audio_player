use std::{fmt::Debug, fs, io, time::Duration, usize};

use soloud::{audio, AudioExt, LoadExt, Soloud, Wav};
use NOSHP_Client::{
    client::{ClientState, NoshpClient, Request, UserDefinedState},
    client_config::{ClientConfig, ParsedConfig},
};

const AUDIO_FILES_PATH: &str = "audio_files/";

fn main() {
    let song_list: Vec<String> = fs::read_dir("audio_files")
        .unwrap()
        .map(|path| path.unwrap().file_name().into_string().unwrap())
        .collect();

    let mut sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();

    wav.load(&std::path::Path::new(&format!(
        "{AUDIO_FILES_PATH}rickroll.mp3"
    )))
    .unwrap();
    sl.play(&wav);
    let mut current_song_index = 0;
    next_song(&mut sl, &mut wav, &song_list, &mut current_song_index);
    println!("Now Playing: {}", song_list[current_song_index]);
    display_ui(&mut sl, &mut wav, &song_list, &mut current_song_index)
}

fn next_song(sl: &mut Soloud, wav: &mut Wav, song_list: &Vec<String>, current_song_index: &mut usize) {

    if *current_song_index >= song_list.len() - 1 {
        *current_song_index = 0;
    } else {
        *current_song_index += 1
    }
    let next_song_path_str = &format!("{AUDIO_FILES_PATH}{}", song_list[*current_song_index].clone());
    let next_song_path = &std::path::Path::new(&next_song_path_str);
    wav.load(next_song_path).expect(&format!(
        "Failed to load song with path: {}",
        next_song_path.to_str().unwrap()
    ));
    sl.play(wav);
}

fn prev_song(sl: &mut Soloud, wav: &mut Wav, song_list: &Vec<String>, current_song_index: &mut usize) {
    if *current_song_index <= 0 {
        *current_song_index = song_list.len() - 1
    } else {
        *current_song_index -= 1
    }

    let next_song_path_str = &format!("{AUDIO_FILES_PATH}{}", song_list[*current_song_index].clone());
    let next_song_path = &std::path::Path::new(&next_song_path_str);
    wav.load(next_song_path).expect(&format!(
        "Failed to load song with path: {}",
        next_song_path.to_str().unwrap()
    ));
    sl.play(wav);
}

fn display_ui(sl: &mut Soloud, wav: &mut Wav, song_list: &Vec<String>, current_song_index: &mut usize) {
    let mut buffer = String::new();
    loop {
        println!("What would you like to do?:\n1. Next Song\n2. Previous Song");

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        println!("{buffer}");
        let selection: usize = buffer.parse().unwrap();
        match selection {
            1 => next_song(sl, wav, song_list, current_song_index),
            2 => prev_song(sl, wav, song_list, current_song_index),
            _ => {
                println!("Invalid Selecetion");
                continue;
            },
        }
        
        println!("Now Playing: {}", song_list[*current_song_index]);
    }
}
