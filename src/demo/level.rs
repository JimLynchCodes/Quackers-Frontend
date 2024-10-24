// Yeah, basically not even using this file anymore, but it 
// is kinda nice to have this example of a custom command
// bc tbh I still don't really understand when to use those...

use bevy::{app::{App, Startup, Update}, prelude::{Commands, EventReader, PerspectiveProjection, Query, World}, utils::info, window::WindowResized};

pub(super) fn plugin(_app: &mut App) {
    // _app.add_systems(Startup, init_camera_ratio);
    _app.add_systems(Update, adjust_camera_ratio);

}

/// A [`Command`] to spawn the level.s
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(_world: &mut World) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    
    // SpawnPlayer { max_speed: 400.0 }.apply(world);
}

// pub fn init_camera_ratio(mut commands: Commands) {
//         commands.spawn_bundle(PerspectiveCameraBundle {
//             perspective_projection: PerspectiveProjection {
//                 aspect_ratio: 16.0 / 9.0, // Initial aspect ratio
//                 ..Default::default()
//             },
//             ..Default::default()
//         });
    
// }

pub fn adjust_camera_ratio(
        mut resize_events: EventReader<WindowResized>,
        mut query: Query<&mut PerspectiveProjection>
    ) {
        for event in resize_events.read() {
            for mut projection in query.iter_mut() {

                println!("adjusting camera");
                projection.aspect_ratio = event.width / event.height;
                
            }
        }
    }

