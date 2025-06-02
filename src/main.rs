use bevy::log::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "error,bevy_jam_6=trace".to_string(),
            level: Level::TRACE,
            ..Default::default()
        }))
        .add_systems(Startup, setup_mvp_scene)
        .add_systems(
            Update,
            (move_picked_object, handle_pick_event, handle_release_event),
        )
        .add_systems(FixedUpdate, controls)
        .add_event::<PickEvent>()
        .add_event::<ReleaseEvent>()
        .run();

    //TODO Pre game start load all spritesheets
    //TODO figure out how to read spritesheets
}

/*
========================================================================================
Constants
========================================================================================
*/
// define z values for various assets; creates layers through z-components of transform
// this ensures a correct drawing order for sprites
// note: z value of 0 should be set for the background; objects drawn "further in front" must have a HIGHER value
const RUNE_RENDER_LAYER: f32 = 10.0;

/*
========================================================================================
Systems
========================================================================================
 */

/*
spawns all runes, objects etc. for the MVP scene
explanation: MVP - Minimum Viable product
means a scene that shows all basic gameplay elements which is loaded by default during development

after finishing the MVP scene I can consider doing more than one scene
*/
fn setup_mvp_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    //spawn a rune
    commands.spawn((
        Rune,
        Pickable,
        //FOR DEBUGGING
        //Picked,
        Transform::from_xyz(100.0, 100.0, RUNE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_001.png")),
    ));

    //spawn a second rune
    commands.spawn((
        Rune,
        Pickable,
        Transform::from_xyz(-100.0, -100.0, RUNE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_002.png")),
    ));

    // TODO spawn a rune slot

    // TODO more things

    info!("Setup completed");
}

/*
Queries the currently moved object and moves it to the cursor position
 */
fn move_picked_object(
    picked: Single<&mut Transform, With<Picked>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut cursor_evr: EventReader<CursorMoved>,
) {
    let mut transform = picked.into_inner();

    for event in cursor_evr.read() {
        trace!(
            "new cursor position: x: {}, y: {}",
            event.position.x, event.position.y
        );

        let (camera, camera_transform) = camera_q.single().ok().unwrap();
        let cursor_position_in_world_coord = camera
            .viewport_to_world_2d(
                camera_transform,
                Vec2::new(event.position.x, event.position.y),
            )
            .ok()
            .unwrap();

        transform.translation.x = cursor_position_in_world_coord.x;
        transform.translation.y = cursor_position_in_world_coord.y;
    }
}

/*
Handles whichever action caused a Pick-Event
 */
fn handle_pick_event(
    mut pick_event_reader: EventReader<PickEvent>,
    mut commands: Commands,
    images: Res<Assets<Image>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    pickables: Query<(Entity, &mut Transform, &Sprite), (With<Pickable>, Without<Picked>)>,
) {
    trace!("Pick event");

    let (camera, camera_transform) = camera_q.single().ok().unwrap();

    for pick_event in pick_event_reader.read() {
        if let Some(cursor_world_position) = camera
            .viewport_to_world_2d(camera_transform, pick_event.location_in_screen_coordinates)
            .ok()
        {
            trace!(
                "Clicked at: {}/{}",
                cursor_world_position.x, cursor_world_position.y
            );

            //react to click if inside Pickable Sprite
            for (entity, transform, sprite) in pickables {
                /*
                IF mouse click is inside pickable
                THEN add Picked-component to that entity AND interrupt loop
                ELSE do nothing

                note: bevy probably has a more elegant way to do this but I didn't find it immediately and won't waste Jam-time searching for it
                note note: it seems bevy currently has no elegant way to find the actual sprite size, so I need to use this monstrosity
                 */
                let sprite_size = if let Some(custom_size) = sprite.custom_size {
                    custom_size
                } else if let Some(image) = images.get(sprite.image.id()) {
                    image.size_f32()
                } else {
                    Vec2::new(1.0, 1.0)
                };

                let sprite_size = sprite_size * transform.scale.truncate();
                trace!("sprite size {:?}", sprite_size);

                // note: this is assuming the Sprite-Anchor is CENTER
                if cursor_world_position.x > transform.translation.x - sprite_size.x * 0.5
                    && cursor_world_position.x < transform.translation.x + sprite_size.x * 0.5
                    && cursor_world_position.y > transform.translation.y - sprite_size.y * 0.5
                    && cursor_world_position.y < transform.translation.y + sprite_size.y * 0.5
                {
                    commands.entity(entity).insert(Picked);
                    break;
                }
            }
        }
    }
}

/*
Handles the release of whichever Action caused a previous pick event

Release event is ignored if no Picked object was found
 */
fn handle_release_event(
    mut commands: Commands,
    mut release_event_reader: EventReader<ReleaseEvent>,
    picked: Single<(Entity, &mut Transform), With<Picked>>,
) {
    trace!("Release event processing");

    let (entity, transform) = picked.into_inner();

    // "drop" the Pickable by removing the Picked-component
    commands.entity(entity).remove::<Picked>();

    //assumption: whatever object was picked has been moved to wherever the cursor had been
    //-> the objects transform is ready for further usage

    // TODO check for overlap between picked entity and potential slot -> how to best do this?
    // read screen-position from event
    // translate to world position
    // check for overlap with all 'slot' entities
}

fn controls(
    input: Res<ButtonInput<MouseButton>>,
    mut release_event_writer: EventWriter<ReleaseEvent>,
    mut pick_event_writer: EventWriter<PickEvent>,
    windows: Query<&Window>,
) {
    // Press Left Mouse
    if input.just_pressed(MouseButton::Left) {
        trace!("Left pressed");

        let window = windows.single().ok().unwrap();

        pick_event_writer.write(PickEvent {
            location_in_screen_coordinates: window.cursor_position().unwrap(),
        });
    }

    // Release Left Mouse
    if input.just_released(MouseButton::Left) {
        trace!("Release Event triggered");

        let window = windows.single().ok().unwrap();

        release_event_writer.write(ReleaseEvent {
            location_in_screen_coordinates: window.cursor_position().unwrap(),
        });
    }

    /*
    // Press Right Mouse
       if input.just_pressed(MouseButton::Right) {
           debug!("Right release")
       }
    */

    /*
    // Press Left Mouse
     if input.just_released(MouseButton::Right) {
         trace!("Right release")
     }
    */
}

/*
========================================================================================
Components
========================================================================================
 */

#[derive(Component)]
struct Rune;

#[derive(Component)]
struct Pickable;

#[derive(Component)]
struct Picked;

/*
========================================================================================
Events
========================================================================================
 */

#[derive(Event)]
struct PickEvent {
    // screen coordinates means (0,0) to (window_width, window_height); from top-left to bottom-right
    location_in_screen_coordinates: Vec2,
}

#[derive(Event)]
struct ReleaseEvent {
    // screen coordinates means (0,0) to (window_width, window_height); from top-left to bottom-right
    location_in_screen_coordinates: Vec2,
}

/*
========================================================================================
Enums
========================================================================================
 */
