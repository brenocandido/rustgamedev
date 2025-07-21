use crate::prelude::*;

pub fn collision_to_damage(
    mut ev_collision: EventReader<CollisionEvent>,
    mut ev_damage: EventWriter<DamageEvent>,

    q_player: Query<(), With<Player>>,
    q_enemy: Query<(), With<Enemy>>,
) {
    for col in ev_collision.read() {
        let a_is_player = q_player.contains(col.0);
        let b_is_player = q_player.contains(col.1);
        let a_is_enemy = q_enemy.contains(col.0);
        let b_is_enemy = q_enemy.contains(col.1);

        // keep only Player ↔ Enemy contacts, in either order
        if (a_is_player && b_is_enemy) || (a_is_enemy && b_is_player) {
            let dmg = 25.0;

            let victim = if a_is_enemy { col.0 } else { col.1 };
            ev_damage.write(DamageEvent {
                victim,
                amount: dmg,
            });
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
