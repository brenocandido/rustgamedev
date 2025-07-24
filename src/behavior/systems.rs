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
            let away = pos - player_pos;
            let distance_sq = flee_comp.distance * flee_comp.distance;
            if away.length_squared() < distance_sq {
                steering += away.normalize() * 1.0; // FLEE_WEIGHT = 1.0
            }
        }
        // 3. Wander behavior: small random jitter
        if let Some(mut wander) = wander {
            let rand_var =
                wander.base_variation * (fastrand::f32() * 2.0 - 1.0) * fixed_time.delta_secs();
            wander.direction = wander.direction.rotate(Vec2::from_angle(rand_var));
            println!("Direction: {0:?}", wander.direction);
            steering += wander.direction * 0.0001; // WANDER_STRENGTH = 0.5
        }
        // 4. MaintainRange behavior: keep a distance from player
        if let Some(range_comp) = range {
            let desired = range_comp.distance;
            let to_player = player_pos - pos;
            let dist = to_player.length();
            if dist > desired + 5.0 {
                // outside the comfortable range (with some tolerance)
                steering += to_player.normalize() * 1.0; // move closer (seek)
            } else if dist < desired - 5.0 {
                // inside the range, too close to target
                steering += (-to_player.normalize()) * 1.0; // move away (flee)
            }
            // if within [desired-5, desired+5], maybe do nothing or a mild wander/strafe
        }
        // 5. AvoidAllies behavior: repulse from nearby allies
        if let Some(avoid_comp) = avoid {
            let entity_radius = match col.0 {
                ColliderShape::Circle { radius } => radius,
                _ => 0.0,
            };
            let r = avoid_comp.radius;
            for (ally_pos, ally_radius) in &ally_positions {
                let offset = pos - *ally_pos;
                let dist = offset.length() - ally_radius - entity_radius;
                if dist > 0.0 && dist < r {
                    // steer away from this ally, stronger when very close
                    let push_strength = (r - dist) / r; // linear falloff of force
                    steering += (offset / dist) * push_strength * 2.0; // AVOID_WEIGHT = 2.0
                }
            }
        }

        input.0 = steering;
    }
}
