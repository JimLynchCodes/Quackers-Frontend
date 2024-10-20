// Yeah, basically not even using this file anymore, but it 
// is kinda nice to have this example of a custom command
// bc tbh I still don't really understand when to use those...

use bevy::{app::App, prelude::World};

pub(super) fn plugin(_app: &mut App) {

}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(_world: &mut World) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    
    // SpawnPlayer { max_speed: 400.0 }.apply(world);
}
