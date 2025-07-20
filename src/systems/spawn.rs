use crate::bundles::*;
use crate::components::*;
use bevy::prelude::*;

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
            Collider(ColliderShape::Circle { radius: 50.0 }),
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

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. Build a mesh straight from the Circle primitive
    let mesh = meshes.add(Circle::new(50.0));

    // 2. Solid-colour material
    let red = materials.add(ColorMaterial::from(Color::linear_rgb(0.7, 0.2, 0.3)));

    // 3. Enemy entity
    commands
        .spawn((
            Name::new("Enemy"),
            Mesh2d(mesh),        // mesh component
            MeshMaterial2d(red), // material component
            Collider(ColliderShape::Circle { radius: 50.0 }),
        ))
        .insert(MovableBundle {
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        });
}

// Spawn the map
pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rectangle_mesh = meshes.add(Mesh::from(Rectangle::new(50.0, 100.0)));
    let color = Color::linear_rgb(0.8, 0.7, 0.3);

    commands.spawn((
        Wall,
        Mesh2d(rectangle_mesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(200.0, 100.0, 0.0)),
        Collider(ColliderShape::Rect {
            half_extents: Vec2::new(25.0, 50.0),
        }),
    ));

    let hw: f32 = 500.0;
    let hh: f32 = 500.0;
    let thickness: f32 = 200.0;

    let height: f32 = (hh + thickness) * 2.0;
    let width: f32 = (hw + thickness) * 2.0;

    // south wall
    commands.spawn((
        Wall,
        Mesh2d(meshes.add(Mesh::from(Rectangle::new(width, thickness)))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(0.0, -hh - (thickness / 2.0), 0.0),
        Collider(ColliderShape::Rect {
            half_extents: Vec2::new(width / 2.0, thickness / 2.0),
        }),
    ));
    // north wall
    commands.spawn((
        Wall,
        Mesh2d(meshes.add(Mesh::from(Rectangle::new(width, thickness)))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(0.0, hh + (thickness / 2.0), 0.0),
        Collider(ColliderShape::Rect {
            half_extents: Vec2::new(width / 2.0, thickness / 2.0),
        }),
    ));
    // west wall
    commands.spawn((
        Wall,
        Mesh2d(meshes.add(Mesh::from(Rectangle::new(thickness, height)))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(-hw - (thickness / 2.0), 0.0, 0.0),
        Collider(ColliderShape::Rect {
            half_extents: Vec2::new(thickness / 2.0, height / 2.0),
        }),
    ));
    // east wall
    commands.spawn((
        Wall,
        Mesh2d(meshes.add(Mesh::from(Rectangle::new(thickness, height)))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(hw + (thickness / 2.0), 0.0, 0.0),
        Collider(ColliderShape::Rect {
            half_extents: Vec2::new(thickness / 2.0, height / 2.0),
        }),
    ));
}

/// Spawn a bit of UI text to explain how to move the player.
pub fn spawn_text(mut commands: Commands) {
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
