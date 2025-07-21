use crate::prelude::*;
use crate::spawn::*;

pub fn load_core_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // geometry
    let rect = meshes.add(Mesh::from(Rectangle::default()));
    let circle = meshes.add(Mesh::from(Circle { radius: 1.0 }));

    // colours
    let bounds_mat = materials.add(Color::linear_rgb(0.2, 0.2, 0.2));
    let wall_mat = materials.add(Color::linear_rgb(0.8, 0.7, 0.3));
    let enemy_mat = materials.add(Color::linear_rgb(0.7, 0.2, 0.3));
    let player_mat = materials.add(Color::linear_rgb(0.2, 0.7, 0.3));

    commands.insert_resource(CoreMeshes { rect, circle });
    commands.insert_resource(CoreMaterials {
        bounds: bounds_mat,
        wall: wall_mat,
        enemy: enemy_mat,
        player: player_mat,
    });
}

/// Spawn the player sprite and a 2D camera.
pub fn spawn_player(
    mut commands: Commands,
    meshes: Res<CoreMeshes>,
    materials: Res<CoreMaterials>,
) {
    // Player entity
    commands
        .spawn((
            Name::new("Player"),
            Player,
            Shape2dBundle::circle(
                meshes.circle.clone(),
                materials.player.clone(),
                50.0,
                Vec2::new(0.0, -150.0),
            ),
        ))
        .insert(MovableBundle::default())
        .with_children(|parent| {
            parent.spawn((
                Camera2d,                            // marker component
                Transform::from_xyz(0.0, 0.0, 20.0), // where the camera sits
                GlobalTransform::default(),          // required by the renderer
                Projection::default(),               // default orthographic projection
            ));
        });
}

// Spawn the map
pub fn spawn_map(mut commands: Commands, meshes: Res<CoreMeshes>, materials: Res<CoreMaterials>) {
    commands.spawn((
        Wall,
        Shape2dBundle::rect(
            meshes.rect.clone(),
            materials.wall.clone(),
            Vec2::new(50.0, 100.0),
            Vec2::new(200.0, 100.0),
        ),
    ));

    let horizontal = Vec2::new((WALL_HALF_W + WALL_THICKNESS) * 2.0, WALL_THICKNESS);
    let vertical = Vec2::new(WALL_THICKNESS, (WALL_HALF_H + WALL_THICKNESS) * 2.0);

    for (size, pos) in [
        (
            horizontal,
            Vec2::new(0.0, -WALL_HALF_H - WALL_THICKNESS * 0.5),
        ), // bottom
        (
            horizontal,
            Vec2::new(0.0, WALL_HALF_H + WALL_THICKNESS * 0.5),
        ), // top
        (
            vertical,
            Vec2::new(-WALL_HALF_W - WALL_THICKNESS * 0.5, 0.0),
        ), // left
        (vertical, Vec2::new(WALL_HALF_W + WALL_THICKNESS * 0.5, 0.0)), // right
    ] {
        commands.spawn((
            Wall, // <- your marker component
            Shape2dBundle::rect(meshes.rect.clone(), materials.bounds.clone(), size, pos),
        ));
    }
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

pub fn spawn_enemy_on_key(
    mut writer: EventWriter<SpawnEnemiesEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Digit0) {
        writer.write(SpawnEnemiesEvent {
            count: 1,
            ..default()
        });
    }
}

pub fn spawn_enemies_event_handler(
    mut commands: Commands,
    meshes: Res<CoreMeshes>,
    materials: Res<CoreMaterials>,
    mut reader: EventReader<SpawnEnemiesEvent>,
) {
    for SpawnEnemiesEvent { count, pos } in reader.read() {
        for i in 0..*count {
            let enemy_pos = pos
                + Vec2::new(
                    // sprinkle them horizontally for variety
                    -200.0 + i as f32 * 80.0,
                    150.0,
                );

            spawn_enemy(&mut commands, &meshes, &materials, enemy_pos);
        }
    }
}

// Helper function
pub fn spawn_enemy(
    commands: &mut Commands,
    meshes: &Res<CoreMeshes>,
    materials: &Res<CoreMaterials>,
    pos: Vec2,
) {
    // Enemy entity
    commands
        .spawn((
            Name::new("Enemy"),
            Shape2dBundle::circle(meshes.circle.clone(), materials.enemy.clone(), 50.0, pos),
        ))
        .insert(MovableBundle::default());
}
