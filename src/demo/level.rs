// Yeah, basically not even using this file anymore, but it
// is kinda nice to have this example of a custom command
// bc tbh I still don't really understand when to use those...

use std::sync::Arc;

use bevy::{
    app::{App, Startup, Update}, asset::AssetServer, prelude::{Res, Commands, Component, Entity, EventReader, PerspectiveProjection, Query, ResMut, Resource, With, World}, utils::info, window::WindowResized
};

use futures::lock::Mutex;
use wasm_bindgen_futures::spawn_local;

use super::check_silent_mode;

pub(super) fn plugin(_app: &mut App) {
    // _app.add_systems(Startup, init_camera_ratio);
    _app.add_systems(Update, adjust_camera_ratio);

    _app.add_systems(Startup, check_for_silent_mode_once);
    // _app.add_systems(Update, display_popup_if_silent);
    // _app.add_systems(Update, display_popup_if_silent.system());
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
    mut query: Query<&mut PerspectiveProjection>,
) {
    for event in resize_events.read() {
        for mut projection in query.iter_mut() {
            println!("adjusting camera");
            projection.aspect_ratio = event.width / event.height;
        }
    }
}

// This resource will store the silent mode status
#[derive(Default, Resource)]
pub struct SilentModeStatus {
    // is_silent: Option<bool>,
    is_silent: Arc<Mutex<Option<bool>>>,
}

// fn check_for_silent_mode_once(mut silent_mode_status: ResMut<SilentModeStatus>) {
//     spawn_local(async move {
//         let is_silent = check_silent_mode::check_silent_mode().await;
//         silent_mode_status.is_silent = Some(is_silent);
//     });
// }

fn check_for_silent_mode_once(silent_mode_status: Res<SilentModeStatus>) {
    let shared_status = silent_mode_status.is_silent.clone();
    spawn_local(check_silent_mode::check_silent_mode(shared_status));
}

#[derive(Component)]
struct SilentModePopup;

// fn display_popup_if_silent(
//     mut commands: Commands,
//     silent_mode_status: Res<SilentModeStatus>,
//     query: Query<Entity, With<SilentModePopup>>,
//     asset_server: Res<AssetServer>,
// ) {
//     // Lock the shared status to access the silent mode value
//     let is_silent = *silent_mode_status.is_silent.lock();

//     if let Some(is_silent) = is_silent {
//         // if is_silent && query.is_empty() {
//         //     commands.spawn_bundle(TextBundle {
//         //         style: Style {
//         //             margin: Rect::all(Val::Px(5.0)),
//         //             ..Default::default()
//         //         },
//         //         text: Text::with_section(
//         //             "Silent Mode is ON!",
//         //             TextStyle {
//         //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//         //                 font_size: 30.0,
//         //                 color: Color::WHITE,
//         //             },
//         //             Default::default(),
//         //         ),
//         //         ..Default::default()
//         //     })
//         //     .insert(SilentModePopup);
//         // }
//     }
// }

// #[derive(Component)]
// struct SilentModePopup;

// fn display_popup_if_silent(
//     mut commands: Commands,
//     silent_mode_status: Res<SilentModeStatus>,
//     query: Query<Entity, With<SilentModePopup>>,
//     asset_server: Res<AssetServer>,
// ) {
//     if let Some(is_silent) = silent_mode_status.is_silent {
//         if is_silent && query.is_empty() {
//             commands
//                 .spawn_bundle(TextBundle {
//                     style: Style {
//                         margin: Rect::all(Val::Px(5.0)),
//                         ..Default::default()
//                     },
//                     text: Text::with_section(
//                         "Silent Mode is ON!",
//                         TextStyle {
//                             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                             font_size: 30.0,
//                             color: Color::WHITE,
//                         },
//                         Default::default(),
//                     ),
//                     ..Default::default()
//                 })
//                 .insert(SilentModePopup);
//         }
//     }
// }
