use bevy::{log::*, prelude::*};

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
}

/*
========================================================================================
Constants
========================================================================================
*/
// define z values for various assets; creates layers through z-components of transform
// this ensures a correct drawing order for sprites
// note: z value of 0 should be set for the background; objects drawn "further in front" must have a HIGHER value
const RUNE_RENDER_LAYER: f32 = 100.0;
const CARD_RANDER_LAYER: f32 = 50.0;
const DONUT_CIRCLE_RENDER_LAYER: f32 = 75.0;
const DONUT_BASE_RENDER_LAYER: f32 = 76.0;
const DONUT_FROSTING_RENDER_LAYER: f32 = 76.0;
const DONUT_SPRINKLES_RENDER_LAYER: f32 = 76.0;

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
        //FOR DEBUGGING
        //Picked,
        Transform::from_xyz(-600.0, -300.0, RUNE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_001.png")),
    ));

    //spawn a second rune
    commands.spawn((
        Rune,
        Pickable,
        Transform::from_xyz(-540.0, -300.0, RUNE_RENDER_LAYER),
        Sprite::from_image(asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_002.png")),
    ));

    /*
    =========================================================================================================
    spawn runeslots
    =========================================================================================================
     */

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
        Transform::from_xyz(-500.0, 200.0, CARD_RANDER_LAYER),
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHearts10.png")),
    ));

    //spawn jack of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-350.0, 200.0, CARD_RANDER_LAYER),
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsJ.png")),
    ));

    //spawn queen of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-200.0, 200.0, CARD_RANDER_LAYER),
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsQ.png")),
    ));

    //spawn king of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-50.0, 200.0, CARD_RANDER_LAYER),
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsK.png")),
    ));

    //spawn ace of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(100.0, -0.0, CARD_RANDER_LAYER),
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsA.png")),
    ));

    info!("Setup completed");
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
            .viewport_to_world_2d(camera_transform, pick_event.location_in_screen_coordinates)
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
    mut release_event_reader: EventReader<ReleaseEvent>,
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
        let (picked_entity, transform) = picked_single.into_inner();

        // "drop" the Pickable by removing the Picked-component
        commands.entity(picked_entity).remove::<Picked>();

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
            location_in_screen_coordinates: window.cursor_position().unwrap(),
        });
        commands.entity(player_entity).insert(PlayerAttemptsPick);
    }

    // Release Left Mouse
    if input.just_released(MouseButton::Left) {
        trace!("Left released");

        let window = windows.single().ok().unwrap();

        release_event_writer.write(ReleaseEvent {
            location_in_screen_coordinates: window.cursor_position().unwrap_or_default(),
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
