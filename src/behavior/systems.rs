use crate::prelude::*;
use fastrand;

#[allow(clippy::type_complexity)]
pub fn enemy_steering_system(
    fixed_time: Res<Time<Fixed>>,
    player_q: Query<&PhysicalTranslation, With<Player>>, // player position
    allies_q: Query<(&PhysicalTranslation, &Collider), With<Ally>>, // all allies to avoid
    mut enemies: Query<
        (
            &mut AccumulatedInput,
            &PhysicalTranslation,
            &Collider,
            Option<&Seek>,
            Option<&Flee>,
            Option<&mut Wander>,
            Option<&MaintainRange>,
            Option<&AvoidAllies>,
        ),
        Without<Player>,
    >,
) {
    let player_pos = if let Ok(xform) = player_q.single() {
        xform.truncate() // 3D Vec to 2D
    } else {
        return; // no player, nothing to do
    };

    // Pre-collect ally positions (could also filter by distance here for optimization)
    let ally_positions: Vec<(Vec2, f32)> = allies_q
        .iter()
        .filter_map(|(pos, collider)| match **collider {
            ColliderShape::Circle { radius } => Some((pos.truncate(), radius)),
            _ => None,
        })
        .collect();

    for (mut input, tf, col, seek, flee, wander, range, avoid) in enemies.iter_mut() {
        let pos = tf.truncate();
        let mut steering = Vec2::ZERO;

        // 1. Seek behavior: accelerate toward player
        if let Some(seek_comp) = seek {
            let to_player = player_pos - pos;
            let distance_sq = seek_comp.distance * seek_comp.distance;

            if to_player.length_squared() < distance_sq {
                steering += to_player.normalize() * 1.0; // SEEK_WEIGHT = 1.0 (example)
            }
        }
        // 2. Flee behavior: accelerate away from player
        if let Some(flee_comp) = flee {
            steering += flee_from(pos, player_pos, flee_comp);
        }
        // 3. Wander behavior: small random jitter
        if let Some(mut wander) = wander {
            steering += wander_entity(&fixed_time, &mut wander);
        }
        // 4. MaintainRange behavior: keep a distance from player
        if let Some(range_comp) = range {
            steering += maintain_range_from(pos, player_pos, range_comp)
        }
        // 5. AvoidAllies behavior: repulse from nearby allies
        if let Some(avoid_comp) = avoid {
            steering += avoid_allies(pos, col, &ally_positions, avoid_comp);
        }

        input.0 = steering;
    }
}

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

fn flee_from(self_pos: Vec2, target_pos: Vec2, comp: &Flee) -> Vec2 {
    let away = self_pos - target_pos;
    let distance_sq = comp.distance * comp.distance;
    if away.length_squared() < distance_sq {
        return away.normalize() * 1.0; // FLEE_WEIGHT = 1.0
    }

    Vec2::ZERO
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

fn wander_entity(fixed_time: &Time<Fixed>, comp: &mut Wander) -> Vec2 {
    let rand_var = comp.base_variation * (fastrand::f32() * 2.0 - 1.0) * fixed_time.delta_secs();
    comp.direction = comp.direction.rotate(Vec2::from_angle(rand_var));
    println!("Direction: {0:?}", comp.direction);

    comp.direction * 0.0001 // WANDER_STRENGTH = 0.5
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

fn maintain_range_from(self_pos: Vec2, target_pos: Vec2, comp: &MaintainRange) -> Vec2 {
    let desired = comp.distance;
    let to_player = target_pos - self_pos;
    let dist = to_player.length();
    if dist > desired + 5.0 {
        // outside the comfortable range (with some tolerance)
        return to_player.normalize() * 1.0; // move closer (seek)
    } else if dist < desired - 5.0 {
        // inside the range, too close to target
        return (-to_player.normalize()) * 1.0; // move away (flee)
    }

    // if within [desired-5, desired+5], maybe do nothing or a mild wander/strafe

    Vec2::ZERO
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

fn avoid_allies(
    self_pos: Vec2,
    self_collider: &Collider,
    allies_pos: &Vec<(Vec2, f32)>,
    comp: &AvoidAllies,
) -> Vec2 {
    let entity_radius = match self_collider.0 {
        ColliderShape::Circle { radius } => radius,
        _ => 0.0,
    };
    let r = comp.radius;
    for (ally_pos, ally_radius) in allies_pos {
        let offset = self_pos - *ally_pos;
        let dist = offset.length() - ally_radius - entity_radius;
        if dist > 0.0 && dist < r {
            // steer away from this ally, stronger when very close
            let push_strength = (r - dist) / r; // linear falloff of force
            return (offset / dist) * push_strength * 2.0; // AVOID_WEIGHT = 2.0
        }
    }

    Vec2::ZERO
}
