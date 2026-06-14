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
    player_q: Query<&PhysicalTranslation, With<Player>>,
    mut enemies: Query<(&PhysicalTranslation, &Flee, &mut AccumulatedInput), Without<Player>>,
) {
    let Ok(player_tf) = player_q.single() else {
        return;
    };
    let player_pos = player_tf.truncate();
    for (tf, flee, mut input) in enemies.iter_mut() {
        let pos = tf.truncate();
        let away = pos - player_pos;
        let distance_sq = flee.distance * flee.distance;

        if away.length_squared() < distance_sq {
            input.0 += away.normalize() * 1.0; // Accumulate force
        }
    }
}

pub fn wander_system(
    fixed_time: Res<Time<Fixed>>,
    mut enemies: Query<(&mut Wander, &mut AccumulatedInput), Without<Player>>,
) {
    for (mut wander, mut input) in enemies.iter_mut() {
        let rand_var =
            wander.base_variation * (fastrand::f32() * 2.0 - 1.0) * fixed_time.delta_secs();
        wander.direction = wander.direction.rotate(Vec2::from_angle(rand_var));
        println!("Direction: {0:?}", wander.direction);

        input.0 += wander.direction * 0.0001; // Accumulate force
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
        let r = avoid.radius;
        for (ally_pos, ally_radius) in &ally_positions {
            let offset = pos - *ally_pos;
            let dist = offset.length() - ally_radius - entity_radius;
            if dist > 0.0 && dist < r {
                // steer away from this ally, stronger when very close
                let push_strength = (r - dist) / r; // linear falloff of force
                input.0 += (offset / dist) * push_strength * 2.0; // AVOID_WEIGHT = 2.0
            }
        }
    }
}
