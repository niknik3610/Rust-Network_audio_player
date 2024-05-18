mod audio_player;
mod ui;

use clap::Parser;
use std::sync::mpsc::Sender;
use std::{error::Error, fmt::Debug, sync::mpsc::channel};

use NOSHP_Client::{
    client::{ClientState, NoshpClient, Request, UserDefinedState},
    client_config::ClientConfig,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start Program with UI, default: false
    #[arg(short, long, default_value_t = false)]
    ui: bool,
}

struct MusicPlayerState {
    pub sender_channel: Sender<audio_player::AudioPlayerCommands>,
}
impl Default for MusicPlayerState {
    fn default() -> Self {
        let (tx, _rx) = channel();
        Self { sender_channel: tx }
    }
}
impl UserDefinedState for MusicPlayerState {}

const CONFIG_PATH: &str = "./config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let (tx, rx) = channel();
    let player = audio_player::AudioPlayer::init(rx);

    if args.ui {
        // let channel_handle =
        //     audio_player::next_song(&mut sl, &mut wav, &song_list, &mut current_song_index);
        //
        // audio_player::toggle_pause_song(&mut sl, pause_state);
        // ui::display_ui(
        //     &mut sl,
        //     &mut wav,
        //     &song_list,
        //     &mut current_song_index,
        //     channel_handle,
        //     &mut pause_state,
        //     &mut volume,
        // );
    } else {
        let config = ClientConfig::load_config(CONFIG_PATH).unwrap();
        let state = MusicPlayerState { sender_channel: tx };

        let client_handler = NoshpClient::new()
            .set_state(state)
            .add_callback("Pause/Play", Box::new(callback_toggle_pause))
            .add_callback("Next", Box::new(callback_next))
            .add_callback("Previous", Box::new(callback_prev))
            .add_callback("Volume", Box::new(callback_vol))
            .run(config);

        let music_handle = tokio::spawn(async move {
            player.run().await;
        });

        tokio::spawn(async move { client_handler.await.unwrap() })
            .await
            .unwrap();

        music_handle.await.unwrap();
    }
    Ok(())
}

fn callback_toggle_pause(state: &mut ClientState<MusicPlayerState>, _req: Request) {
    state
        .user_state
        .sender_channel
        .send(audio_player::AudioPlayerCommands::TogglePause)
        .unwrap();
}

fn callback_next(state: &mut ClientState<MusicPlayerState>, _req: Request) {
    state
        .user_state
        .sender_channel
        .send(audio_player::AudioPlayerCommands::NextSong)
        .unwrap();
}

fn callback_prev(state: &mut ClientState<MusicPlayerState>, _req: Request) {
    state
        .user_state
        .sender_channel
        .send(audio_player::AudioPlayerCommands::PrevSong)
        .unwrap();
}

fn callback_vol(state: &mut ClientState<MusicPlayerState>, req: Request) {
    let mut volume = req.value.unwrap();
    volume = volume / 100.0;
    state
        .user_state
        .sender_channel
        .send(audio_player::AudioPlayerCommands::SetVol(volume))
        .unwrap();
}
