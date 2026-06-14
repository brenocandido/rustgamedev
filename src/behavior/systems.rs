use crate::behavior::{
    DEFAULT_PUSH_WEIGHT, WANDER_ACCEL_VARIATION_RATE, WANDER_MAX_ACCEL_MULTIPLIER,
    WANDER_MIN_ACCEL_MULTIPLIER, WANDER_TURN_VARIATION_RATE,
};
use crate::prelude::*;
use fastrand;

pub fn seek_player_system(
    player_q: Query<&PhysicalTranslation, With<Player>>,
    mut enemies: Query<(&PhysicalTranslation, &Seek, &mut AccumulatedInput), Without<Player>>,
) {
    let Ok(player_tf) = player_q.single() else {
        return;
    };

    let player_pos = player_tf.truncate();
    for (tf, seek, mut input) in enemies.iter_mut() {
        let pos = tf.truncate();
        let to_player = player_pos - pos;
        let distance_sq = seek.distance * seek.distance;

        if to_player.length_squared() < distance_sq {
            input.0 += to_player.normalize() * 1.0; // SEEK_WEIGHT = 1.0 (example)
        }
    }
}

pub fn flee_from_player_system(
    player_q: Query<(&PhysicalTranslation, &Collider), With<Player>>,
    mut enemies: Query<
        (
            &PhysicalTranslation,
            &Collider,
            &Flee,
            &mut AccumulatedInput,
        ),
        Without<Player>,
    >,
) {
    let Ok((player_tf, player_col)) = player_q.single() else {
        return;
    };
    let player_pos = &player_tf.truncate();
    let player_radius = match player_col.0 {
        ColliderShape::Circle { radius } => radius,
        _ => 0.0,
    };
    for (tf, col, flee, mut input) in enemies.iter_mut() {
        let radius = match col.0 {
            ColliderShape::Circle { radius } => radius,
            _ => 0.0,
        };

        let pos = &tf.truncate();
        input.0 += get_push_vector(
            pos,
            player_pos,
            radius,
            player_radius,
            flee.distance,
            Some(DEFAULT_PUSH_WEIGHT * 2.0),
        );
    }
}

pub fn wander_system(
    fixed_time: Res<Time<Fixed>>,
    mut enemies: Query<(&mut Wander, &mut AccumulatedInput), Without<Player>>,
) {
    let dt = fixed_time.delta_secs();

    for (mut wander, mut input) in enemies.iter_mut() {
        // Change the current variation slowly over time using random noise.
        // This results in a smoother change of direction instead of jittering back and forth.
        let delta_variation =
            (fastrand::f32() * 2.0 - 1.0) * wander.base_variation * WANDER_TURN_VARIATION_RATE * dt;
        wander.current_variation += delta_variation;

        // Clamp current_variation so it doesn't spin wildly.
        wander.current_variation = wander
            .current_variation
            .clamp(-wander.base_variation, wander.base_variation);

        // Apply the turn rate to the direction
        wander.direction = wander
            .direction
            .rotate(Vec2::from_angle(wander.current_variation * dt));

        // Smoothly vary the movement acceleration over time
        // This allows them to organically speed up and slow down their application of force
        let delta_accel = (fastrand::f32() * 2.0 - 1.0) * WANDER_ACCEL_VARIATION_RATE * dt;
        wander.current_accel_multiplier += delta_accel;

        // Clamp the multiplier to a range so it doesn't stop entirely or go too fast.
        wander.current_accel_multiplier = wander
            .current_accel_multiplier
            .clamp(WANDER_MIN_ACCEL_MULTIPLIER, WANDER_MAX_ACCEL_MULTIPLIER);

        // Apply the force vector to the input
        input.0 += wander.direction * wander.current_accel_multiplier;
    }
}

pub fn maintain_range_from_player_system(
    player_q: Query<&PhysicalTranslation, With<Player>>,
    mut enemies: Query<
        (&PhysicalTranslation, &MaintainRange, &mut AccumulatedInput),
        Without<Player>,
    >,
) {
    let Ok(player_tf) = player_q.single() else {
        return;
    };
    let player_pos = player_tf.truncate();
    for (tf, comp, mut input) in enemies.iter_mut() {
        let desired = comp.distance;
        let pos = tf.truncate();
        let to_player = player_pos - pos;
        let dist = to_player.length();
        if dist > desired + 5.0 {
            // outside the comfortable range (with some tolerance)
            input.0 += to_player.normalize() * 1.0; // move closer (seek)
        } else if dist < desired - 5.0 {
            // inside the range, too close to target
            input.0 += (-to_player.normalize()) * 1.0; // move away (flee)
        }
    }
}

pub fn avoid_allies_system(
    mut enemies: Query<
        (
            &PhysicalTranslation,
            &Collider,
            &AvoidAllies,
            &mut AccumulatedInput,
        ),
        Without<Player>,
    >,
    allies_q: Query<(&PhysicalTranslation, &Collider), With<Ally>>, // all allies to avoid
) {
    // Pre-collect ally positions (could also filter by distance here for optimization)
    let ally_positions: Vec<(Vec2, f32)> = allies_q
        .iter()
        .filter_map(|(pos, collider)| match **collider {
            ColliderShape::Circle { radius } => Some((pos.truncate(), radius)),
            _ => None,
        })
        .collect();

    for (tf, col, avoid, mut input) in enemies.iter_mut() {
        let entity_radius = match col.0 {
            ColliderShape::Circle { radius } => radius,
            _ => 0.0,
        };

        let pos = tf.truncate();

        for (ally_pos, ally_radius) in &ally_positions {
            input.0 += get_push_vector(
                &pos,
                ally_pos,
                entity_radius,
                *ally_radius,
                avoid.radius,
                None,
            );
        }
    }
}

fn get_push_vector(
    self_pos: &Vec2,
    target_pos: &Vec2,
    self_radius: f32,
    target_radius: f32,
    max_distance: f32,
    weight: Option<f32>,
) -> Vec2 {
    let w: f32 = weight.unwrap_or(DEFAULT_PUSH_WEIGHT);

    let offset = self_pos - target_pos;
    let dist = (offset.length() - target_radius - self_radius).max(1.0);

    if dist > 0.0 && dist < max_distance {
        // steer away from target ally, stronger when very close
        let push_strength = (max_distance - dist) / max_distance; // linear falloff of force
        return (offset / dist) * push_strength * w;
    }

    Vec2::ZERO
}
