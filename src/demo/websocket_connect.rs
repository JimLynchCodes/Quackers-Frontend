use std::env;
use std::{io::ErrorKind, net::TcpStream};

use thiserror::Error;

use bevy::{
    ecs::world::CommandQueue,
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::JsValue;

// use tungstenite::{connect, http::Response, stream::MaybeTlsStream, Message, WebSocket};
// use rustls::CryptoProvider;

use strum_macros::EnumString;
// extern crate web_sys;

use ewebsock::{WsEvent, WsMessage};
use futures::channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use wasm_bindgen_futures::spawn_local;

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

    // app.add_systems(Startup, setup_scene)
    app.add_systems(Startup, begin_ws_connect);
    app.add_systems(Update, setup_connection);
    app.add_systems(Update, handle_tasks);
    app.add_event::<WebSocketConnectionEvents>();
    // app.add_systems(Update, send_info);
    app.add_systems(Update, recv_info);
    // app.insert_resource(SendMessageConfig {
    //     timer: Timer::new(Duration::from_secs(4), TimerMode::Repeating),
    // });

    // app.add_systems(Startup, actually_connect);
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

#[derive(Debug, Clone, Deserialize, Serialize, Event)]
pub struct GenericIncomingRequest {
    pub action_type: S2CActionTypes,
    pub data: Value,
}

// fn actually_connect(// _input: Res<ButtonInput<KeyCode>>,
//     mut ev_connect: EventWriter<WebSocketConnectionEvents>,
//     mut commands: Commands
// ) {
//     // ev_connect.send(WebSocketConnectionEvents::SetupConnection);

//     let options = ewebsock::Options::default();
//     // see documentation for more options
//     let (mut sender, receiver) = ewebsock::connect("ws://0.0.0.0:8000/ws", options).unwrap();
//     // let (mut sender, receiver) = ewebsock::connect("wss://quackers-beta.jimlynchcodes.com/ws/", options).unwrap();

//     println!("Conected!");

//     // commands.spawn(());

//     sender.send(ewebsock::WsMessage::Text("Hello!".into()));
//     while let Some(event) = receiver.try_recv() {
//         println!("Received message {:?}", event);

//         // web_sys::log_1(&"Received message {:?}".into(), event);
//         // web_sys::console::log_1("foo");

//         // ev_connect.send(WebSocketConnectionEvents::SetupConnection);
//         // sender.send(ewebsock::WsMessage::Text("Hello!".into()));
//     }
// }

// Define a struct for WebSocket messages
// #[derive(Debug)]
// pub struct WebSocketMessage(WsEvent);

// // Create a wrapper struct for the WebSocket message sender
// #[derive(Resource)]
// pub struct WebSocketMessageSender(UnboundedSender<WebSocketMessage>);

// // Create a wrapper struct for the WebSocket message receiver
// #[derive(Resource)]
// pub struct WebSocketMessageReceiver(UnboundedReceiver<WebSocketMessage>);

// use ewebsock::WsSender as WebSocketSender;

// #[derive(Component)]
// pub struct WebSocketClient {
//     pub sender: WebSocketSender, // This will be the sender to send messages through the WebSocket
//     // You can add additional fields if needed
//     // For example: an identifier for the client, state information, etc.
// }

// fn setup_connection_old(
//     mut ev_connect: EventReader<WebSocketConnectionEvents>,
//     mut commands: Commands,
//     sender: Res<WebSocketMessageSender>, // Use the sender resource
// ) {
//     for ev in ev_connect.read() {
//         match ev {
//             WebSocketConnectionEvents::SetupConnection => {
//                 info!("Setting up connection!");

//                 let entity = commands.spawn_empty().id();

//                 spawn_local(async move {
//                     let options = ewebsock::Options::default();

//                     // Connect using ewebsock
//                     let (websocket_sender, mut receiver) = ewebsock::connect("ws://0.0.0.0:8000/ws", options)
//                         // .await
//                         .expect("Failed to connect to WebSocket");

//                     info!("Connected successfully!");

//                     // Create a task to handle incoming messages
//                     spawn_local(async move {
//                         // Loop to handle incoming messages

//                         while let Some(event) = receiver.try_recv() {
//                             println!("Received {:?}", event);

//                             // Send the message through the channel
//                             // if let Err(_) = sender.0.unbounded_send(WebSocketMessage(event)) {
//                             //     info!("Failed to send message through channel");
//                             // }
//                         }
//                     });

//                     // Insert the WebSocketClient component in Bevy
//                     // commands.entity(entity).insert(WebSocketClient { sender: websocket_sender });
//                 });
//             }
//         }
//     }
// }

// fn handle_incoming_messages(
//     mut receiver: ResMut<WebSocketMessageReceiver>, // Use the wrapper struct
//     // mut query: Query<&mut WebSocketClient>, // Optionally, access WebSocketClient if needed
// ) {
//     // Process messages from the channel
//     while let Ok(message) = receiver.0.try_next() { // Access the inner receiver
//         for msg in message {
//             // Handle the incoming message
//             // info!("Received WebSocket message: {}", msg.0);
//             // You can also modify the Bevy world or entities based on the message here
//         }
//     }
// }

use crate::demo::other_player::DuckDirection;
use crate::demo::websocket_join_msg::build_join_request_msg;

use super::{cracker::YouGotCrackerSoundFx, other_player::OtherPlayerData};

// use std::{
//     sync::Mutex},
//     time::Duration,
// };

use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};

#[cfg(not(target_arch = "wasm32"))]
use tungstenite::{connect, http::Response, stream::MaybeTlsStream, Message, WebSocket};

// fn main() {
//     #[cfg(not(target_arch = "wasm32"))]
//     {
//         rustls::crypto::aws_lc_rs::default_provider()
//             .install_default()
//             .expect("Failed to install rustls crypto provider");
//     }
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(PerfUiPlugin)
//         .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
//         .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
//         .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
//         .add_plugins(PhysicsPlugins::default())
//         .add_systems(Startup, setup_scene)
//         .add_systems(Update, check_connection_input)
//         .add_systems(Update, setup_connection)
//         .add_systems(Update, handle_tasks)
//         .add_event::<WebSocketConnectionEvents>()
//         .add_systems(Update, send_info)
//         .add_systems(Update, recv_info)
//         .insert_resource(SendMessageConfig {
//             timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
//         })
//         .run();
// }

#[cfg(target_arch = "wasm32")]
mod wasm_websocket {
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    use bevy::log::info;
    use web_sys::{
        js_sys::{ArrayBuffer, Uint8Array},
        wasm_bindgen::{prelude::Closure, JsCast},
        BinaryType, Event, MessageEvent,
    };

    use wasm_bindgen::JsValue;

    pub struct Client {
        pub socket: web_sys::WebSocket,
        pub recv_queue: Rc<RefCell<VecDeque<JsValue>>>,
        _open_cb: Closure<dyn FnMut(Event)>,
        _message_cb: Closure<dyn FnMut(MessageEvent)>,
    }

    impl Client {
        pub fn new(url: &str) -> send_wrapper::SendWrapper<Self> {
            info!("Opening wasm websocket");
            let recv_queue = Rc::new(RefCell::new(VecDeque::new()));
            let socket = web_sys::WebSocket::new(url).expect("Failed to create WebSocket object");
            socket.set_binary_type(BinaryType::Arraybuffer);
            let open_cb: Closure<dyn FnMut(_)> = Closure::new(|_event: Event| {
                web_sys::console::log_1(&"Connection opened".into());
            });
            socket
                .add_event_listener_with_callback("open", open_cb.as_ref().dyn_ref().unwrap())
                .unwrap();
            let message_cb: Closure<dyn FnMut(_)> = Closure::new({
                let recv_queue = Rc::clone(&recv_queue);
                move |event: MessageEvent| {
                    web_sys::console::log_1(&format!("Got message: {:?}", event.data()).into());
                    // if let Some(buf) = event.data().dyn_ref::<ArrayBuffer>() {

                    let data_vec = Uint8Array::new(&event.data().into()).to_vec();
                    web_sys::console::log_1(&format!("Pushin it!: {:?}", event.data()).into());
                    web_sys::console::log_1(&format!("Pushin vec!: {:?}", data_vec).into());
                    web_sys::console::log_1(
                        &format!("as a string!: {:?}", event.data().as_string()).into(),
                    );

                    recv_queue.borrow_mut().push_back(event.data().clone());
                    // }
                }
            });
            socket
                .add_event_listener_with_callback("message", message_cb.as_ref().dyn_ref().unwrap())
                .unwrap();
            send_wrapper::SendWrapper::new(Client {
                socket,
                recv_queue,
                _open_cb: open_cb,
                _message_cb: message_cb,
            })
        }
    }
}

#[derive(Component)]
pub struct WebSocketClient(
    #[cfg(target_arch = "wasm32")] pub send_wrapper::SendWrapper<wasm_websocket::Client>,
    #[cfg(not(target_arch = "wasm32"))]
    pub  (
        WebSocket<MaybeTlsStream<TcpStream>>,
        Response<Option<Vec<u8>>>,
    ),
);

// #[derive(Event)]
// enum WebSocketConnectionEvents {
//     SetupConnection,
// }

fn begin_ws_connect(
    // input: Res<ButtonInput<KeyCode>>,
    mut ev_connect: EventWriter<WebSocketConnectionEvents>,
) {
    // if input.just_pressed(KeyCode::Space) {
    // set up connection

    ev_connect.send(WebSocketConnectionEvents::SetupConnection);
    // }
}

// use thiserror::Error;
// use web_sys::MessageEvent;
// #[cfg(target_arch = "wasm32")]
// use web_sys::{
//     wasm_bindgen::{prelude::Closure, JsCast, JsValue},
//     Event,
// };

#[derive(Error, Debug)]
enum ConnectionSetupError {
    #[error("IO")]
    Io(#[from] std::io::Error),
    #[cfg(target_arch = "wasm32")]
    #[error("WebSocket")]
    WebSocket(), // TODO: remove or fill in actual error and do error handling with it?
    #[cfg(not(target_arch = "wasm32"))]
    #[error("WebSocket")]
    WebSocket(#[from] tungstenite::Error),
}

#[derive(Component)]
struct WebSocketConnectionSetupTask(
    #[allow(unused)] Task<Result<CommandQueue, ConnectionSetupError>>,
);

#[derive(Serialize)]
struct MyMessage {
    content: String,
    timestamp: u64,
}

impl MyMessage {
    fn new(content: String) -> Self {
        MyMessage {
            content: "".to_string(),
            timestamp: 0,
        }
    }
}

fn setup_connection(
    mut ev_connect: EventReader<WebSocketConnectionEvents>,
    mut commands: Commands,
) {
    for ev in ev_connect.read() {
        match ev {
            WebSocketConnectionEvents::SetupConnection => {
                info!("Setting up connection!");

                let mut url = "".to_string();
                let default_url = "ws://127.0.0.1:8000/ws".to_string();

                #[cfg(not(target_arch = "wasm32"))]
                {
                    url = std::env::var("BACKEND_WS_ENDPOINT".to_string()).unwrap_or(default_url);
                    info!("endpoint is: {}", url)
                }

                #[cfg(target_arch = "wasm32")]
                {
                    let value = env!("BACKEND_WS_ENDPOINT"); // TODO prevent from crashing and use default?
                    if value == "" {
                        url = default_url;
                    } else {
                        url = value.to_string();
                    }
                }

                let entity = commands.spawn_empty().id();

                // Define the message to send
                let message = MyMessage::new("Hello, WebSocket!".to_string());
                // let json_message = serde_json::to_string(&message).unwrap();
                let json_message = build_join_request_msg("foo".to_string());

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let pool = AsyncComputeTaskPool::get();
                    let task = pool.spawn(async move {
                        let mut client = connect(&url)?;
                        match client.0.get_mut() {
                            MaybeTlsStream::Plain(p) => p.set_nonblocking(true)?,
                            MaybeTlsStream::Rustls(stream_owned) => {
                                stream_owned.get_mut().set_nonblocking(true)?
                            }
                            _ => todo!(),
                        };
                        info!("Connected successfully!");
                        // client.0.send(tungstenite::Message::Text(json_message))?;

                        let mut command_queue = CommandQueue::default();

                        command_queue.push(move |world: &mut World| {
                            world
                                .entity_mut(entity)
                                .insert(WebSocketClient(client))
                                // Task is complete, so remove task component from entity
                                .remove::<WebSocketConnectionSetupTask>();
                        });

                        Ok(command_queue)
                    });
                    commands
                        .entity(entity)
                        .insert(WebSocketConnectionSetupTask(task));
                }
                #[cfg(target_arch = "wasm32")]
                {
                    web_sys::console::log_1(&"//1 wasm connecting".into());
                    // Use the ewebsock or wasm-websocket client to send the message
                    let client = wasm_websocket::Client::new(&url);
                    web_sys::console::log_1(&"//1 wasm connected".into());

                    // let message = MyMessage::new("Hello, WebSocket!".to_string());
                    // let json_message = serde_json::to_string(&message).unwrap();

                    // let msg = bincode::serialize(&json_message).unwrap();

                    // web_sys::console::log_1(&"//wasm built message".into());
                    // web_sys::console::log_1(&client.socket.into());
                    // client.socket.send(json_message);

                    // client.socket.send_info(msg);
                    // let _ = client.socket.send_with_str(&json_message);
                    // match client.socket.send_with_str(&json_message) {
                    //     Ok(_) => web_sys::console::log_1(&"Message sent successfully".into()),
                    //     Err(err) => web_sys::console::log_1(&format!("Error sending message: {:?}", err).into()),
                    // }

                    // client

                    // web_sys::console::log_1(&"//wasm sent message".into());
                    // client.send(json_message);

                    commands.entity(entity).insert(WebSocketClient(client));
                }
            }
        }
    }
}

fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<&mut WebSocketConnectionSetupTask>,
) {
    for mut task in &mut transform_tasks {
        if let Some(result) = block_on(future::poll_once(&mut task.0)) {
            // append the returned command queue to have it execute later
            match result {
                Ok(mut commands_queue) => {
                    commands.append(&mut commands_queue);
                }
                Err(e) => {
                    info!("Connection failed with: {e:?}");
                }
            }
        }
    }
}

#[derive(Resource)]
struct SendMessageConfig {
    timer: Timer,
}

// fn send_info(
//     some_data: Query<(&Transform,)>,
//     time: Res<Time>,
//     mut entities_with_client: Query<(&mut WebSocketClient,)>,
//     // mut config: ResMut<SendMessageConfig>,
// ) {
//     // config.timer.tick(time.delta());
//     if config.timer.finished() {
//         // only send messages once every second, so we don't spam the server
//         info!("Time to send data again...");
//         for (mut client,) in entities_with_client.iter_mut() {
//             let transforms = &some_data.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
//             info!("Sending data: {transforms:?}");
//             // let msg = bincode::serialize(transforms).unwrap();
//             let json_message = build_join_request_msg("foo".to_string());
//             #[cfg(target_arch = "wasm32")]
//             {
//                 // let message = MyMessage::new("Hello, WebSocket!".to_string());
//                 // let json_message = serde_json::to_string(&message).unwrap();

//                 // let message = MyMessage::new("Hello, WebSocket!".to_string());
//                 // let json_message = serde_json::to_string(&message).unwrap();

//                 match client.0.socket.send_with_str(&json_message) {
//                     Ok(_) => web_sys::console::log_1(&"Message sent successfully".into()),
//                     Err(err) => {
//                         web_sys::console::log_1(&format!("Error sending message: {:?}", err).into())
//                     }
//                 }

//                 // TODO: do some handling so we know whether the websocket is connected yet
//                 // let _ = client.0.socket.send_with_u8_array(msg.as_slice()); // ignore the error because the websocket may still be connecting
//             }
//             #[cfg(not(target_arch = "wasm32"))]
//             {
//                 match client.0 .0.send(Message::Text(json_message)) {
//                     Ok(_) => info!("Data successfully sent!"),
//                     #[cfg(not(target_arch = "wasm32"))]
//                     Err(tungstenite::Error::Io(e)) if e.kind() == ErrorKind::WouldBlock => { /* ignore */
//                     }
//                     Err(e) => {
//                         warn!("Could not send the message: {e:?}");
//                     }
//                 }
//             }
//         }
//     }
// }

fn recv_info(
    mut q: Query<(&mut WebSocketClient,)>,
    mut bevy_event_writer_you_joined: EventWriter<YouJoinedWsReceived>,

    //     mut bevy_event_writer_you_joined: EventWriter<YouJoinedWsReceived>,
    mut bevy_event_writer_other_player_joined: EventWriter<OtherPlayerJoinedWsReceived>,
    mut bevy_event_writer_other_player_quacked: EventWriter<OtherPlayerQuackedWsReceived>,
    mut bevy_event_writer_other_player_moved: EventWriter<OtherPlayerMovedWsReceived>,
    mut bevy_event_writer_move_crackers: EventWriter<MoveCrackersBevyEvent>,
    mut bevy_event_writer_user_disconnected: EventWriter<UserDisconnectedBevyEvent>,
    mut bevy_event_writer_update_your_score: EventWriter<UpdateYourScoreBevyEvent>,
    mut bevy_event_writer_update_leaderboard: EventWriter<UpdateLeaderboardBevyEvent>,
    audio: Res<YouGotCrackerSoundFx>,
    audio_assets: Res<Assets<AudioSource>>,
    mut commands: Commands,
    // mut bevy_event_writer_generic_message: EventWriter<GenericIncomingRequest>,
) {
    // let generic_msg =
    //                     serde_json::from_str(&m.to_text().unwrap()).unwrap_or_else(|op| {
    //                         info!("Failed to parse incoming websocket message: {}", op);
    //                         GenericIncomingRequest {
    //                             action_type: S2CActionTypes::Empty,
    //                             data: Value::Null,
    //                         }
    //                     });

    //                 match generic_msg.action_type {
    for (mut client,) in q.iter_mut() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match client.0 .0.read() {
                Ok(m) => {
                    info!("Received message {m:?}");

                    let generic_msg =
                        serde_json::from_str(&m.to_text().unwrap()).unwrap_or_else(|op| {
                            info!("Failed to parse incoming websocket message: {}", op);
                            GenericIncomingRequest {
                                action_type: S2CActionTypes::Empty,
                                data: Value::Null,
                            }
                        });

                    fun_name(
                        &mut commands,
                        generic_msg,
                        &mut bevy_event_writer_you_joined,
                        &mut bevy_event_writer_other_player_joined,
                        &mut bevy_event_writer_other_player_quacked,
                        &mut bevy_event_writer_other_player_moved,
                        &audio_assets,
                        &audio,
                        &mut bevy_event_writer_move_crackers,
                        &mut bevy_event_writer_update_your_score,
                        &mut bevy_event_writer_user_disconnected,
                        &mut bevy_event_writer_update_leaderboard,
                    );

                    // bevy_event_writer_you_joined.send(generic_msg);

                    // handle_incoming_ws_message(generic_msg, bevy_event_writer_you_joined);
                }
                Err(tungstenite::Error::Io(e)) if e.kind() == ErrorKind::WouldBlock => { /* ignore */
                }
                Err(e) => warn!("error receiving: {e}"),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            // info!("Queue {:?}", client.0.recv_queue.length);

            while let Some(m) = client.0.recv_queue.borrow_mut().pop_front() {
                web_sys::console::log_1(&format!("Received message {:?}", m).into());

                if let Some(json_str) = m.as_string() {
                    // If the JsValue is a string, parse it as JSON using serde_json::from_str
                    let generic_msg: GenericIncomingRequest = serde_json::from_str(&json_str)
                        .unwrap_or_else(|err| {
                            web_sys::console::log_1(
                                &format!("Failed to parse JSON string: {:?}", err).into(),
                            );
                            // Fallback to a default empty message if parsing fails
                            GenericIncomingRequest {
                                action_type: S2CActionTypes::Empty,
                                data: Value::Null,
                            }
                        });

                    web_sys::console::log_1(
                        &format!("Parsed message from string: {:?}", generic_msg).into(),
                    );

                    fun_name(
                        &mut commands,
                        generic_msg,
                        &mut bevy_event_writer_you_joined,
                        &mut bevy_event_writer_other_player_joined,
                        &mut bevy_event_writer_other_player_quacked,
                        &mut bevy_event_writer_other_player_moved,
                        &audio_assets,
                        &audio,
                        &mut bevy_event_writer_move_crackers,
                        &mut bevy_event_writer_update_your_score,
                        &mut bevy_event_writer_user_disconnected,
                        &mut bevy_event_writer_update_leaderboard,
                    );

                    // match generic_msg.action_type {
                    //     S2CActionTypes::YouJoined => {
                    //         info!("Received 'YouJoined' message from ws server!");
                    //         bevy_event_writer_you_joined.send(YouJoinedWsReceived {
                    //             data: generic_msg.data,
                    //         });
                    //     }
                    //     _ => (),
                    // }
                }
                // else {
                //     // Step 2: If it's not a string, attempt to deserialize the JsValue directly
                //     let generic_msg: GenericIncomingRequest = serde_wasm_bindgen::from_value(m)
                //         .unwrap_or_else(|err| {
                //             web_sys::console::log_1(
                //                 &format!("Failed to deserialize JsValue: {:?}", err).into(),
                //             );
                //             // Fallback to a default empty message if deserialization fails
                //             GenericIncomingRequest {
                //                 action_type: S2CActionTypes::Empty,
                //                 data: Value::Null,
                //             }
                //         });

                //     web_sys::console::log_1(
                //         &format!("Parsed message from JsValue object: {:?}", generic_msg).into(),
                //     );
                // }

                // let parsed_msg: GenericIncomingRequest = match web_sys::js_sys::JSON::parse(m) {
                //     Ok(parsed) => {

                //         web_sys::console::log_1(&format!("parsed JSON: {:?}", err).into());
                //         parsed
                //     },
                //     Err(err) => {
                //         web_sys::console::log_1(&format!("Failed to parse JSON: {:?}", err).into());
                //         return;
                //     }
                // };

                // let parsed_msg = match m.from_serde()  {
                //     Ok(parsed) => parsed,
                //     Err(err) => {
                //         web_sys::console::log_1(&format!("Failed to parse JSON: {:?}", err).into());
                //         return;
                //     }
                // };

                // if let Some(stringified) = m.as_string() {
                //     web_sys::console::log_1(&format!("Stringified it: {:?}", stringified).into());

                // let generic_msg: GenericIncomingRequest = serde_json::from_str(&stringified)
                //     .unwrap_or_else(|err| {
                //         web_sys::console::log_1(
                //             &format!("Failed to parse incoming websocket message: {:?}", err)
                //                 .into(),
                //         );
                //         // Fallback to a default empty message if deserialization fails
                //         GenericIncomingRequest {
                //             action_type: S2CActionTypes::Empty,
                //             data: Value::Null,
                //         }
                //     });

                // web_sys::console::log_1(
                //     &format!(
                //         "Parsed a generic message. action type is! {:?}",
                //         &generic_msg.action_type
                //     )
                //     .into(),
                // );

                // // bevy_event_writer_generic_message.send(generic_msg);

                // match generic_msg.action_type {
                //     S2CActionTypes::YouJoined => {
                //         info!("Received 'YouJoined' message from ws server!");
                //         bevy_event_writer_you_joined.send(YouJoinedWsReceived {
                //             data: generic_msg.data,
                //         });
                //     }
                //     _ => (),
                // }
                // }

                // serde_json::from_str();

                // let generic_msg = match m {
                //     Ok()
                // }
                // serde_json::from_str(&m).unwrap_or_else(|op| {
                //     info!("Failed to parse incoming websocket message: {}", op);
                //     GenericIncomingRequest {
                //         action_type: S2CActionTypes::Empty,
                //         data: Value::Null,
                //     }
                // });
                // bevy_event_writer_generic_message.send(generic_msg);

                // match generic_msg.action_type {
                //     S2CActionTypes::YouJoined => {
                //         info!("Received 'YouJoined' message from ws server!");
                //         bevy_event_writer_you_joined.send(YouJoinedWsReceived {
                //             data: generic_msg.data,
                //         });
                //     }
                //     _ => (),
                // }
            }
        }
    }
}

fn fun_name(
    commands: &mut Commands,
    generic_msg: GenericIncomingRequest,
    bevy_event_writer_you_joined: &mut EventWriter<'_, YouJoinedWsReceived>,
    bevy_event_writer_other_player_joined: &mut EventWriter<'_, OtherPlayerJoinedWsReceived>,
    bevy_event_writer_other_player_quacked: &mut EventWriter<'_, OtherPlayerQuackedWsReceived>,
    bevy_event_writer_other_player_moved: &mut EventWriter<'_, OtherPlayerMovedWsReceived>,
    audio_assets: &Res<'_, Assets<AudioSource>>,
    audio: &Res<'_, YouGotCrackerSoundFx>,
    bevy_event_writer_move_crackers: &mut EventWriter<'_, MoveCrackersBevyEvent>,
    bevy_event_writer_update_your_score: &mut EventWriter<'_, UpdateYourScoreBevyEvent>,
    bevy_event_writer_user_disconnected: &mut EventWriter<'_, UserDisconnectedBevyEvent>,
    bevy_event_writer_update_leaderboard: &mut EventWriter<'_, UpdateLeaderboardBevyEvent>,
) {
    match generic_msg.action_type {
        S2CActionTypes::YouJoined => {
            info!("Received 'YouJoined' message from ws server!");
            bevy_event_writer_you_joined.send(YouJoinedWsReceived {
                data: generic_msg.data,
            });

            // bevy_event_writer_move_crackers.send(YouJoinedWsReceived{ data: generic_msg.data });
            // bevy_event_writer_move_crackers.send(YouJoinedWsReceived{ data: generic_msg.data });
        }
        S2CActionTypes::OtherPlayerJoined => {
            let other_player_joined_response_data =
                serde_json::from_value(generic_msg.data.clone()).unwrap_or_else(|op| {
                    info!("Failed to parse incoming websocket message: {}", op);
                    OtherPlayerData {
                        player_uuid: "error".to_string(),
                        player_friendly_name: "error".to_string(),
                        color: "error".to_string(),
                        x_position: 0.,
                        y_position: 0.,
                        direction_facing: DuckDirection::Right,
                    }
                });

            bevy_event_writer_other_player_joined.send(OtherPlayerJoinedWsReceived {
                // data: generic_msg.data,
                data: other_player_joined_response_data,
            });
            info!("Received 'OtherPlayerJoined' message from ws server!");
        }
        S2CActionTypes::YouQuacked => {
            // Basically ignored (bc quack sound already played before sending to server)
            info!("Received 'YouQuacked' message from ws server!");
        }
        S2CActionTypes::OtherPlayerQuacked => {
            bevy_event_writer_other_player_quacked.send(OtherPlayerQuackedWsReceived {
                data: generic_msg.data,
            });
            info!("Received 'OtherPlayerQuacked' message from ws server!");
        }
        S2CActionTypes::YouMoved => {
            // Basically ignored (bc you already moved before sending to server)
            info!("Received 'YouMoved' message from ws server!");
        }
        S2CActionTypes::OtherPlayerMoved => {
            bevy_event_writer_other_player_moved.send(OtherPlayerMovedWsReceived {
                data: generic_msg.data,
            });
            info!("Received 'OtherPlayerMoved' message from ws server!");
        }
        S2CActionTypes::YouGotCrackers => {
            // Handle "YouGotCrackersMsg" from server.
            let you_got_crackers_msg_data = serde_json::from_value(generic_msg.data.clone())
                .unwrap_or_else(|op| {
                    info!("Failed to parse incoming websocket message: {}", op);
                    GotCrackerResponseData {
                        player_uuid: "error".to_string(),
                        player_friendly_name: "error".to_string(),
                        old_cracker_x_position: 0.,
                        old_cracker_y_position: 0.,
                        new_cracker_x_position: 0.,
                        new_cracker_y_position: 0.,
                        old_cracker_point_value: 0,
                        new_cracker_point_value: 0,
                        new_player_score: 0,
                    }
                });

            info!(
                "Received 'YouGotCrackers' message from ws server, new score: {}",
                you_got_crackers_msg_data.new_player_score
            );

            // Play special you got crackers sound
            if let Some(_) = audio_assets.get(&audio.sound_handle) {
                // Spawn an audio source to play the sound
                commands.spawn(AudioSourceBundle {
                    source: audio.sound_handle.clone(),
                    ..Default::default()
                });
                println!("Playing your quack sound.");
            } else {
                println!("Audio not loaded yet.");
            }

            // --> send event for crackers to move
            bevy_event_writer_move_crackers.send(MoveCrackersBevyEvent {
                x_position: you_got_crackers_msg_data.new_cracker_x_position,
                y_position: you_got_crackers_msg_data.new_cracker_y_position,
                points: you_got_crackers_msg_data.new_cracker_point_value,
                you_got_crackers: true,
            });

            // --> send event to update your score
            bevy_event_writer_update_your_score.send(UpdateYourScoreBevyEvent {
                new_score: you_got_crackers_msg_data.new_player_score,
            });
        }
        S2CActionTypes::OtherPlayerGotCrackers => {
            // Handle "OtherPlayerGotCrackers" from server.
            let other_player_got_crackers_msg_data =
                serde_json::from_value(generic_msg.data.clone()).unwrap_or_else(|op| {
                    info!("Failed to parse incoming websocket message: {}", op);
                    GotCrackerResponseData {
                        player_uuid: "error".to_string(),
                        player_friendly_name: "error".to_string(),
                        old_cracker_x_position: 0.,
                        old_cracker_y_position: 0.,
                        new_cracker_x_position: 0.,
                        new_cracker_y_position: 0.,
                        old_cracker_point_value: 0,
                        new_cracker_point_value: 0,
                        new_player_score: 0,
                    }
                });

            // --> send event for crackers to move
            bevy_event_writer_move_crackers.send(MoveCrackersBevyEvent {
                x_position: other_player_got_crackers_msg_data.new_cracker_x_position,
                y_position: other_player_got_crackers_msg_data.new_cracker_y_position,
                points: other_player_got_crackers_msg_data.new_cracker_point_value,
                you_got_crackers: false,
            });
            info!("Received 'OtherPlayerGotCrackers' message from ws server!");
        }
        S2CActionTypes::YouDied => {
            info!("Received 'YouDied' message from ws server!");
        }
        S2CActionTypes::OtherPlayerGotDied => {
            info!("Received 'OtherPlayerGotDied' message from ws server!");
        }
        S2CActionTypes::UserDisconnected => {
            bevy_event_writer_user_disconnected.send(UserDisconnectedBevyEvent {
                data: generic_msg.data,
            });

            info!("Received 'UserDisconnected' message from ws server!");
        }
        S2CActionTypes::Empty => {
            info!("Received 'Empty' message from ws server!");
        }
        S2CActionTypes::LeaderboardUpdate => {
            bevy_event_writer_update_leaderboard.send(UpdateLeaderboardBevyEvent {
                data: generic_msg.data,
            });
            info!("Received 'LeaderboardUpdate' message from ws server!");
        }
    }
}

// fn handle_generic_incoming_ws_message()
//     mut ev_connect: EventReader<GenericIncomingRequest>,
//     mut commands: Commands,
//     mut receiver_sender: ResMut<UnboundedSender<WebSocketMessage>>, // Channel for outgoing messages
// ) {
//     for generic_msg in ev_connect.read() {

//         match generic_msg {
//             _ =>
//         }

//     }
// )

// fn handle_incoming_ws_message(
//     generic_msg: GenericIncomingRequest,
//     mut bevy_event_writer_you_joined: EventWriter<YouJoinedWsReceived>,
// ) {
//     match generic_msg.action_type {
//         S2CActionTypes::YouJoined => {
//             info!("Received 'YouJoined' message from ws server!");
//             bevy_event_writer_you_joined.send(YouJoinedWsReceived {
//                 data: generic_msg.data,
//             });
//         }
//         _ => (),
//     }
// }

// #[derive(Resource)]
// pub struct WebSocketMessageReceiver(UnboundedReceiver<WsMessage>);

// fn handle_incoming_messages(
//     mut receiver: ResMut<WebSocketMessageReceiver>, // Use the wrapper struct
//     // mut query: Query<&mut WebSocketClient>, // Optionally, access WebSocketClient if needed
// ) {
//     // Process messages from the channel
//     while let Ok(message) = receiver.0.try_next() { // Access the inner receiver
//         for msg in message {
//             // Handle the incoming message
//             info!("Received WebSocket message: {}", msg.0);
//             // You can also modify the Bevy world or entities based on the message here
//         }
//     }
// }

// fn setup_connection(
//     mut ev_connect: EventReader<WebSocketConnectionEvents>,
//     mut commands: Commands,
//     mut receiver_sender: ResMut<UnboundedSender<WebSocketMessage>>, // Channel for outgoing messages
// ) {
//     for ev in ev_connect.read() {
//         match ev {
//             WebSocketConnectionEvents::SetupConnection => {
//                 info!("Setting up connection!");

//                 let entity = commands.spawn_empty().id();

//                 // Spawn the WebSocket task in WASM (or native) environment
//                 spawn_local(async move {
//                     let options = ewebsock::Options::default();

//                     // Connect using ewebsock
//                     let (sender, receiver) = ewebsock::connect("ws://0.0.0.0:8000/ws", options)
//                         .await
//                         .expect("Failed to connect to WebSocket");

//                     info!("Connected successfully!");

//                     // Create a task to handle incoming messages
//                     spawn_local(async move {
//                         let mut receiver = receiver;

//                         // Loop to handle incoming messages
//                         while let Some(msg) = receiver.next().await {
//                             match msg {
//                                 Ok(message) => {
//                                     // Convert message to string or appropriate format
//                                     let msg_str = message.to_string(); // Modify according to your message handling

//                                     // Send the message through the channel
//                                     if let Err(_) = receiver_sender.unbounded_send(WebSocketMessage(msg_str)) {
//                                         info!("Failed to send message through channel");
//                                     }
//                                 }
//                                 Err(e) => {
//                                     info!("Error receiving message: {:?}", e);
//                                 }
//                             }
//                         }
//                     });

//                     // Insert the WebSocketClient component in Bevy
//                     commands.entity(entity).insert(WebSocketClient { sender });
//                 });
//             }
//         }
//     }
// }

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
