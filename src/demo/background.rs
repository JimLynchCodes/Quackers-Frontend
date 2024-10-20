use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, create_background);
}

fn create_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("images/ducks-bg-test.png");

    println!("creating background...");

    // Spawn an entity with the background image as a sprite
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform {
            scale: Vec3::new(1.0, 1.0, 0.0),
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
