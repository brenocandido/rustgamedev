use crate::combat::*;
use crate::prelude::*;
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>().add_systems(
            FixedUpdate,
            (
                collision_to_damage.after(crate::physics::advance_physics),
                apply_damage.after(collision_to_damage),
            ),
        );
    }
}
