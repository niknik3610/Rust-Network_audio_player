mod audio_player;
mod ui;

use std::{
    error::Error,
    fmt::Debug,
    fs,
    usize,
};

use clap::Parser;
use soloud::{audio, AudioExt, Soloud, Wav};
use NOSHP_Client::{
    client::{ClientState, NoshpClient, Request, UserDefinedState},
    client_config::{ClientConfig},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start Program with UI, default: false
    #[arg(short, long, default_value_t = false)]
    ui: bool,
}

struct MusicPlayerState {
    pub sl: Soloud,
    pub wav: Wav,
    pub song_list: Vec<String>,
    pub current_song_index: usize,
    pub channel_handle: soloud::Handle,
    pub pause_state: bool,
    pub volume: f32,
}
impl Default for MusicPlayerState {
    fn default() -> Self {
        Self {
            sl: Soloud::default().unwrap(),
            wav: audio::Wav::default(),
            song_list: Vec::new(),
            current_song_index: 0,
            channel_handle: soloud::Handle::PRIMARY,
            pause_state: false,
            volume: 1.0,
        }
    }
}
impl UserDefinedState for MusicPlayerState {}

const CONFIG_PATH: &str = "./example_config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    let channel_handle =
        audio_player::next_song(&mut sl, &mut wav, &song_list, &mut current_song_index);
    audio_player::toggle_pause_song(&mut sl, pause_state);

    if args.ui {
        ui::display_ui(
            &mut sl,
            &mut wav,
            &song_list,
            &mut current_song_index,
            channel_handle,
            &mut pause_state,
            &mut volume,
        );
    } else {
        let config = ClientConfig::load_config(CONFIG_PATH).unwrap();

        let state = MusicPlayerState {
            sl,
            wav,
            song_list,
            current_song_index,
            channel_handle,
            pause_state,
            volume,
        };

        let client_handler = NoshpClient::new();
        client_handler
            .set_state(state)
            .add_callback("Pause/Play", Box::new(callback_toggle_pause))
            .add_callback("Next", Box::new(callback_next))
            .add_callback("Previous", Box::new(callback_prev))
            .add_callback("Volume", Box::new(callback_vol))
            .run(config)
            .await
            .unwrap();
    }
    Ok(())
}

fn callback_toggle_pause(state: &mut ClientState<MusicPlayerState>, _req: Request) {
    state.user_state.pause_state =
        audio_player::toggle_pause_song(&mut state.user_state.sl, state.user_state.pause_state);
}

fn callback_next(state: &mut ClientState<MusicPlayerState>, _req: Request) {
    state.user_state.channel_handle = audio_player::next_song(
        &mut state.user_state.sl,
        &mut state.user_state.wav,
        &state.user_state.song_list,
        &mut state.user_state.current_song_index,
    );
}

fn callback_prev(state: &mut ClientState<MusicPlayerState>, _req: Request) {
    state.user_state.channel_handle = audio_player::prev_song(
        &mut state.user_state.sl,
        &mut state.user_state.wav,
        &state.user_state.song_list,
        &mut state.user_state.current_song_index,
    );
}

fn callback_vol(state: &mut ClientState<MusicPlayerState>, req: Request) {
    state.user_state.volume = req.value.unwrap();
    audio_player::set_volume(
        &mut state.user_state.sl,
        state.user_state.channel_handle,
        state.user_state.volume,
    )
}
