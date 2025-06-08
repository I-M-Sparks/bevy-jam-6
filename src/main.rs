use std::{thread::sleep, time::Duration};

use avian2d::{math, prelude::*};
use bevy::{ecs::system::command, log::*, prelude::*};

fn main() {
    App::new()
        // DEfault Plugin
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "error,bevy_jam_6=trace".to_string(),
            level: Level::TRACE,
            ..Default::default()
        }))
        // Add Default Physics
        // length unit 100 => 1m = 1 pixels.
        .add_plugins(PhysicsPlugins::default().with_length_unit(1.0))
        // Debug physics
        .add_plugins(PhysicsDebugPlugin::default())
        // Startup
        .add_systems(Startup, setup_mvp_scene)
        // Input handling
        .add_systems(
            PreUpdate,
            (move_picked_object, handle_pick_event, handle_release_event),
        )
        // Collision handling
        .add_systems(PostUpdate, handle_collision_ball_with_ball_firing_thingy)
        // Input forwarding
        .add_systems(FixedUpdate, controls)
        // Add colliders to sprites
        .add_systems(Last, add_colliders)
        //Events
        .add_event::<PickEvent>()
        .add_event::<ReleaseEvent>()
        // Run
        .run();
}

/*
========================================================================================
Constants
========================================================================================
*/
// define z values for various assets; creates layers through z-components of transform
// this ensures a correct drawing order for sprites
// note: z value of 0 should be set for the background; objects drawn "further in front" must have a HIGHER value

const RUNE_MACHINE_RENDER_LAYER: f32 = 50.0;
// Rune-slots are child-entities of rune machine parts, so they are rendered on top of them
const RUNE_SLOT_RENDER_LAYER: f32 = 1.0;

const DONUT_CIRCLE_RENDER_LAYER: f32 = 75.0;
const DONUT_BASE_RENDER_LAYER: f32 = 76.0;
const DONUT_FROSTING_RENDER_LAYER: f32 = 77.0;
const DONUT_SPRINKLES_RENDER_LAYER: f32 = 78.0;

const BALL_FIRING_THINGY_RENDER_LAYER: f32 = 99.0;

const RUNE_RENDER_LAYER: f32 = 100.0;
const BALL_RENDER_LAYER: f32 = 101.0;

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
fn setup_mvp_scene(
    // Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    /*
    =========================================================================================================
    spawn meta entities
    =========================================================================================================
     */
    // spawn player entity
    commands.spawn((Player, PlayerCanPick));

    // spawn camera
    commands.spawn(Camera2d);

    /*
    =========================================================================================================
    spawn runes
    =========================================================================================================
     */

    //spawn a rune
    commands.spawn((
        Rune,
        Pickable,
        Transform::from_xyz(-600.0, -300.0, RUNE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_001.png")),
        AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::Rectangle,
        },
    ));

    //spawn a second rune
    commands.spawn((
        Rune,
        Pickable,
        Transform::from_xyz(-540.0, -300.0, RUNE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_002.png")),
        AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::Rectangle,
        },
    ));

    /*
    =========================================================================================================
    Spawn starter-balls
    =========================================================================================================
     */

    // Blue ball
    let blue_ball_sprite =
        Sprite::from_image(asset_server.load("Puzzle Assets/PNG/Double/ballBlue.png"));

    let blue_ball_default_position = Vec2::new(-540.0, -200.0);

    let blue_ball_transform = Transform::from_xyz(
        blue_ball_default_position.x,
        blue_ball_default_position.y,
        BALL_RENDER_LAYER,
    );

    commands.spawn((
        BlueBall {
            default_position: blue_ball_default_position.clone(),
        },
        blue_ball_sprite,
        blue_ball_transform,
        AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::Circle,
        },
        Pickable,
        RigidBody::Kinematic,
    ));

    // Grey balls
    let grey_ball_sprite =
        Sprite::from_image(asset_server.load("Puzzle Assets/PNG/Double/ballGrey.png"));

    let mut grey_ball_default_position = Vec2::new(-490.0, -200.0);

    commands.spawn((
        GreyBall {
            default_position: grey_ball_default_position.clone(),
        },
        grey_ball_sprite.clone(),
        Transform::from_xyz(
            grey_ball_default_position.x,
            grey_ball_default_position.y,
            BALL_RENDER_LAYER,
        ),
        RigidBody::Kinematic,
    ));

    grey_ball_default_position = Vec2::new(-440.0, -200.0);

    commands.spawn((
        GreyBall {
            default_position: grey_ball_default_position.clone(),
        },
        grey_ball_sprite.clone(),
        Transform::from_xyz(
            grey_ball_default_position.x,
            grey_ball_default_position.y,
            BALL_RENDER_LAYER,
        ),
        Pickable,
        RigidBody::Kinematic,
    ));

    // Ball firing Thingy
    let ball_firing_thingy_sprite = Sprite::from_image(
        asset_server.load("UI Pack/PNG/Grey/Double/check_round_grey_circle.png"),
    );
    let ball_firing_thingy_transform =
        Transform::from_xyz(500.0, -150.0, BALL_FIRING_THINGY_RENDER_LAYER);

    // note: default direction of BallFiringThingy is to the left
    commands
        .spawn((
            BallFiringThingy {
                // speed is units per second; see addition of physics plugin to determine how much that is in pixels
                firing_direction: Vec2::new(-100.0, 0.0),
            },
            ball_firing_thingy_sprite,
            ball_firing_thingy_transform,
            AddCollider {
                collider_scale: 0.3,
                collider_type: ColliderType::Circle,
            },
        ))
        .with_children(|parent| {
            // spawn arrow of ball firing thingy
            parent.spawn((
                Sprite::from_image(
                    asset_server.load("UI Pack/PNG/Grey/Double/arrow_decorative_w.png"),
                ),
                Transform::from_xyz(-64.0, 0.0, 0.0),
            ));
        });

    /*
    =========================================================================================================
    spawn target donut
    =========================================================================================================
     */

    // spawn target donut presentation circle of mystic holyness
    let mut donut_circle_sprite =
        Sprite::from_image(asset_server.load("Particle Pack/PNG (Black Background)/magic_02.png"));
    donut_circle_sprite.custom_size = Some(Vec2::new(200.0, 200.0));

    commands.spawn((
        DonutCircle,
        Transform::from_xyz(540.0, 260.0, DONUT_CIRCLE_RENDER_LAYER),
        donut_circle_sprite,
    ));

    // spawn target donut base
    let mut donut_base_sprite = Sprite::from_image(asset_server.load("Donuts/PNG/donut_1.png"));
    donut_base_sprite.custom_size = Some(Vec2::new(80.0, 80.0));

    commands.spawn((
        DonutCircle,
        Transform::from_xyz(540.0, 260.0, DONUT_BASE_RENDER_LAYER),
        donut_base_sprite,
    ));

    // spawn target donut frosting
    let mut donut_frosting_sprite =
        Sprite::from_image(asset_server.load("Donuts/PNG/glazing_5.png"));
    donut_frosting_sprite.custom_size = Some(Vec2::new(70.0, 70.0));

    commands.spawn((
        DonutCircle,
        Transform::from_xyz(540.0, 260.0, DONUT_FROSTING_RENDER_LAYER),
        donut_frosting_sprite,
    ));

    // spawn target donut sprinkles
    let mut donut_sprinkles_sprite =
        Sprite::from_image(asset_server.load("Donuts/PNG/sprinkles_1.png"));
    donut_sprinkles_sprite.custom_size = Some(Vec2::new(70.0, 70.0));

    commands.spawn((
        DonutCircle,
        Transform::from_xyz(540.0, 260.0, DONUT_SPRINKLES_RENDER_LAYER),
        donut_sprinkles_sprite,
    ));

    /*
    =========================================================================================================
    spawn cards
    =========================================================================================================
     */
    //spawn 10 of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-500.0, 200.0, RUNE_MACHINE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHearts10.png")),
    ));

    //spawn jack of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-350.0, 200.0, RUNE_MACHINE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsJ.png")),
    ));

    //spawn queen of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-200.0, 200.0, RUNE_MACHINE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsQ.png")),
    ));

    //spawn king of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-50.0, 200.0, RUNE_MACHINE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsK.png")),
    ));

    //spawn ace of hearts
    commands
        .spawn((
            Card,
            Transform::from_xyz(100.0, -200.0, RUNE_MACHINE_RENDER_LAYER),
            Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsA.png")),
        ))
        .with_children(|parent| {
            // spawn rune slot
            parent.spawn((
                RuneSlot,
                Transform::from_xyz(0.0, 0.0, RUNE_SLOT_RENDER_LAYER).with_scale(Vec3::splat(1.3)),
                Sprite::from_image(
                    asset_server.load("runes/PNG/Black/Slab/runeBlack_slab_036.png"),
                ),
                AddCollider {
                    collider_scale: 0.7,
                    collider_type: ColliderType::Rectangle,
                },
            ));
        });

    info!("Setup completed");
}

/*
Adjusts ball collider size
 */
fn add_colliders(
    // Globals
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    entities_with_add_collider_tag: Query<(Entity, &Transform, &Sprite, &AddCollider)>,
) {
    for (entity, transform, sprite, add_collider) in entities_with_add_collider_tag {
        if asset_server.is_loaded(&sprite.image) {
            trace!("asset loaded, adding collider");

            // make sure to remove Add Collider
            commands.entity(entity).remove::<AddCollider>();

            match add_collider.collider_type {
                ColliderType::Circle => {
                    // add circle collider
                    let collider_size = add_collider.collider_scale
                        * (calculate_sprite_size(&images, &sprite, &transform.scale).x * 0.5);

                    commands
                        .entity(entity)
                        .insert(Collider::circle(collider_size));

                    debug!("Circle Collider created with size {}", collider_size);
                }
                ColliderType::Rectangle => {
                    let collider_size = add_collider.collider_scale
                        * calculate_sprite_size(&images, &sprite, &transform.scale);

                    commands
                        .entity(entity)
                        .insert(Collider::rectangle(collider_size.x, collider_size.y));

                    debug!("Circle Collider created with size {}", collider_size);
                }
            }
        } else {
            trace!("Asset not yet loaded");
        }
    }
}

/*
Handles Collisions
*/
fn handle_collision_ball_with_ball_firing_thingy(
    // Execution condition
    blue_ball: Single<(Entity, &BlueBall, &mut Transform, &mut LinearVelocity), With<Placed>>,
    // Globals
    mut commands: Commands,
    //Collisions
    collisions: Collisions,
    // Queries
    ball_firing_thingies: Query<(&BallFiringThingy, &Transform), Without<Placed>>,
) {
    let (blue_ball_entity, blue_ball, mut blue_ball_transform, mut blue_ball_velocity) =
        blue_ball.into_inner();

    trace!("Handling potential collision");

    // remove placed immediately, regardless of actual collision
    commands.entity(blue_ball_entity).remove::<Placed>();

    blue_ball_transform.translation.x = blue_ball.default_position.x;
    blue_ball_transform.translation.y = blue_ball.default_position.y;

    for contact_pair in collisions.iter() {
        /*
        The condition is:
                IF one of the two colliders is the SINGLE blue ball entity
                AND one of the two is in the query (the other one, but that doesn't matter yet)
                -> move blue ball to ball_firing_thingy detected
        */
        if (contact_pair.collider1.eq(&blue_ball_entity)
            || contact_pair.collider2.eq(&blue_ball_entity))
            && (ball_firing_thingies.contains(contact_pair.collider1)
                || ball_firing_thingies.contains(contact_pair.collider2))
        {
            // ball and ball firing thingy
            trace!("Ball placed in firing thingy");

            // closure; moves blue ball to ball firing thingy translation
            let move_blue_ball_to_firing_thingy =
                |ball_firing_entity: Entity,
                 query: &Query<(&BallFiringThingy, &Transform), Without<Placed>>,
                 blue_ball_translation: &mut Vec3| {
                    let ball_firing_thingy_transform =
                        query.get(ball_firing_entity).ok().unwrap().1;

                    // place ball in firign thingy
                    blue_ball_translation.x = ball_firing_thingy_transform.translation.x;
                    blue_ball_translation.y = ball_firing_thingy_transform.translation.y;
                };

            let entity_ball_firing_thingy: Entity;

            // place ball at the transform of firing thingy
            if contact_pair.collider1.eq(&blue_ball_entity) {
                entity_ball_firing_thingy = contact_pair.collider2;
            } else
            // colliders2 is blue ball
            {
                entity_ball_firing_thingy = contact_pair.collider1;
            }

            move_blue_ball_to_firing_thingy(
                entity_ball_firing_thingy,
                &ball_firing_thingies,
                &mut blue_ball_transform.translation,
            );

            // fire ball in direction of firing thingy
            // TODO consider rotation of ball firing_thingy (from Transform-component)
            blue_ball_velocity.0 = ball_firing_thingies
                .get(entity_ball_firing_thingy)
                .ok()
                .unwrap()
                .0
                .firing_direction;

            // blue ball can no longer be picked -> remove Pickable component
            commands.entity(blue_ball_entity).remove::<Pickable>();

            // relevant collision was detected & handled -> interrupt the loop
            break;
        }
    }
}

/*
Queries the currently moved object and moves it to the cursor position
 */
fn move_picked_object(
    // Execution Conditions
    picked: Single<&mut Transform, With<Picked>>,
    // Globals
    camera_q: Query<(&Camera, &GlobalTransform)>,
    // Events
    mut cursor_evr: EventReader<CursorMoved>,
) {
    // actual code after this

    let mut transform = picked.into_inner();

    for event in cursor_evr.read() {
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

        trace!(
            "new Pickable position: x: {}, y: {}",
            transform.translation.x, transform.translation.y
        );
    }
}

/*
Calculates sprite size and returns it
 */
fn calculate_sprite_size(images: &Res<Assets<Image>>, sprite: &Sprite, scale: &Vec3) -> Vec2 {
    let mut _sprite_size = if let Some(custom_size) = sprite.custom_size {
        trace!("Using custom sprite size");
        custom_size
    } else if let Some(image) = images.get(sprite.image.id()) {
        trace!("using image size");
        image.size_f32()
    } else {
        warn!("no custom size or sprite size found");
        Vec2::new(1.0, 1.0)
    };

    _sprite_size = _sprite_size * scale.truncate();

    _sprite_size
}

/*
Handles whichever action caused a Pick-Event
 */
fn handle_pick_event(
    //excution conditions
    player_single: Single<Entity, (With<PlayerCanPick>, With<PlayerAttemptsPick>)>,
    //Globals
    mut commands: Commands,
    images: Res<Assets<Image>>,
    //Event readers
    mut pick_event_reader: EventReader<PickEvent>,
    //Queries
    camera_q: Query<(&Camera, &GlobalTransform)>,
    pickables: Query<(Entity, &mut Transform, &Sprite), (With<Pickable>, Without<Picked>)>,
) {
    trace!("Pick event processing");
    let player_entity = player_single.into_inner();

    commands
        .entity(player_entity)
        .remove::<PlayerAttemptsPick>();

    let (camera, camera_transform) = camera_q.single().ok().unwrap();

    // in world coordinates
    let mut event_location_in_world: Vec2 = Vec2::new(0.0, 0.0);

    //read event
    for pick_event in pick_event_reader.read() {
        event_location_in_world = camera
            .viewport_to_world_2d(camera_transform, pick_event._location_in_screen_coordinates)
            .ok()
            .unwrap();
    }

    //react to click if inside Pickable Sprite
    for (pickable_entity, transform, sprite) in pickables {
        /*
        IF mouse click is inside pickable
        THEN add Picked-component to that entity AND interrupt loop
        ELSE do nothing

        note: bevy probably has a more elegant way to do this but I didn't find it immediately and won't waste Jam-time searching for it
        note note: it seems bevy currently has no elegant way to find the actual sprite size, so I need to use this monstrosity
         */

        let sprite_size = calculate_sprite_size(&images, sprite, &transform.scale);
        trace!("sprite size {:?}", sprite_size);

        // note: this is assuming the Sprite-Anchor is CENTER
        if event_location_in_world.x > transform.translation.x - sprite_size.x * 0.5
            && event_location_in_world.x < transform.translation.x + sprite_size.x * 0.5
            && event_location_in_world.y > transform.translation.y - sprite_size.y * 0.5
            && event_location_in_world.y < transform.translation.y + sprite_size.y * 0.5
        {
            commands.entity(pickable_entity).insert(Picked);

            commands.entity(player_entity).remove::<PlayerCanPick>();
            break;
        }
    }
}

/*
Handles the release of whichever Action caused a previous pick event

Release event is ignored if no Picked object was found
 */
fn handle_release_event(
    // Execution conditions
    player_single: Single<Entity, (With<Player>, With<PlayerAttemptsRelease>)>,
    // Globals
    mut commands: Commands,
    mut _release_event_reader: EventReader<ReleaseEvent>,
    // Queries
    picked: Option<Single<(Entity, &mut Transform), With<Picked>>>,
) {
    trace!("Release event processing");

    let player_entity = player_single.into_inner();

    commands
        .entity(player_entity)
        .remove::<PlayerAttemptsRelease>();

    // add PlayerCanPick marker to player
    commands.entity(player_entity).insert(PlayerCanPick);

    if let Some(picked_single) = picked {
        let (picked_entity, _transform) = picked_single.into_inner();

        // "drop" the Pickable by removing the Picked-component
        commands.entity(picked_entity).remove::<Picked>();
        commands.entity(picked_entity).insert(Placed);

        //assumption: whatever object was picked has been moved to wherever the cursor had been
        //-> the objects transform is ready for further usage

        // TODO check for overlap between picked entity and potential slot -> how to best do this?
        // read screen-position from event
        // translate to world position
        // check for overlap with all 'slot' entities
    }
}

fn controls(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut release_event_writer: EventWriter<ReleaseEvent>,
    mut pick_event_writer: EventWriter<PickEvent>,
    windows: Query<&Window>,
    player_single: Single<Entity, With<Player>>,
) {
    let player_entity = player_single.into_inner();

    // Press Left Mouse
    if input.just_pressed(MouseButton::Left) {
        trace!("Left pressed");

        let window = windows.single().ok().unwrap();

        pick_event_writer.write(PickEvent {
            _location_in_screen_coordinates: window.cursor_position().unwrap(),
        });
        commands.entity(player_entity).insert(PlayerAttemptsPick);
    }

    // Release Left Mouse
    if input.just_released(MouseButton::Left) {
        trace!("Left released");

        let window = windows.single().ok().unwrap();

        release_event_writer.write(ReleaseEvent {
            _location_in_screen_coordinates: window.cursor_position().unwrap_or_default(),
        });
        commands.entity(player_entity).insert(PlayerAttemptsRelease);
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

/*
 */
#[derive(Component)]
struct Rune;

/*
marks an object as 'Pickable', meaning the player can pick it and drag it around
*/
#[derive(Component)]
struct Pickable;

/*
marks a an object as 'Picked', meaning the player is dragging it
*/
#[derive(Component)]
struct Picked;

/*
marks the player entity
the player entity holds marker components for game logic
 */
#[derive(Component)]
struct Player;

/*
should be added to player Component when a Picked entity is released
is used to make sure Pick-Events are only handled while the player holds this tag
 */
#[derive(Component)]
struct PlayerCanPick;

/*
marks that the player is attempting a pick
 */
#[derive(Component)]
struct PlayerAttemptsPick;

/*
marks that the player is attempting a release
 */
#[derive(Component)]
struct PlayerAttemptsRelease;

#[derive(Component)]
struct Card;

#[derive(Component)]
struct DonutCircle;

#[derive(Component)]
struct BlueBall {
    default_position: Vec2,
}

#[derive(Component)]
struct GreyBall {
    default_position: Vec2,
}

#[derive(Component)]
struct BallFiringThingy {
    firing_direction: Vec2,
}

#[derive(Component)]
struct RuneSlot;

#[derive(Component)]
struct Placed;

#[derive(Component)]
struct AddCollider {
    collider_scale: f32,
    collider_type: ColliderType,
}

/*
========================================================================================
Events
========================================================================================
 */

#[derive(Event)]
struct PickEvent {
    // screen coordinates means (0,0) to (window_width, window_height); from top-left to bottom-right
    _location_in_screen_coordinates: Vec2,
}

#[derive(Event)]
struct ReleaseEvent {
    // screen coordinates means (0,0) to (window_width, window_height); from top-left to bottom-right
    _location_in_screen_coordinates: Vec2,
}

/*
========================================================================================
Enums
========================================================================================
 */
pub enum ColliderType {
    Circle,
    Rectangle,
}
