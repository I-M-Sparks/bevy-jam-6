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
        .add_systems(FixedUpdate, controls)
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

fn controls(
    input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    pickables: Query<(&mut Transform, &Sprite, Entity), (With<Pickable>, Without<Picked>)>,
    picked: Option<Single<(&mut Transform, Entity), With<Picked>>>,
    mut cursor_evr: EventReader<CursorMoved>,
    images: Res<Assets<Image>>,
    mut commands: Commands,
) {
    if input.just_pressed(MouseButton::Left) && picked.is_none() {
        trace!("Left pressed");

        let window = windows.single().ok().unwrap();
        let (camera, camera_transform) = camera_q.single().ok().unwrap();

        if let Some(cursor_world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            trace!(
                "Clicked at: {}/{}",
                cursor_world_position.x, cursor_world_position.y
            );
            //react to click if inside Pickable Sprite
            for (transform, sprite, entity) in pickables {
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
    /*
       if input.just_pressed(MouseButton::Right) {
           debug!("Right release")
       }
    */

    /*
     if input.just_released(MouseButton::Right) {
         trace!("Right release")
     }

    */

    /*
    Turns out I need this after all :D

    I query for a single entity with the "Picked" Component (there should only ever be one) and set it's transform to the cursor position in world coordinates
     */
    if picked.is_some() {
        let picked_single = picked.unwrap();
        let (mut transform, entity) = picked_single.into_inner();

        if input.just_released(MouseButton::Left) {
            trace!("Release Picked entity");

            // drop the Pickable
            commands.entity(entity).remove::<Picked>();
        } else {
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
    }
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
