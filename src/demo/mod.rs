use bevy::prelude::*;

pub mod level;
mod movement;
pub mod player;
pub mod player_animation;
pub mod other_player;
pub mod other_player_animation;
pub mod cracker;
pub mod score;
pub mod background;
pub mod websocket_connect;
pub mod websocket_join_msg;
pub mod websocket_move_msg;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        movement::plugin,
        level::plugin,
        player::plugin,
        player_animation::plugin,
        other_player::plugin,
        other_player_animation::plugin,
        cracker::plugin,
        score::plugin,
        background::plugin,
        websocket_connect::plugin,
        websocket_join_msg::plugin,
        websocket_move_msg::plugin,
    ));
}
