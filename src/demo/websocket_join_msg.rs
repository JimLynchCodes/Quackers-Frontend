use std::io::ErrorKind;

use bevy::
    prelude::*
;
use serde::{Deserialize, Serialize};

use super::websocket_connect::WebSocketClient;
// use tungstenite::Message;

#[derive(Serialize, Deserialize)]
struct TransformData {
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
}

pub(super) fn plugin(app: &mut App) {

    app.add_event::<JoinRequestEvent>();

    // app.add_systems(Update, join_request_bevy_event_listener);
}

// use super::websocket_connect::WebSocketClient;

#[derive(Event)]
pub struct JoinRequestEvent(pub String);

// Listens for bevy events for ws messages and fires them off to the server
// fn join_request_bevy_event_listener(
//     mut ev_join_request: EventReader<JoinRequestEvent>,
//     mut entities_with_client: Query<(&mut WebSocketClient,)>,
// ) {
//     for ev in ev_join_request.read() {
//         println!("heard join request bevy event");
//         for mut client in entities_with_client.iter_mut() {
//             println!("sending join request ws msg");
//             let message = build_join_request_msg(ev.0.clone());

//             match client.0 .0 .0.send(Message::text(message)) {
//                 Ok(_) => info!("Join request ws msg successfully sent to server!"),
//                 Err(tungstenite::Error::Io(e)) if e.kind() == ErrorKind::WouldBlock => { /* ignore */
//                 }
//                 Err(e) => {
//                     warn!("Could not send the message: {e:?}");
//                 }
//             }
//         }
//     }
// }

#[derive(serde::Serialize)]
struct JoinRequestData {
    friendly_name: String,
}

#[derive(serde::Serialize)]
struct JoinRequest {
    action_type: String,
    data: JoinRequestData,
}

pub fn build_join_request_msg(friendly_name: String) -> String {
    let join_request_hardcoded = JoinRequest {
        action_type: "join".to_string(),
        data: JoinRequestData {
            friendly_name: friendly_name,
        },
    };

    let g = serde_json::ser::to_string(&join_request_hardcoded).unwrap_or_else(|_op| {
        println!("Couldn't convert You Quacked struct to string");
        "".to_string()
    });
    g
}
