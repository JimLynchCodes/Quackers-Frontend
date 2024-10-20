use std::{io::ErrorKind, net::TcpStream};
use std::env;

use bevy::{
    ecs::world::CommandQueue,
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
// use tungstenite::{connect, http::Response, stream::MaybeTlsStream, Message, WebSocket};
// use rustls::CryptoProvider;

use strum_macros::EnumString;

// Client to Server types
#[derive(Debug, PartialEq, EnumString, Serialize)]
pub enum C2SActionTypes {
    #[strum(serialize = "join", serialize = "j")]
    Join,

    #[strum(serialize = "quack", serialize = "q")]
    Quack,

    #[strum(serialize = "move", serialize = "m")]
    Move,

    #[strum(serialize = "interact", serialize = "i")]
    Interact,

    #[strum(serialize = "empty", serialize = "e")]
    Empty, // used as a default in order to ignore invalid inputs without panicing
}

// Server to Client actions
#[derive(Debug, PartialEq, EnumString, Serialize, Clone, Deserialize)]
pub enum S2CActionTypes {
    #[strum(serialize = "you_joined", serialize = "yj")]
    YouJoined,
    #[strum(serialize = "other_player_joined", serialize = "opj")]
    OtherPlayerJoined,

    #[strum(serialize = "you_quacked", serialize = "yq")]
    YouQuacked,
    #[strum(serialize = "other_player_quacked", serialize = "opq")]
    OtherPlayerQuacked,

    #[strum(serialize = "you_moved", serialize = "ym")]
    YouMoved,
    #[strum(serialize = "other_player_moved", serialize = "opm")]
    OtherPlayerMoved,

    #[strum(serialize = "you_got_crackers", serialize = "ygc")]
    YouGotCrackers,
    #[strum(serialize = "other_player_got_crackers", serialize = "opgc")]
    OtherPlayerGotCrackers,

    #[strum(serialize = "you_died", serialize = "yd")]
    YouDied,
    #[strum(serialize = "other_player_died", serialize = "opd")]
    OtherPlayerGotDied,

    #[strum(serialize = "empty", serialize = "e")]
    Empty,

    #[strum(serialize = "user_disconnected", serialize = "ud")]
    UserDisconnected,

    #[strum(serialize = "leaderboard_update", serialize = "lu")]
    LeaderboardUpdate,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GotCrackerResponseData {
    pub player_uuid: String,
    pub player_friendly_name: String,

    pub old_cracker_x_position: f32,
    pub old_cracker_y_position: f32,

    pub new_cracker_x_position: f32,
    pub new_cracker_y_position: f32,

    pub old_cracker_point_value: u64,
    pub new_cracker_point_value: u64,

    pub new_player_score: u64,
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<WebSocketConnectionEvents>();
    app.add_event::<YouJoinedWsReceived>();
    app.add_event::<OtherPlayerJoinedWsReceived>();
    app.add_event::<OtherPlayerMovedWsReceived>();
    app.add_event::<OtherPlayerQuackedWsReceived>();
    app.add_event::<MoveCrackersBevyEvent>();
    app.add_event::<UpdateYourScoreBevyEvent>();
    app.add_event::<UpdateLeaderboardBevyEvent>();
    app.add_event::<UserDisconnectedBevyEvent>();

    app.add_systems(Startup, actually_connect);
    // app.add_systems(Update, setup_connection);
    // app.add_systems(Update, handle_tasks);
    // app.add_systems(Update, receive_ws_msg);
}

// #[derive(Component)]
// pub struct WebSocketClient(
//     pub  (
//         WebSocket<MaybeTlsStream<TcpStream>>,
//         Response<Option<Vec<u8>>>,
//     ),
// );

#[derive(Event)]
enum WebSocketConnectionEvents {
    SetupConnection,
}

#[derive(Event, Debug, Clone)]
pub struct YouJoinedWsReceived {
    pub data: Value,
}

#[derive(Event, Debug, Clone, Deserialize)]
pub struct MoveCrackersBevyEvent {
    pub x_position: f32,
    pub y_position: f32,
    pub points: u64,
    pub you_got_crackers: bool,
}

#[derive(Event, Debug, Clone, Deserialize)]
pub struct UpdateLeaderboardBevyEvent {
    pub data: Value,
}

#[derive(Event, Debug, Clone, Deserialize)]
pub struct UpdateYourScoreBevyEvent {
    pub new_score: u64,
}

#[derive(Event, Debug, Clone, Deserialize)]
pub struct UserDisconnectedBevyEvent {
    pub data: Value,
}

#[derive(Event, Debug, Clone)]
pub struct OtherPlayerJoinedWsReceived {
    pub data: OtherPlayerData,
}

#[derive(Event, Debug, Clone)]
pub struct OtherPlayerQuackedWsReceived {
    pub data: Value,
}

#[derive(Event, Debug, Clone)]
pub struct OtherPlayerMovedWsReceived {
    pub data: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenericIncomingRequest {
    pub action_type: S2CActionTypes,
    pub data: Value,
}

fn actually_connect(
    _input: Res<ButtonInput<KeyCode>>,
    mut ev_connect: EventWriter<WebSocketConnectionEvents>,
) {
    ev_connect.send(WebSocketConnectionEvents::SetupConnection);
}

use crate::demo::other_player::DuckDirection;

use super::{cracker::YouGotCrackerSoundFx, other_player::OtherPlayerData};

// #[derive(Error, Debug)]
// enum ConnectionSetupError {
//     #[error("IO")]
//     Io(#[from] std::io::Error),
//     #[error("WebSocket")]
//     WebSocket(#[from] tungstenite::Error),
// }

// #[derive(Component)]
// struct WebSocketConnectionSetupTask(
//     #[allow(unused)] Task<Result<CommandQueue, ConnectionSetupError>>,
// );

// fn setup_connection(
//     mut ev_connect: EventReader<WebSocketConnectionEvents>,
//     mut commands: Commands,
// ) {
//     for ev in ev_connect.read() {
//         match ev {
//             WebSocketConnectionEvents::SetupConnection => {
//                 info!("Setting up connection!");
//                 let pool = AsyncComputeTaskPool::get();
//                 let entity = commands.spawn_empty().id();
//                 let task = pool.spawn(async move {
                    
//                     // CryptoProvider::install_default()?;

//                     // let backend_endpoint = Url::parse(&env::var("BACKEND_WS_ENDPOINT") // read from env var if it exists
//                     // .unwrap_or("ws://127.0.0.1:8000/ws")).unwrap(); // if env is not set, try connecting to local websocket server

//                     let backend_endpoint_s = &env::var("BACKEND_WS_ENDPOINT") // read from env var if it exists
//                     .unwrap_or("ws://127.0.0.1:8000/ws".to_string());

//                     let mut client = connect(backend_endpoint_s)?;
//                     match client.0.get_mut() {
//                         MaybeTlsStream::Plain(p) => p.set_nonblocking(true)?,
//                         MaybeTlsStream::Rustls(stream_owned) => {
//                             stream_owned.get_mut().set_nonblocking(true)?
//                         }
//                         _ => todo!(),
//                     };
//                     info!("Connected successfully!");
//                     let mut command_queue = CommandQueue::default();

//                     command_queue.push(move |world: &mut World| {
//                         world
//                             .entity_mut(entity)
//                             .insert(WebSocketClient(client))
//                             // Task is complete, so remove task component from entity
//                             .remove::<WebSocketConnectionSetupTask>();
//                     });

//                     Ok(command_queue)
//                 });
//                 commands
//                     .entity(entity)
//                     .insert(WebSocketConnectionSetupTask(task));
//             }
//         }
//     }
// }

// fn receive_ws_msg(
//     mut commands: Commands,
//     mut q: Query<(&mut WebSocketClient,)>,
//     mut bevy_event_writer_you_joined: EventWriter<YouJoinedWsReceived>,
//     mut bevy_event_writer_other_player_joined: EventWriter<OtherPlayerJoinedWsReceived>,
//     mut bevy_event_writer_other_player_quacked: EventWriter<OtherPlayerQuackedWsReceived>,
//     mut bevy_event_writer_other_player_moved: EventWriter<OtherPlayerMovedWsReceived>,
//     mut bevy_event_writer_move_crackers: EventWriter<MoveCrackersBevyEvent>,
//     mut bevy_event_writer_user_disconnected: EventWriter<UserDisconnectedBevyEvent>,
//     mut bevy_event_writer_update_your_score: EventWriter<UpdateYourScoreBevyEvent>,
//     mut bevy_event_writer_update_leaderboard: EventWriter<UpdateLeaderboardBevyEvent>,
//     audio: Res<YouGotCrackerSoundFx>,
//     audio_assets: Res<Assets<AudioSource>>,
// ) {
//     for (mut client,) in q.iter_mut() {
//         match client.0 .0.read() {
//             Ok(m) => {
//                 info!("Received message ws connect {m:?}");

//                 let generic_msg =
//                     serde_json::from_str(&m.to_text().unwrap()).unwrap_or_else(|op| {
//                         info!("Failed to parse incoming websocket message: {}", op);
//                         GenericIncomingRequest {
//                             action_type: S2CActionTypes::Empty,
//                             data: Value::Null,
//                         }
//                     });

//                 match generic_msg.action_type {
//                     S2CActionTypes::YouJoined => {
//                         info!("Received 'YouJoined' message from ws server!");
//                         bevy_event_writer_you_joined.send(YouJoinedWsReceived {
//                             data: generic_msg.data,
//                         });

//                         // bevy_event_writer_move_crackers.send(YouJoinedWsReceived{ data: generic_msg.data });
//                         // bevy_event_writer_move_crackers.send(YouJoinedWsReceived{ data: generic_msg.data });
//                     }
//                     S2CActionTypes::OtherPlayerJoined => {
                        
//                         let other_player_joined_response_data = serde_json::from_value(generic_msg.data.clone())
//                         .unwrap_or_else(|op| {
//                             info!("Failed to parse incoming websocket message: {}", op);
//                             OtherPlayerData {
//                                 player_uuid: "error".to_string(),
//                                 player_friendly_name: "error".to_string(),
//                                 color: "error".to_string(),
//                                 x_position: 0.,
//                                 y_position: 0.,
//                                 direction_facing: DuckDirection::Right,
//                             }
//                         });

//                         bevy_event_writer_other_player_joined.send(OtherPlayerJoinedWsReceived {
//                             // data: generic_msg.data,
//                             data: other_player_joined_response_data,
//                         });
//                         info!("Received 'OtherPlayerJoined' message from ws server!");
//                     }
//                     S2CActionTypes::YouQuacked => {
//                         // Basically ignored (bc quack sound already played before sending to server)
//                         info!("Received 'YouQuacked' message from ws server!");
//                     }
//                     S2CActionTypes::OtherPlayerQuacked => {
//                         bevy_event_writer_other_player_quacked.send(OtherPlayerQuackedWsReceived {
//                             data: generic_msg.data,
//                         });
//                         info!("Received 'OtherPlayerQuacked' message from ws server!");
//                     }
//                     S2CActionTypes::YouMoved => {
//                         // Basically ignored (bc you already moved before sending to server)
//                         info!("Received 'YouMoved' message from ws server!");
//                     }
//                     S2CActionTypes::OtherPlayerMoved => {
//                         bevy_event_writer_other_player_moved.send(OtherPlayerMovedWsReceived {
//                             data: generic_msg.data,
//                         });
//                         info!("Received 'OtherPlayerMoved' message from ws server!");
//                     }
//                     S2CActionTypes::YouGotCrackers => {
//                         // Handle "YouGotCrackersMsg" from server.
//                         let you_got_crackers_msg_data =
//                             serde_json::from_value(generic_msg.data.clone()).unwrap_or_else(|op| {
//                                 info!("Failed to parse incoming websocket message: {}", op);
//                                 GotCrackerResponseData {
//                                     player_uuid: "error".to_string(),
//                                     player_friendly_name: "error".to_string(),
//                                     old_cracker_x_position: 0.,
//                                     old_cracker_y_position: 0.,
//                                     new_cracker_x_position: 0.,
//                                     new_cracker_y_position: 0.,
//                                     old_cracker_point_value: 0,
//                                     new_cracker_point_value: 0,
//                                     new_player_score: 0,
//                                 }
//                             });

//                         info!(
//                             "Received 'YouGotCrackers' message from ws server, new score: {}",
//                             you_got_crackers_msg_data.new_player_score
//                         );

//                         // Play special you got crackers sound
//                         if let Some(_) = audio_assets.get(&audio.sound_handle) {
//                             // Spawn an audio source to play the sound
//                             commands.spawn(AudioSourceBundle {
//                                 source: audio.sound_handle.clone(),
//                                 ..Default::default()
//                             });
//                             println!("Playing your quack sound.");
//                         } else {
//                             println!("Audio not loaded yet.");
//                         }

//                         // --> send event for crackers to move
//                         bevy_event_writer_move_crackers.send(MoveCrackersBevyEvent {
//                             x_position: you_got_crackers_msg_data.new_cracker_x_position,
//                             y_position: you_got_crackers_msg_data.new_cracker_y_position,
//                             points: you_got_crackers_msg_data.new_cracker_point_value,
//                             you_got_crackers: true,
//                         });

//                         // --> send event to update your score
//                         bevy_event_writer_update_your_score.send(UpdateYourScoreBevyEvent {
//                             new_score: you_got_crackers_msg_data.new_player_score,
//                         });
//                     }
//                     S2CActionTypes::OtherPlayerGotCrackers => {
//                         // Handle "OtherPlayerGotCrackers" from server.
//                         let other_player_got_crackers_msg_data =
//                             serde_json::from_value(generic_msg.data.clone()).unwrap_or_else(|op| {
//                                 info!("Failed to parse incoming websocket message: {}", op);
//                                 GotCrackerResponseData {
//                                     player_uuid: "error".to_string(),
//                                     player_friendly_name: "error".to_string(),
//                                     old_cracker_x_position: 0.,
//                                     old_cracker_y_position: 0.,
//                                     new_cracker_x_position: 0.,
//                                     new_cracker_y_position: 0.,
//                                     old_cracker_point_value: 0,
//                                     new_cracker_point_value: 0,
//                                     new_player_score: 0,
//                                 }
//                             });

//                         // --> send event for crackers to move
//                         bevy_event_writer_move_crackers.send(MoveCrackersBevyEvent {
//                             x_position: other_player_got_crackers_msg_data.new_cracker_x_position,
//                             y_position: other_player_got_crackers_msg_data.new_cracker_y_position,
//                             points: other_player_got_crackers_msg_data.new_cracker_point_value,
//                             you_got_crackers: false,
//                         });
//                         info!("Received 'OtherPlayerGotCrackers' message from ws server!");
//                     }
//                     S2CActionTypes::YouDied => {
//                         info!("Received 'YouDied' message from ws server!");
//                     }
//                     S2CActionTypes::OtherPlayerGotDied => {
//                         info!("Received 'OtherPlayerGotDied' message from ws server!");
//                     }
//                     S2CActionTypes::UserDisconnected => {
//                         bevy_event_writer_user_disconnected.send(UserDisconnectedBevyEvent {
//                             data: generic_msg.data,
//                         });

//                         info!("Received 'UserDisconnected' message from ws server!");
//                     }
//                     S2CActionTypes::Empty => {
//                         info!("Received 'Empty' message from ws server!");
//                     }
//                     S2CActionTypes::LeaderboardUpdate => {

//                         bevy_event_writer_update_leaderboard.send(UpdateLeaderboardBevyEvent { data: generic_msg.data });
//                         info!("Received 'LeaderboardUpdate' message from ws server!");
//                     }
//                 }
//             }
//             Err(tungstenite::Error::Io(e)) if e.kind() == ErrorKind::WouldBlock => { /* ignore */ }
//             Err(e) => warn!("error receiving: {e}"),
//         }
//     }
// }

// This system queries for entities that have our Task<Transform> component. It polls the
// tasks to see if they're complete. If the task is complete it takes the result, adds a
// new [`Mesh3d`] and [`MeshMaterial3d`] to the entity using the result from the task's work, and
// removes the task component from the entity.
// fn handle_tasks(
//     mut commands: Commands,
//     mut transform_tasks: Query<&mut WebSocketConnectionSetupTask>,
// ) {
//     for mut task in &mut transform_tasks {
//         if let Some(result) = block_on(future::poll_once(&mut task.0)) {
//             // append the returned command queue to have it execute later
//             match result {
//                 Ok(mut commands_queue) => {
//                     commands.append(&mut commands_queue);
//                 }
//                 Err(e) => {
//                     info!("Connection failed with: {e:?}");
//                 }
//             }
//         }
//     }
// }
