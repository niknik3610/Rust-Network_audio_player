mod audio_player;
mod ui;

use std::{
    env::args, fmt::Debug, fs, io::{self, BufRead}, time::Duration, usize
};

use clap::Parser;
use soloud::{audio, AudioExt, Handle, LoadExt, Soloud, Wav};
use NOSHP_Client::{
    client::{ClientState, NoshpClient, Request, UserDefinedState},
    client_config::{ClientConfig, ParsedConfig},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start Program with UI, default: false 
    #[arg(short, long, default_value_t = false)]
    ui: bool,
}

fn main() {
    let args = Args::parse();

    let song_list: Vec<String> = fs::read_dir("audio_files")
        .unwrap()
        .map(|path| path.unwrap().file_name().into_string().unwrap())
        .collect();

    let mut sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();

    let mut pause_state = false;
    let mut volume = 100.0;

    let mut current_song_index = 0;
    let song_handle =
        audio_player::next_song(&mut sl, &mut wav, &song_list, &mut current_song_index);
    audio_player::toggle_pause_song(&mut sl, pause_state);

    if args.ui {
        ui::display_ui(
            &mut sl,
            &mut wav,
            &song_list,
            &mut current_song_index,
            song_handle,
            &mut pause_state,
            &mut volume,
        );
    }
    else {
        
    }
}
