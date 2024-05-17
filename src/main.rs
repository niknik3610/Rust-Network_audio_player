use std::{fmt::Debug, fs, io::{self, BufRead}, time::Duration, usize};

use soloud::{audio, AudioExt, Handle, LoadExt, Soloud, Wav};
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

    let mut current_song_index = 0;
    let song_handle = next_song(&mut sl, &mut wav, &song_list, &mut current_song_index);
    display_ui(&mut sl, &mut wav, &song_list, &mut current_song_index, song_handle)
}

fn next_song(sl: &mut Soloud, wav: &mut Wav, song_list: &Vec<String>, current_song_index: &mut usize) -> soloud::Handle {
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

    println!("Now Playing: {}", song_list[*current_song_index]);
    return sl.play(wav);
}

fn prev_song(sl: &mut Soloud, wav: &mut Wav, song_list: &Vec<String>, current_song_index: &mut usize) -> soloud::Handle {
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

    println!("Now Playing: {}", song_list[*current_song_index]);
    return sl.play(wav);
}

fn toggle_pause_song(sl: &mut Soloud, pause_state: bool) -> bool{
    sl.set_pause_all(!pause_state);
    return !pause_state;
}

fn set_volume_ui(sl: &mut Soloud, channel_handle: soloud::Handle, volume: &mut f32) {
    println!("Enter the new volume: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop();
    let selection: f32 = buffer.parse().unwrap();

    if selection > 1.0 || selection < 0.0 {
        println!("yo don't blow out ur speakers, volume is between 0-1");
        return;
    }

    *volume = selection;
    set_volume(sl, channel_handle, selection);
}


fn set_volume(sl: &mut Soloud, channel_handle: soloud::Handle, new_vol: f32) {
    sl.set_volume(channel_handle, new_vol);
}

fn display_ui(sl: &mut Soloud, wav: &mut Wav, song_list: &Vec<String>, current_song_index: &mut usize, mut channel_handle: Handle) {
    let mut buffer = String::new();
    let mut pause_state = false;
    let mut volume = 100.0;
    loop {
        println!("What would you like to do?:\n1. Next Song\n2. Previous Song\n3. Toggle Pause Current Song\n4. Set Volume");

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        println!("{buffer}");
        let selection: usize = buffer.parse().unwrap();
        match selection {
            1 => channel_handle = next_song(sl, wav, song_list, current_song_index),
            2 => channel_handle = prev_song(sl, wav, song_list, current_song_index),
            3 => pause_state = toggle_pause_song(sl, pause_state),
            4 => set_volume_ui(sl, channel_handle, &mut volume),
            _ => {
                println!("Invalid Selection");
                continue;
            },
        } 

    }
}
