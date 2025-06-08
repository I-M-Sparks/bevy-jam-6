use avian2d::prelude::*;
use bevy::{ecs::hierarchy, log::*, prelude::*};

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
        // game logic
        .add_systems(Update, apply_rune_effects)
        // Collision handling
        .add_systems(
            PostUpdate,
            (
                handle_collision_ball_with_ball_firing_thingy,
                handle_collision_rune_with_rune_slot,
                handle_collision_blue_ball_and_runes,
            ),
        )
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

const BACKGROUND_RENDER_LAYER: f32 = 0.0;

const RUNE_MACHINE_RENDER_LAYER: f32 = 50.0;
// Rune-slots are child-entities of rune machine parts, so they are rendered on top of them
const RUNE_SLOT_RENDER_LAYER: f32 = 51.0;

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
    let mut render_layer = RUNE_RENDER_LAYER;

    let mut rune_default_position;

    rune_default_position = Vec2::new(-595.0, -290.0);

    //spawn a rune
    commands.spawn((
        Rune {
            default_position: rune_default_position,
            affected_entity: None,
            rune_effect: RuneEffect {
                rune_effect_type: RuneEffectType::MoveUp,
                rune_effect_move_speed: Some(Vec2::new(0.0, 100.0)),
            },
        },
        Pickable,
        RenderLayer {
            render_layer: render_layer,
        },
        Transform::from_xyz(
            rune_default_position.x,
            rune_default_position.y,
            render_layer,
        ),
        Sprite::from_image(asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_001.png")),
        AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::Rectangle,
        },
    ));

    rune_default_position = Vec2::new(-535.0, -290.0);

    //spawn a second rune
    commands.spawn((
        Rune {
            default_position: rune_default_position,
            affected_entity: None,
            rune_effect: RuneEffect {
                rune_effect_type: RuneEffectType::MoveLeft,
                rune_effect_move_speed: Some(Vec2::new(-100.0, 0.0)),
            },
        },
        Pickable,
        RenderLayer {
            render_layer: render_layer,
        },
        Transform::from_xyz(
            rune_default_position.x,
            rune_default_position.y,
            render_layer,
        ),
        Sprite::from_image(asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_002.png")),
        AddCollider {
            collider_scale: 1.0,
            collider_type: ColliderType::Rectangle,
        },
    ));

    /*
    =========================================================================================================
    Spawn Pickable object area
    =========================================================================================================
     */
    render_layer = BACKGROUND_RENDER_LAYER;

    commands.spawn((
        RenderLayer {
            render_layer: render_layer,
        },
        Transform::from_xyz(-448.0, -296.0, render_layer),
        Sprite::from_image(
            asset_server.load("UI Pack/PNG/Blue/Double/button_rectangle_depth_line.png"),
        ),
    ));

    commands.spawn((
        RenderLayer {
            render_layer: render_layer,
        },
        Transform::from_xyz(448.0, -296.0, render_layer),
        Sprite::from_image(
            asset_server.load("UI Pack/PNG/Blue/Double/button_rectangle_depth_line.png"),
        ),
    ));

    /*
    =========================================================================================================
    Spawn starter-balls
    =========================================================================================================
     */

    // Blue ball
    let blue_ball_sprite =
        Sprite::from_image(asset_server.load("Puzzle Assets/PNG/Double/ballBlue.png"));

    let blue_ball_default_position = Vec2::new(300.0, -296.0);

    render_layer = BALL_RENDER_LAYER;

    let blue_ball_transform = Transform::from_xyz(
        blue_ball_default_position.x,
        blue_ball_default_position.y,
        render_layer,
    );

    commands.spawn((
        BlueBall {
            default_position: blue_ball_default_position.clone(),
        },
        blue_ball_sprite,
        blue_ball_transform,
        RenderLayer {
            render_layer: render_layer,
        },
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

    let mut grey_ball_default_position = Vec2::new(350.0, -296.0);

    commands.spawn((
        GreyBall {
            default_position: grey_ball_default_position.clone(),
        },
        grey_ball_sprite.clone(),
        RenderLayer {
            render_layer: render_layer,
        },
        Transform::from_xyz(
            grey_ball_default_position.x,
            grey_ball_default_position.y,
            render_layer,
        ),
        RigidBody::Kinematic,
    ));

    grey_ball_default_position = Vec2::new(400.0, -296.0);

    commands.spawn((
        GreyBall {
            default_position: grey_ball_default_position.clone(),
        },
        grey_ball_sprite.clone(),
        Transform::from_xyz(
            grey_ball_default_position.x,
            grey_ball_default_position.y,
            render_layer,
        ),
        Pickable,
        RigidBody::Kinematic,
    ));

    // Ball firing Thingy
    let ball_firing_thingy_sprite = Sprite::from_image(
        asset_server.load("UI Pack/PNG/Grey/Double/check_round_grey_circle.png"),
    );

    render_layer = BALL_FIRING_THINGY_RENDER_LAYER;

    let ball_firing_thingy_transform = Transform::from_xyz(500.0, -150.0, render_layer);

    // note: default direction of BallFiringThingy is to the left
    commands
        .spawn((
            BallFiringThingy {
                // speed is units per second; see addition of physics plugin to determine how much that is in pixels
                firing_direction: Vec2::new(-100.0, 0.0),
            },
            ball_firing_thingy_sprite,
            ball_firing_thingy_transform,
            RenderLayer {
                render_layer: render_layer,
            },
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
        Sprite::from_image(asset_server.load("Particle Pack/PNG (Transparent)/magic_02.png"));
    donut_circle_sprite.custom_size = Some(Vec2::new(200.0, 200.0));
    donut_circle_sprite.color = Color::linear_rgb(1.0, 0.1, 0.1);

    render_layer = DONUT_CIRCLE_RENDER_LAYER;

    commands.spawn((
        DonutCircle,
        Transform::from_xyz(540.0, 260.0, render_layer),
        RenderLayer {
            render_layer: render_layer,
        },
        donut_circle_sprite,
    ));

    // spawn target donut base
    let mut donut_base_sprite = Sprite::from_image(asset_server.load("Donuts/PNG/donut_1.png"));
    donut_base_sprite.custom_size = Some(Vec2::new(80.0, 80.0));

    render_layer = DONUT_BASE_RENDER_LAYER;

    commands.spawn((
        DonutCircle,
        Transform::from_xyz(540.0, 260.0, render_layer),
        RenderLayer {
            render_layer: render_layer,
        },
        donut_base_sprite,
    ));

    // spawn target donut frosting
    let mut donut_frosting_sprite =
        Sprite::from_image(asset_server.load("Donuts/PNG/glazing_5.png"));
    donut_frosting_sprite.custom_size = Some(Vec2::new(70.0, 70.0));

    render_layer = DONUT_FROSTING_RENDER_LAYER;

    commands.spawn((
        DonutCircle,
        Transform::from_xyz(540.0, 260.0, render_layer),
        RenderLayer {
            render_layer: render_layer,
        },
        donut_frosting_sprite,
    ));

    // spawn target donut sprinkles
    let mut donut_sprinkles_sprite =
        Sprite::from_image(asset_server.load("Donuts/PNG/sprinkles_1.png"));
    donut_sprinkles_sprite.custom_size = Some(Vec2::new(70.0, 70.0));

    render_layer = DONUT_SPRINKLES_RENDER_LAYER;

    commands.spawn((
        DonutCircle,
        Transform::from_xyz(540.0, 260.0, render_layer),
        RenderLayer {
            render_layer: render_layer,
        },
        donut_sprinkles_sprite,
    ));

    /*
    =========================================================================================================
    spawn cards
    =========================================================================================================
     */
    //spawn 10 of hearts
    render_layer = RUNE_MACHINE_RENDER_LAYER;

    commands.spawn((
        Card,
        Transform::from_xyz(-500.0, 200.0, render_layer),
        RenderLayer {
            render_layer: render_layer,
        },
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHearts10.png")),
    ));

    //spawn jack of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-350.0, 200.0, render_layer),
        RenderLayer {
            render_layer: render_layer,
        },
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsJ.png")),
    ));

    //spawn queen of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-200.0, 200.0, render_layer),
        RenderLayer {
            render_layer: render_layer,
        },
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsQ.png")),
    ));

    //spawn king of hearts
    commands.spawn((
        Card,
        Transform::from_xyz(-50.0, 200.0, render_layer),
        RenderLayer {
            render_layer: render_layer,
        },
        Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsK.png")),
    ));

    let rune_slot_render_layer: f32 = RUNE_SLOT_RENDER_LAYER - render_layer;

    //spawn ace of hearts
    commands
        .spawn((
            Card,
            Transform::from_xyz(100.0, -200.0, render_layer),
            RenderLayer {
                render_layer: render_layer,
            },
            Sprite::from_image(asset_server.load("Boardgame Pack/PNG/Cards/cardHeartsA.png")),
        ))
        .with_children(|parent| {
            // spawn rune slot
            parent.spawn((
                RuneSlot,
                Transform::from_xyz(0.0, 0.0, rune_slot_render_layer).with_scale(Vec3::splat(1.3)),
                RenderLayer {
                    render_layer: RUNE_SLOT_RENDER_LAYER,
                },
                Sprite::from_image(
                    asset_server.load("runes/PNG/Black/Slab/runeBlack_slab_036.png"),
                ),
                AddCollider {
                    collider_scale: 0.5,
                    collider_type: ColliderType::Rectangle,
                },
            ));
        });

    info!("Setup completed");
}

/*
========================================================================================
Collision Handling
========================================================================================
 */

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
Handle Collisions between the blue ball and the ball firing thingy (or thingies, if there are multiple)
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
    trace!("Handling potential collision between blue ball and ball firing thingy");

    let (blue_ball_entity, blue_ball, mut blue_ball_transform, mut blue_ball_velocity) =
        blue_ball.into_inner();

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
Handles Collision between the placed rune and all rune slots
 */
fn handle_collision_rune_with_rune_slot(
    // Execution condition
    rune: Single<(Entity, &mut Rune, &mut Transform), With<Placed>>,
    // Globals
    mut commands: Commands,
    //Collisions
    collisions: Collisions,
    // Queries
    rune_slots: Query<(&RuneSlot, &GlobalTransform, &ChildOf), (Without<Placed>, Without<Rune>)>,
    runes: Query<
        (&mut Rune, &mut Transform, &RenderLayer, &ChildOf),
        (Without<RuneSlot>, Without<Placed>),
    >,
) {
    trace!("Handling potential collision between rune and rune slot");

    let (rune_entity, mut rune, mut rune_transform) = rune.into_inner();

    // remove placed immediately, regardless of actual collision
    commands.entity(rune_entity).remove::<Placed>();

    // place rune in it's default position; will be overwritten if a collision is detected
    rune_transform.translation.x = rune.default_position.x;
    rune_transform.translation.y = rune.default_position.y;

    // check for rune that was replaced and reset it
    let mut entity_replaced_rune: Option<Entity> = None;

    for contact_pair in collisions.iter() {
        /*
        The condition is:
                IF one of the two colliders is the SINGLE rune entity
                AND one of the two is in the query (the other one, but that doesn't matter yet)
                -> move rune to rune_slot detected (by making it a child?)
        */
        if (contact_pair.collider1.eq(&rune_entity) || contact_pair.collider2.eq(&rune_entity))
            && (rune_slots.contains(contact_pair.collider1)
                || rune_slots.contains(contact_pair.collider2))
        {
            trace!("Rune placed in rune slot, handling...");

            let entity_rune_slot: Entity;

            if contact_pair.collider1.eq(&rune_entity) {
                entity_rune_slot = contact_pair.collider2;
            } else {
                entity_rune_slot = contact_pair.collider1;
            }

            // move rune into rune-slot by setting rune-translation to zero and making it a child
            commands.entity(entity_rune_slot).add_child(rune_entity);
            // reset tranlation so rune will be cneterd in slot
            rune_transform.translation = Vec3::ZERO;
            // increase z-component so rune will be drawn on top of slot
            rune_transform.translation.z += 1.0;

            // need to inverse the rune-slots scale or the rune will sclae up as well
            rune_transform.scale = 1.0 / rune_slots.get(entity_rune_slot).ok().unwrap().1.scale();

            // store the affected entity (the entity the rune slot is attached to)
            rune.affected_entity = Some(rune_slots.get(entity_rune_slot).ok().unwrap().2.parent());

            // check for a collision with another rune, and if there is one, place that rune back to it's default position
            // TODO this could be improved by reducing the query to only runes that are placed in a slot
            for contact_pair in collisions.iter() {
                trace!("Rune was placed, checking for another rune that should be reset...");

                if (contact_pair.collider1.eq(&rune_entity)
                    || contact_pair.collider2.eq(&rune_entity))
                    && (runes.contains(contact_pair.collider1)
                        || runes.contains(contact_pair.collider2))
                {
                    trace!(
                        "Rune was placed in slot that was already filled; resetting previous rune"
                    );

                    if contact_pair.collider1.eq(&rune_entity) {
                        entity_replaced_rune = Some(contact_pair.collider2);
                    } else {
                        entity_replaced_rune = Some(contact_pair.collider1);
                    }

                    // replaced rune has been detected -> interrupt the loop
                    // implicit assumption: there is only ever one rune-with-rune collision, because there is never more than one rune inside a slot and slots dont' overlap
                    break;
                }
            }

            // relevant collision (rune with rune slot) was detected & handled -> interrupt the loop
            break;
        }
    }

    // condition: if replaced rune option was found
    if let Some(entity_replaced_rune) = entity_replaced_rune {
        let (
            mut replaced_rune,
            mut replaced_rune_transform,
            replaced_rune_render_layer,
            replaced_rune_child_of,
        ) = runes.get_inner(entity_replaced_rune).ok().unwrap();

        commands
            .entity(replaced_rune_child_of.parent())
            .remove_children(&[entity_replaced_rune]);

        replaced_rune_transform.translation.x = replaced_rune.default_position.x;
        replaced_rune_transform.translation.y = replaced_rune.default_position.y;
        replaced_rune_transform.translation.z = replaced_rune_render_layer.render_layer;

        replaced_rune_transform.scale = Vec3::ONE;

        replaced_rune.affected_entity = None;
    }
}

/*
Handles collisions between Blue Ball and Runes
 */
fn handle_collision_blue_ball_and_runes(
    // Execution condition
    blue_ball: Single<(Entity, &BlueBall), Without<Pickable>>,
    // Globals
    mut commands: Commands,
    //Collisions
    collisions: Collisions,
    // Queries
    runes: Query<(&Rune, &mut Sprite)>,
) {
    trace!("Handling potential collision between rune and blue ball");

    for contact_pair in collisions.iter() {
        if (contact_pair.collider1.eq(&blue_ball.0) || contact_pair.collider2.eq(&blue_ball.0))
            && (runes.contains(contact_pair.collider1) || runes.contains(contact_pair.collider2))
        {
            trace!("Handling collision between blue ball and rune");

            commands.entity(blue_ball.0).despawn();

            let rune_entity: Entity;

            // move rune effect component from rune to affected entity
            if contact_pair.collider1.eq(&blue_ball.0) {
                rune_entity = contact_pair.collider2;
            } else {
                rune_entity = contact_pair.collider1;
            }

            // move the RuneEffect from the rune entity to the affected entity (as provided by the RuneSlot)
            let (rune, mut sprite) = runes.get_inner(rune_entity).ok().unwrap();

            commands
                .entity(rune.affected_entity.unwrap())
                .insert(rune.rune_effect.clone());

            // change color tint
            sprite.color = Color::linear_rgb(0.2, 0.2, 1.0);

            trace!("RuneEffect component should now be added to Card-Entity");

            break;
        }
    }
}

/*
========================================================================================
Input Handling
========================================================================================
 */

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
    pickables: Query<
        (
            Entity,
            &mut Transform,
            &GlobalTransform,
            &Sprite,
            &RenderLayer,
            Option<&ChildOf>,
        ),
        (With<Pickable>, Without<Picked>, Without<Camera>),
    >,
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
    for (pickable_entity, mut transform, global_transform, sprite, render_layer, child_of) in
        pickables
    {
        /*
        IF mouse click is inside pickable
        THEN add Picked-component to that entity AND interrupt loop
        ELSE do nothing

        note: bevy probably has a more elegant way to do this but I didn't find it immediately and won't waste Jam-time searching for it
        note note: it seems bevy currently has no elegant way to find the actual sprite size, so I need to use this monstrosity
         */

        let sprite_size = calculate_sprite_size(&images, sprite, &global_transform.scale());
        trace!("sprite size {:?}", sprite_size);

        // note: this is assuming the Sprite-Anchor is CENTER
        if event_location_in_world.x > global_transform.translation().x - sprite_size.x * 0.5
            && event_location_in_world.x < global_transform.translation().x + sprite_size.x * 0.5
            && event_location_in_world.y > global_transform.translation().y - sprite_size.y * 0.5
            && event_location_in_world.y < global_transform.translation().y + sprite_size.y * 0.5
        {
            // mark as picked
            commands.entity(pickable_entity).insert(Picked);

            // rmeove pplayers ability to pick objects to prevent bugs from "double input" (probably an edge case, but hey, it's cheap to handle)
            commands.entity(player_entity).remove::<PlayerCanPick>();

            //remove rune from any parent (which would be a rune slot)
            if let Some(child_of) = child_of {
                commands
                    .entity(child_of.parent())
                    .remove_children(&[pickable_entity]);
                // reset scale
                transform.scale = Vec3::ONE;
                transform.translation.x = event_location_in_world.x;
                transform.translation.y = event_location_in_world.y;
                transform.translation.z = render_layer.render_layer;
            }

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
    // Execution conditions
    player_single: Single<Entity, With<Player>>,
    _blue_ball: Single<&BlueBall, With<Pickable>>,
    //Globals
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut release_event_writer: EventWriter<ReleaseEvent>,
    mut pick_event_writer: EventWriter<PickEvent>,
    // Queries
    windows: Query<&Window>,
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
Game Logic
========================================================================================
 */
fn apply_rune_effects(
    //Globals
    time: Res<Time>,
    //Queries
    active_entities: Query<(&RuneEffect, &mut Transform)>,
) {
    for (rune_effect, mut transform) in active_entities {
        // TODO actually do things here

        match rune_effect.rune_effect_type {
            RuneEffectType::MoveUp
            | RuneEffectType::MoveDown
            | RuneEffectType::MoveLeft
            | RuneEffectType::MoveRight => {
                if let Some(move_speed) = rune_effect.rune_effect_move_speed {
                    transform.translation.x += move_speed.x * time.delta_secs();
                    transform.translation.y += move_speed.y * time.delta_secs();
                } else {
                    warn!("Effect is movement, but no move speed was set!");
                }
            }
        }
    }
}

/*
========================================================================================
Components
========================================================================================
 */

/*
 */
#[derive(Component)]
struct Rune {
    default_position: Vec2,
    affected_entity: Option<Entity>,
    rune_effect: RuneEffect,
}

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
Each Rendered Object can be queried along with it's render layer

Used to calculate correct render layers at all times, especially in parent-child hierarchies

IMPORTANT: this stores the TARGET render layer, not the currently used one!
so if a child entity has a transform.z = 1.0, the render_layer stored in this component might still be 51.0
 */
#[derive(Component)]
struct RenderLayer {
    render_layer: f32,
}

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

#[derive(Component, Default, Copy, Clone)]
struct RuneEffect {
    rune_effect_type: RuneEffectType,
    rune_effect_move_speed: Option<Vec2>,
}

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

#[derive(Copy, Clone, Default)]
pub enum RuneEffectType {
    #[default]
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}
