use bevy::prelude::*;
use bevy::log::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(LogPlugin {
            filter: "error,bevy_jam_6=trace".to_string(),
            level: Level::TRACE,
            ..Default::default()
        }))
    .add_systems(Startup, load_sprite)
    .add_systems(FixedUpdate, controls)
    .run();

    //TODO Pre game start load all spritesheets
    //TODO figure out how to read spritesheets
}

fn load_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn(Camera2d);

    commands.spawn(Sprite::from_image(
        asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_001.png"),
    ));    
}

fn controls(
    input: Res<ButtonInput<MouseButton>>,
    cursor_evr: EventReader<CursorMoved>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
)
{
    if input.just_pressed(MouseButton::Left) {
        trace!("Left pressed");

        let window = windows.single().ok().unwrap();
        let (camera, camera_transform) = camera_q.single().ok().unwrap();

        if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
             trace!("World coords: {}/{}", world_position.x, world_position.y);
        }
    }

    if input.just_pressed(MouseButton::Right) {
        debug!("Right release")
    }

    if input.just_released(MouseButton::Left) {
        trace!("Left release")
    }

    if input.just_released(MouseButton::Right) {
        trace!("Right release")
    }

    /* 
    keeping this for reading, but don't actually need it
    it prints mouse cursor in pixel coordinates, with 0,0 being top left
    these coordinates differ from world coordinates, where 0,0 is screen center (with default 2d camera)
    for event in cursor_evr.read() {
        trace!(
            "new cursor position: x: {}, y: {}",
            event.position.x, event.position.y
        );
    }
     */
}

#[derive(Component)]
struct Rune;