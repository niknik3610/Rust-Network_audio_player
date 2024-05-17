use soloud::{Soloud, Wav};
use std::io;

use crate::audio_player;

pub fn set_volume_ui(sl: &mut Soloud, channel_handle: soloud::Handle, volume: &mut f32) {
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
    audio_player::set_volume(sl, channel_handle, selection);
}

pub fn display_ui(
    sl: &mut Soloud,
    wav: &mut Wav,
    song_list: &Vec<String>,
    current_song_index: &mut usize,
    mut channel_handle: soloud::Handle,
    pause_state: &mut bool,
    volume: &mut f32,
) {
    let mut buffer = String::new();
    loop {
        println!("What would you like to do?:\n1. Next Song\n2. Previous Song\n3. Toggle Pause Current Song\n4. Set Volume");

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        println!("{buffer}");
        let selection: usize = buffer.parse().unwrap();
        match selection {
            1 => channel_handle = audio_player::next_song(sl, wav, song_list, current_song_index),
            2 => channel_handle = audio_player::prev_song(sl, wav, song_list, current_song_index),
            3 => *pause_state = audio_player::toggle_pause_song(sl, *pause_state),
            4 => set_volume_ui(sl, channel_handle, volume),
            _ => {
                println!("Invalid Selection");
                continue;
            }
        }
    }
}
