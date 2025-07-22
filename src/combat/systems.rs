use crate::combat::*;
use crate::prelude::*;

pub fn collision_to_damage(
    mut ev_collision: EventReader<CollisionEvent>,
    mut ev_damage: EventWriter<DamageEvent>,

    cfg: Res<PhysicsConfig>,

    q_player: Query<(), With<Player>>,
    q_enemy: Query<(), With<Enemy>>,
) {
    for ev in ev_collision.read() {
        if let CollisionEvent::Started {
            a,
            b,
            impulse,
            v_a_n,
            v_b_n,
        } = *ev
        {
            println!("Collision impulse: {impulse}. v_a_n: {v_a_n}. v_b_n: {v_b_n}.");

            // Identify which side is the player / enemy
            let a_is_player = q_player.contains(a);
            let b_is_player = q_player.contains(b);
            let a_is_enemy = q_enemy.contains(a);
            let b_is_enemy = q_enemy.contains(b);

            if (a_is_player && b_is_enemy) || (a_is_enemy && b_is_player) {
                let victim = if a_is_enemy { a } else { b };
                let v_n = if a_is_player { v_a_n } else { v_b_n };
                let v_norm = v_n / cfg.max_speed;
                let damage = (v_norm * BASE_COLLISION_DAMAGE).max(0.0);

                println!("Collision damage: {damage}.");

                ev_damage.write(DamageEvent {
                    victim,
                    amount: damage,
                });
            }
        }
    }
}

pub fn apply_damage(
    mut ev_damage: EventReader<DamageEvent>,
    mut q_health: Query<&mut Health>,
    mut commands: Commands,
) {
    for ev in ev_damage.read() {
        if let Ok(mut hp) = q_health.get_mut(ev.victim) {
            hp.damage(ev.amount);
            if hp.is_dead() {
                println!("Entity {:?} died", ev.victim);
                commands.entity(ev.victim).despawn();
            }
        }
    }
}
