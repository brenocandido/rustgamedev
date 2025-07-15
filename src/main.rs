use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod phy_engine;
use phy_engine::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .register_type::<AccumulatedInput>()
        .register_type::<Velocity>()
        .register_type::<PhysicalTranslation>()
        .register_type::<PreviousPhysicalTranslation>()
        .register_type::<Acceleration>()
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, (spawn_text, spawn_player, spawn_map))
        // Advance the physics simulation using a fixed timestep.
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(
            // The `RunFixedMainLoop` schedule allows us to schedule systems to run before and after the fixed timestep loop.
            RunFixedMainLoop,
            (
                circle_rect_collider.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                // The physics simulation needs to know the player's input, so we run this before the fixed timestep loop.
                // Note that if we ran it in `Update`, it would be too late, as the physics simulation would already have been advanced.
                // If we ran this in `FixedUpdate`, it would sometimes not register player input, as that schedule may run zero times per frame.
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                // The player's visual representation needs to be updated after the physics simulation has been advanced.
                // This could be run in `Update`, but if we run it here instead, the systems in `Update`
                // will be working with the `Transform` that will actually be shown on screen.
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        )
        .run();
}

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct CircleCollider {
    pub radius: f32,
}

#[derive(Component)]
struct RectCollider {
    pub half_extents: Vec2,
}

/// Spawn the player sprite and a 2D camera.
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. Build a mesh straight from the Circle primitive
    let mesh = meshes.add(Circle::new(50.0));

    // 2. Solid-colour material
    let green = materials.add(ColorMaterial::from(Color::linear_rgb(0.2, 0.7, 0.3)));

    // 3. Player entity
    commands
        .spawn((
            Name::new("Player"),
            Mesh2d(mesh),          // mesh component
            MeshMaterial2d(green), // material component
            Player,
            CircleCollider { radius: 50.0 },
        ))
        .insert(MovableBundle {
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Camera2d,                              // marker component
                Transform::from_xyz(0.0, 0.0, 1000.0), // where the camera sits
                GlobalTransform::default(),            // required by the renderer
                Projection::default(),                 // default orthographic projection
            ));
        });
}

// Spawn the map
fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rectangle_mesh = meshes.add(Mesh::from(Rectangle::new(50.0, 100.0)));

    let color = Color::linear_rgb(0.8, 0.7, 0.3);

    commands.spawn((
        Mesh2d(rectangle_mesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(200.0, 100.0, 0.0)),
        Wall,
        RectCollider {
            half_extents: Vec2::new(25.0, 50.0),
        },
    ));
}

/// Spawn a bit of UI text to explain how to move the player.
fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
        .with_child((
            Text::new("Move the player with WASD"),
            TextFont {
                font_size: 25.0,
                ..default()
            },
        ));
}

/// Handle keyboard input and accumulate it in the `AccumulatedInput` component.
///
/// There are many strategies for how to handle all the input that happened since the last fixed timestep.
/// This is a very simple one: we just accumulate the input and average it out by normalizing it.
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut Acceleration)>,
) {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    const ACCELERATION: f32 = 20.0;
    for (mut input, mut acceleration) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.vec.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            input.vec.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.vec.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.vec.x += 1.0;
        }

        input.cnt += 1;

        // Need to normalize and scale because otherwise
        // diagonal movement would be faster than horizontal or vertical movement.
        // This effectively averages the accumulated input.
        acceleration.0 = input.vec.extend(0.0).normalize_or_zero() * ACCELERATION;
    }
}

fn circle_rect_collider(
    mut movers: Query<(&mut Transform, &mut Velocity, &CircleCollider), With<Player>>,
    statics: Query<(&Transform, &RectCollider), Without<Player>>,
) {
    for (mut tf, mut vel, circle) in &mut movers {
        //----------------------------------------------------------------------
        // 2) For each wall: test & resolve
        //----------------------------------------------------------------------
        // World-space circle centre & radius
        let centre = tf.translation.truncate();
        let radius = circle.radius * tf.scale.x; // assume uniform scale

        for (w_tf, rect) in &statics {
            // World-space box centre & half-extents (apply scale if any)
            let box_center = w_tf.translation.truncate();
            let box_he = rect.half_extents * w_tf.scale.truncate();

            // Closest point on the AABB to the circle centre
            let closest = (centre - box_center).clamp(-box_he, box_he) + box_center;

            // Vector from closest point to centre
            let delta = centre - closest;
            let dist2 = delta.length_squared();

            if dist2 < radius * radius {
                // ---------- overlap! ----------
                let dist = dist2.sqrt();
                // If circle centre is *inside* the box (dist == 0) push it out vertically
                let penetration_depth = radius - dist;
                let push_dir = if dist > 0.0001 { delta / dist } else { Vec2::Y };

                // Move circle out & zero velocity along the push axis
                tf.translation += (push_dir * penetration_depth).extend(0.0);

                if push_dir.x.abs() > push_dir.y.abs() {
                    if push_dir.x >= 0.0 {
                        vel.0.x = f32::max(0.0, vel.0.x);
                    } else {
                        vel.0.x = f32::min(0.0, vel.0.x);
                    }
                } else {
                    if push_dir.y >= 0.0 {
                        vel.0.y = f32::max(0.0, vel.0.y);
                    } else {
                        vel.0.y = f32::min(0.0, vel.0.y);
                    }
                }

                // input.0 = Vec2::ZERO;
            }
        }
    }
}
