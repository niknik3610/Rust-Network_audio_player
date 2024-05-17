use soloud::{LoadExt, Soloud, Wav};

pub const AUDIO_FILES_PATH: &str = "audio_files/";
pub fn next_song(
    sl: &mut Soloud,
    wav: &mut soloud::Wav,
    song_list: &Vec<String>,
    current_song_index: &mut usize,
) -> soloud::Handle {
    if *current_song_index >= song_list.len() - 1 {
        *current_song_index = 0;
    } else {
        *current_song_index += 1
    }
    let next_song_path_str = &format!(
        "{AUDIO_FILES_PATH}{}",
        song_list[*current_song_index].clone()
    );
    let next_song_path = &std::path::Path::new(&next_song_path_str);
    wav.load(next_song_path).expect(&format!(
        "Failed to load song with path: {}",
        next_song_path.to_str().unwrap()
    ));

    println!("Now Playing: {}", song_list[*current_song_index]);
    return sl.play(wav);
}

pub fn prev_song(
    sl: &mut Soloud,
    wav: &mut Wav,
    song_list: &Vec<String>,
    current_song_index: &mut usize,
) -> soloud::Handle {
    if *current_song_index <= 0 {
        *current_song_index = song_list.len() - 1
    } else {
        *current_song_index -= 1
    }

    let next_song_path_str = &format!(
        "{AUDIO_FILES_PATH}{}",
        song_list[*current_song_index].clone()
    );
    let next_song_path = &std::path::Path::new(&next_song_path_str);
    wav.load(next_song_path).expect(&format!(
        "Failed to load song with path: {}",
        next_song_path.to_str().unwrap()
    ));

    println!("Now Playing: {}", song_list[*current_song_index]);
    return sl.play(wav);
}

pub fn toggle_pause_song(sl: &mut Soloud, pause_state: bool) -> bool {
    sl.set_pause_all(!pause_state);
    return !pause_state;
}

pub fn set_volume(sl: &mut Soloud, channel_handle: soloud::Handle, new_vol: f32) {
    sl.set_volume(channel_handle, new_vol);
}
