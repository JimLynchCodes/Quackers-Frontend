use std::env;
// use std::sync::Arc;
use std::{io::ErrorKind, net::TcpStream};

// use futures_util::lock::Mutex;
// use tokio::sync::Mutex; // Import the tokio mutex
// use futures_util::{SinkExt, StreamExt};

use bevy::{
    ecs::world::CommandQueue,
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
// use tungstenite::{connect, http::Response, stream::MaybeTlsStream, Message, WebSocket};
// use rustls::CryptoProvider;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite_wasm::{connect, Message};
use std::sync::{Arc, Mutex};

use strum_macros::EnumString;

use tokio_tungstenite_wasm::WebSocketStream;
// use tokio::task;
use wasm_bindgen_futures::spawn_local;
use futures_util::stream::SplitSink;
use tokio::sync::mpsc;

use bevy::{
    color::palettes,
    tasks::{TaskPool, TaskPoolBuilder},
};
use bevy_eventwork::{ConnectionId, EventworkRuntime, Network, NetworkData, NetworkEvent};

use bevy_eventwork::NetworkMessage;
use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};

// use bevy_eventwork_mod_websockets::WebSocketProvider;
// use web_sys::console;

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


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserChatMessage {
    pub message: String,
}

impl NetworkMessage for UserChatMessage {
    const NAME: &'static str = "example:UserChatMessage";
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewChatMessage {
    pub name: String,
    pub message: String,
}

impl NetworkMessage for NewChatMessage {
    const NAME: &'static str = "example:NewChatMessage";
}

#[allow(unused)]
pub fn client_register_network_messages(app: &mut App) {
    use bevy_eventwork::AppNetworkMessage;

    // The client registers messages that arrives from the server, so that
    // it is prepared to handle them. Otherwise, an error occurs.
    app.listen_for_message::<NewChatMessage, WebSocketProvider>();
}

#[allow(unused)]
pub fn server_register_network_messages(app: &mut App) {
    use bevy_eventwork::AppNetworkMessage;

    // The server registers messages that arrives from a client, so that
    // it is prepared to handle them. Otherwise, an error occurs.
    app.listen_for_message::<UserChatMessage, WebSocketProvider>();
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



    // / You need to add the `ClientPlugin` first before you can register
    // `ClientMessage`s
    app.add_plugins(bevy_eventwork::EventworkPlugin::<
        WebSocketProvider,
        bevy::tasks::TaskPool,
    >::default());

    // Make sure you insert the EventworkRuntime resource with your chosen Runtime
    app.insert_resource(EventworkRuntime(
        TaskPoolBuilder::new().num_threads(2).build(),
    ));

    // A good way to ensure that you are not forgetting to register
    // any messages is to register them where they are defined!
    // shared::client_register_network_messages(&mut app);

    // app.add_systems(Startup, setup_ui);

    app.add_systems(Startup, handle_connect_startup);
    app.add_systems(Update, handle_message_button);
    app.add_systems(Update, handle_incoming_messages);
    app.add_systems(Update, handle_network_events);

    // app.add_systems(
    //     Update,
    //     (
    //         handle_connect_button,
    //         handle_message_button,
    //         handle_incoming_messages,
    //         handle_network_events,
    //     ),
    // );

    // We have to insert the TCP [`NetworkSettings`] with our chosen settings.
    app.insert_resource(NetworkSettings::default());

    // app.init_resource::<GlobalChatSettings>();

    // app.add_systems(PostUpdate, handle_chat_area);

    // app.run();

    // app.insert_resource(WebSocketSender(Arc::new(Mutex::new(None))));
    // app.insert_resource(ReceivedMessages::default());
    // app.add_systems(Startup, setup_websocket);
    // app.add_systems(Update, handle_messages);
    // app.add_systems(Update, example_system);

    // app.insert_resource(WebSocketResource::default());
    // app.add_systems(Startup, actually_connect);
    // app.add_systems(Update, setup_connection);
    // app.add_systems(Update, handle_tasks);
    // app.add_systems(Update, receive_ws_msg);
}



///////////////////////////////////////////////////////////////
////////////// Incoming Message Handler ///////////////////////
///////////////////////////////////////////////////////////////

fn handle_incoming_messages(
    mut messages: Query<&mut GameChatMessages>,
    mut new_messages: EventReader<NetworkData<NewChatMessage>>,
) {
    let mut messages = messages.get_single_mut().unwrap();

    for new_message in new_messages.read() {
        messages.add(UserMessage::new(&new_message.name, &new_message.message));
    }
}

fn handle_network_events(
    mut new_network_events: EventReader<NetworkEvent>,
    connect_query: Query<&Children, With<ConnectButton>>,
    mut text_query: Query<&mut Text>,
    mut messages: Query<&mut GameChatMessages>,
) {
    let connect_children = connect_query.get_single().unwrap();
    let mut text = text_query.get_mut(connect_children[0]).unwrap();
    let mut messages = messages.get_single_mut().unwrap();

    for event in new_network_events.read() {
        info!("Received event");
        match event {
            NetworkEvent::Connected(_) => {
                messages.add(SystemMessage::new(
                    "Succesfully connected to server!".to_string(),
                ));
                text.sections[0].value = String::from("Disconnect");
            }

            NetworkEvent::Disconnected(_) => {
                messages.add(SystemMessage::new("Disconnected from server!".to_string()));
                text.sections[0].value = String::from("Connect to server");
            }
            NetworkEvent::Error(err) => {
                messages.add(UserMessage::new(String::from("SYSTEM"), err.to_string()));
            }
        }
    }
}

///////////////////////////////////////////////////////////////
////////////// Data Definitions ///////////////////////////////
///////////////////////////////////////////////////////////////

#[derive(Resource)]
struct GlobalChatSettings {
    chat_style: TextStyle,
    author_style: TextStyle,
}

impl FromWorld for GlobalChatSettings {
    fn from_world(_world: &mut World) -> Self {
        GlobalChatSettings {
            chat_style: TextStyle {
                font_size: 20.,
                color: Color::BLACK,
                ..default()
            },
            author_style: TextStyle {
                font_size: 20.,
                color: palettes::css::RED.into(),
                ..default()
            },
        }
    }
}

enum ChatMessage {
    SystemMessage(SystemMessage),
    UserMessage(UserMessage),
}

impl ChatMessage {
    fn get_author(&self) -> String {
        match self {
            ChatMessage::SystemMessage(_) => "SYSTEM".to_string(),
            ChatMessage::UserMessage(UserMessage { user, .. }) => user.clone(),
        }
    }

    fn get_text(&self) -> String {
        match self {
            ChatMessage::SystemMessage(SystemMessage(msg)) => msg.clone(),
            ChatMessage::UserMessage(UserMessage { message, .. }) => message.clone(),
        }
    }
}

impl From<SystemMessage> for ChatMessage {
    fn from(other: SystemMessage) -> ChatMessage {
        ChatMessage::SystemMessage(other)
    }
}

impl From<UserMessage> for ChatMessage {
    fn from(other: UserMessage) -> ChatMessage {
        ChatMessage::UserMessage(other)
    }
}

struct SystemMessage(String);

impl SystemMessage {
    fn new<T: Into<String>>(msg: T) -> SystemMessage {
        Self(msg.into())
    }
}

#[derive(Component)]
struct UserMessage {
    user: String,
    message: String,
}

impl UserMessage {
    fn new<U: Into<String>, M: Into<String>>(user: U, message: M) -> Self {
        UserMessage {
            user: user.into(),
            message: message.into(),
        }
    }
}

#[derive(Component)]
struct ChatMessages<T> {
    messages: Vec<T>,
}

impl<T> ChatMessages<T> {
    fn new() -> Self {
        ChatMessages { messages: vec![] }
    }

    fn add<K: Into<T>>(&mut self, msg: K) {
        let msg = msg.into();
        self.messages.push(msg);
    }
}

type GameChatMessages = ChatMessages<ChatMessage>;

///////////////////////////////////////////////////////////////
////////////// UI Definitions/Handlers ////////////////////////
///////////////////////////////////////////////////////////////

#[derive(Component)]
struct ConnectButton;

fn handle_connect_startup(
    net: ResMut<Network<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    // interaction_query: Query<
    //     (&Interaction, &Children),
    //     (Changed<Interaction>, With<ConnectButton>),
    // >,
    // mut text_query: Query<&mut Text>,
    // mut messages: Query<&mut GameChatMessages>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    // let mut messages = if let Ok(messages) = messages.get_single_mut() {
    //     messages
    // } else {
    //     return;
    // };

    // for (interaction, children) in interaction_query.iter() {
    //     let mut text = text_query.get_mut(children[0]).unwrap();
    //     if let Interaction::Pressed = interaction {


    println!("Trying to connect new ");

            if net.has_connections() {
                net.disconnect(ConnectionId { id: 0 })
                    .expect("Couldn't disconnect from server!");
            } else {
                // text.sections[0].value = String::from("Connecting...");
                // messages.add(SystemMessage::new("Connecting to server..."));

                net.connect(
                    url::Url::parse("ws://127.0.0.1:8000").unwrap(),
                    &task_pool.0,
                    &settings,
                );


            }
        // }
    // }
}

#[derive(Component)]
struct MessageButton;

fn handle_message_button(
    net: Res<Network<WebSocketProvider>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MessageButton>)>,
    mut messages: Query<&mut GameChatMessages>,
) {
    let mut messages = if let Ok(messages) = messages.get_single_mut() {
        messages
    } else {
        return;
    };

    for interaction in interaction_query.iter() {
        if let Interaction::Pressed = interaction {
            match net.send_message(
                ConnectionId { id: 0 },
                UserChatMessage {
                    message: String::from("Hello there!"),
                },
            ) {
                Ok(()) => (),
                Err(err) => messages.add(SystemMessage::new(format!(
                    "Could not send message: {}",
                    err
                ))),
            }
        }
    }
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

// #[derive(Default, Resource)]
// struct WebSocketResource {
//     sender: Option<Arc<Mutex<SplitSink<WebSocketStream, tokio_tungstenite_wasm::Message>>>>,
// }

// Resource to store the WebSocket sender
// #[derive(Resource)]
// struct WebSocketSender(Arc<Mutex<Option<futures_util::stream::SplitSink<tokio_tungstenite_wasm::WebSocketStream, Message>>>>);


// Resource to store received messages
#[derive(Resource, Default)]
struct ReceivedMessages(Vec<String>);

// fn setup_websocket(mut commands: Commands) {
//     wasm_bindgen_futures::spawn_local(async move {
//         let ws = connect("wss://echo.websocket.org/").await.unwrap();
//         let (sender, mut receiver) = ws.split();
        
//         commands.insert_resource(WebSocketSender(Arc::new(Mutex::new(Some(sender)))));

//         while let Some(msg) = receiver.next().await {
//             if let Ok(msg) = msg {
//                 if let Ok(text) = msg.into_text() {
//                     commands.insert_resource(ReceivedMessages(vec![text]));
//                 }
//             }
//         }
//     });
// }

// fn handle_messages(mut received_messages: ResMut<ReceivedMessages>) {
//     for message in received_messages.0.drain(..) {
//         println!("Received message: {}", message);
//     }
// }

// fn example_system(websocket_sender: Res<WebSocketSender>) {
//     // if let Some(sender) = websocket_sender.0.lock().unwrap() {
//         // Example of sending a message from another system
//         // let message = "Hello from Bevy!";
//         // let sender_clone = sender.clone();
//         // tokio::spawn(async move {
//         //     sender_clone.send(Message::Text(message.to_string())).await.unwrap();
//         // });
//     // }
// }








// WebSocket connection states
// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// enum WebSocketState {
//     Disconnected,
//     Connected,
// }

// // Resource to store the WebSocket sender
// #[derive(Resource)]
// struct WebSocketSender(Arc<Mutex<Option<futures_util::stream::SplitSink<tokio_tungstenite_wasm::WebSocketStream, Message>>>>);

// // Resource to store the WebSocket connection
// #[derive(Resource)]
// struct WebSocketConnection {
//     sender: Arc<Mutex<Option<futures_util::stream::SplitSink<tokio_tungstenite_wasm::WebSocketStream, Message>>>>,
//     receiver: Arc<Mutex<Option<futures_util::stream::SplitStream<tokio_tungstenite_wasm::WebSocketStream>>>>,
// }


// // Event for received messages
// #[derive(Event)]
// struct WebSocketMessage(String);


// fn setup_websocket(mut commands: Commands) {
//     // Note: This is a blocking operation. In a real-world scenario, you'd want to handle this asynchronously.
//     let (ws_stream, _) = futures::executor::block_on(connect("wss://echo.websocket.org/")).unwrap();
//     let (sender, receiver) = ws_stream.split();
    
//     commands.insert_resource(WebSocketConnection {
//         sender: Arc::new(Mutex::new(Some(sender))),
//         receiver: Arc::new(Mutex::new(Some(receiver))),
//     });
// }

// fn handle_messages(
//     mut connection: ResMut<WebSocketConnection>,
//     mut received_messages: ResMut<ReceivedMessages>
// ) {
//     if let Some(receiver) = &mut *connection.receiver.lock().unwrap() {
//         // Note: This is a blocking operation. In a real-world scenario, you'd want to handle this asynchronously.
//         if let Some(message) = futures::executor::block_on(receiver.next()) {
//             if let Ok(message) = message {
//                 if let Ok(text) = message.into_text() {
//                     received_messages.0.push(text);
//                 }
//             }
//         }
//     }
// }

// fn example_system(connection: Res<WebSocketConnection>) {
//     if let Some(sender) = &*connection.sender.lock().unwrap() {
//         let message = "Hello from Bevy!";
//         // Note: This is a blocking operation. In a real-world scenario, you'd want to handle this asynchronously.
//         futures::executor::block_on(async {
//             sender.send(Message::Text(message.to_string())).await.unwrap();
//         });
//     }
// }

// fn example_system(sender: Res<WebSocketSender>) {
//     if let Some(sender) = &*sender.0.lock().unwrap() {
//         let message = "Hello from Bevy!";
//         // This is a simplified example. In a real-world scenario, you'd want to handle this asynchronously
//         futures::executor::block_on(async {
//             sender.send(Message::Text(message.to_string())).await.unwrap();
//         });
//     }
// }

fn connect_wasm(// _input: Res<ButtonInput<KeyCode>>,
    // mut web_socket_resource: ResMut<WebSocketResource>,
) {
    // ev_connect.send(WebSocketConnectionEvents::SetupConnection);
    // console::log_1(&"Connecting *(*(*(*( !".into());
    spawn_local(async move {
        log::info!("Connecting");
        let ws = tokio_tungstenite_wasm::connect("ws://0.0.0.0:8000/ws")
            .await
            .unwrap();
        let (sender, mut receiver) = ws.split();

        // let sender_clone = sender.clone();

        // Lock the sender and await for the lock to be acquired
        // match sender.lock().await {
        //     Ok(mut sender) => { // On success, we have a mutable reference to the SplitSink
        //         // Attempt to send the message
        //         if let Err(e) = sender.send(tokio_tungstenite_wasm::Message::text("foo")).await {
        //             log::error!("Failed to send message: {:?}", e);
        //         }
        //     },
        //     Err(e) => {
        //         log::error!("Failed to lock the sender: {:?}", e);
        //     }
        // }

        log::warn!("Connected!");

        // // Wrap the sender in Arc<Mutex<>> and store it in the resource
        // web_socket_resource.sender = Some(Arc::new(Mutex::new(sender)));

        // let msg = receiver.next().await.unwrap().unwrap();
        // log::info!("Received a message: {:?}", msg);

        // let payload = "This is a test message.";
        
        // // Send the message after acquiring the lock
        // {
        //     // let sender = web_socket_resource.sender.as_ref().unwrap().clone();
        //     // let mut sender = sender.lock().await.; // Lock to access the sender
        //     // sender.send(tokio_tungstenite_wasm::Message::text(payload)).await.unwrap();

        //     match sender.lock().await {
        //         Ok(mut sender) => {
        //             // Attempt to send the message
        //             if let Err(e) = sender.send(tokio_tungstenite_wasm::Message::text("foo")).await {
        //                 log::error!("Failed to send message: {:?}", e);
        //             }
        //         },
        //         Err(e) => {
        //             log::error!("Failed to lock the sender: {:?}", e);
        //         }
        //     }
        // }

        // log::info!("Sent payload to echo server. Waiting for response...");

        let msg = receiver.next().await.unwrap().unwrap();
        log::info!("Received and validated response.");
    });
}

// fn send_message(
//     mut web_socket_resource: ResMut<WebSocketResource>, // Access the resource
// ) {
//     if let Some(sender) = &mut web_socket_resource.sender {
//         let payload = "Another test message.";
        
//         // Use tokio's spawn_local to send messages asynchronously
//         spawn_local(async move {
//             if let Err(e) = sender.send(tokio_tungstenite_wasm::Message::text(payload)).await {
//                 log::error!("Failed to send message: {:?}", e);
//             }
//         });
//     } else {
//         log::warn!("WebSocket sender is not initialized.");
//     }
// }

// fn actually_connect(
//     _input: Res<ButtonInput<KeyCode>>,
//     mut ev_connect: EventWriter<WebSocketConnectionEvents>,
// ) {
//     // ev_connect.send(WebSocketConnectionEvents::SetupConnection);
// }

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
