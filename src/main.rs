mod audio_player;
use std::sync::mpsc::Sender;
use std::{error::Error, sync::mpsc::channel};

use NOSHP_Client::{
    client::{ClientState, NoshpClient, Request, UserDefinedState},
    client_config::ClientConfig,
};

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
    let (tx, rx) = channel();
    let player = audio_player::AudioPlayer::init(rx);

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
