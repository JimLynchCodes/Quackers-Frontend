use bevy::color::palettes::css::{LIME, RED};
use bevy::prelude::*;
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use virtual_joystick::{
    create_joystick, JoystickFloating, JoystickInvisible, NoAction, VirtualJoystickEvent,
    VirtualJoystickPlugin,
};
// use bevy::ui::{UiPlugin};
// use bevy::prelude::*;

use crate::theme::palette::{BUTTON_TEXT, NODE_BACKGROUND};
use crate::theme::widgets::Containers;

use crate::demo::other_player::{unpack_duck_color, NewJoinerDataWithAllPlayers};
use crate::{
    asset_tracking::LoadResource,
    demo::{movement::MovementController, player_animation::PlayerAnimation},
    screens::Screen,
};

use super::websocket_connect::{
    MoveCrackersBevyEvent, OtherPlayerJoinedWsReceived, WebSocketClient, YouJoinedWsReceived
};

#[derive(Event)]
pub struct QuackRequestEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<QuackRequestEvent>();
    app.add_systems(Update, quack_request_bevy_event_listener);
}

fn quack_request_bevy_event_listener(
    mut ev_join_request: EventReader<QuackRequestEvent>,
    mut entities_with_client: Query<(&mut WebSocketClient,)>,
) {

    for ev in ev_join_request.read() {
        println!("heard quack request bevy event");
        
        for mut client in entities_with_client.iter_mut() {
            println!("sending quack request ws msg");
            let message = build_quack_request_msg();

            #[cfg(not(target_arch = "wasm32"))]
            {
                match client.0 .0 .0.send(tungstenite::Message::text(message)) {
                    Ok(_) => info!("Join request ws msg successfully sent to server!"),
                    Err(tungstenite::Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => { /* ignore */
                    }
                    Err(e) => {
                        warn!("Could not send the message: {e:?}");
                    }
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                match client.0.0.socket.send_with_str(&message) {
                    Ok(_) => web_sys::console::log_1(&"Message sent successfully".into()),
                    Err(err) => {
                        web_sys::console::log_1(&format!("Error sending message: {:?}", err).into())
                    }
                }
            }
        }
    }
}


#[derive(serde::Serialize)]
struct QuackRequestData {

}

#[derive(serde::Serialize)]
struct QuackRequest {
    action_type: String,
    data: QuackRequestData,
}

fn build_quack_request_msg() -> String {
    let join_request_hardcoded = QuackRequest {
        action_type: "quack".to_string(),
        data: QuackRequestData {},
    };

    serde_json::ser::to_string(&join_request_hardcoded).unwrap_or_else(|_op| {
        println!("Couldn't convert quack request struct to string");
        "".to_string()
    })
}