use crate::physics::*;
use crate::prelude::*;
use std::collections::HashMap;
use std::mem;

#[derive(Clone, Copy)]
struct Contact {
    normal: Vec2, // points *from* B to A
    penetration: f32,
}

/// System for circle (dynamic) vs rectangle (static wall) collisions.
pub fn circle_wall_collision_system(
    mut movers: Query<
        (
            Entity,
            &mut PhysicalTranslation,
            &mut Velocity,
            &Collider,
            &Mass,
        ),
        With<Velocity>,
    >,
    walls: Query<(Entity, &Transform, &Collider), Without<Velocity>>,
    mut contacts: ResMut<Contacts>,
) {
    for (mover, mut pos, mut vel, col, m) in &mut movers {
        let ColliderShape::Circle { radius } = col.0 else {
            continue;
        };
        let center = Vec2::new(pos.x, pos.y);

        for (wall, tf, wall_col) in &walls {
            let ColliderShape::Rect { half_extents } = wall_col.0 else {
                continue;
            };
            let rect_pos = tf.translation.truncate();

            if let Some(contact) = circle_vs_rect(center, radius, rect_pos, half_extents) {
                let data = circle_wall_contact_data(vel.0.truncate(), m.0, &contact);
                resolve_circle_wall(pos.reborrow(), vel.reborrow(), contact, RESTITUTION);
                contacts.current.insert(ordered_pair(mover, wall), data);
            }
        }
    }
}

/// System for circle vs circle collisions (between dynamic moving entities).
pub fn circle_circle_collision_system(
    mut q: Query<(
        Entity,
        &mut PhysicalTranslation,
        &mut Velocity,
        &Collider,
        &Mass,
    )>,
    mut contacts: ResMut<Contacts>,
) {
    let mut combos = q.iter_combinations_mut::<2>();
    while let Some(
        [
            (e1, mut p1, mut v1, col1, m1),
            (e2, mut p2, mut v2, col2, m2),
        ],
    ) = combos.fetch_next()
    {
        let ColliderShape::Circle { radius: r1 } = col1.0 else {
            continue;
        };
        let ColliderShape::Circle { radius: r2 } = col2.0 else {
            continue;
        };

        if let Some(contact) =
            circle_vs_circle(Vec2::new(p1.x, p1.y), r1, Vec2::new(p2.x, p2.y), r2)
        {
            let data =
                circle_circle_contact_data(v1.0.truncate(), v2.0.truncate(), m1.0, m2.0, &contact);

            resolve_circle_circle(
                p1.reborrow(),
                v1.reborrow(),
                m1.0,
                p2.reborrow(),
                v2.reborrow(),
                m2.0,
                contact,
                RESTITUTION,
            );

            contacts.current.insert(ordered_pair(e1, e2), data);
        }
    }
}

pub fn emit_collision_events(
    mut contacts: ResMut<Contacts>,
    mut writer: EventWriter<CollisionEvent>,
) {
    let current: HashMap<(Entity, Entity), ContactData> = mem::take(&mut contacts.current);

    for (&pair, data) in &current {
        if !contacts.prev.contains_key(&pair) {
            writer.write(CollisionEvent::Started {
                a: pair.0,
                b: pair.1,
                impulse: data.impulse,
                v_a_n: data.v_a_n,
                v_b_n: data.v_b_n,
            });
        }
    }
    for pair in contacts.prev.keys() {
        if !current.contains_key(pair) {
            writer.write(CollisionEvent::Stopped {
                a: pair.0,
                b: pair.1,
            });
        }
    }

    contacts.prev = current;
}

//--------------------------------------------------
// Helper functions
//--------------------------------------------------

fn ordered_pair(a: Entity, b: Entity) -> (Entity, Entity) {
    if a.index() < b.index() {
        (a, b)
    } else {
        (b, a)
    }
}

fn circle_vs_rect(circle_pos: Vec2, radius: f32, rect_pos: Vec2, half: Vec2) -> Option<Contact> {
    // closest point on the rectangle to the circle centre
    let delta = circle_pos - rect_pos;
    let clamped = delta.clamp(-half, half);
    let closest = rect_pos + clamped;

    let diff = circle_pos - closest;
    let dist_sq = diff.length_squared();
    if dist_sq > radius * radius {
        return None; // no overlap
    }

    let dist = dist_sq.sqrt();
    let normal = if dist != 0.0 {
        diff / dist
    } else if delta.x.abs() > delta.y.abs() {
        Vec2::new(delta.x.signum(), 0.0)
    } else {
        Vec2::new(0.0, delta.y.signum())
    };
    Some(Contact {
        normal,
        penetration: radius - dist,
    })
}

fn circle_vs_circle(p1: Vec2, r1: f32, p2: Vec2, r2: f32) -> Option<Contact> {
    let diff = p2 - p1;
    let dist_sq = diff.length_squared();
    let r_sum = r1 + r2;
    if dist_sq >= r_sum * r_sum {
        return None;
    }
    let dist = dist_sq.sqrt();
    let normal = if dist > 0.0 {
        diff / dist
    } else {
        Vec2::ONE.normalize()
    };
    Some(Contact {
        normal,
        penetration: r_sum - dist,
    })
}

fn resolve_circle_wall(
    mut circle_pos: Mut<PhysicalTranslation>,
    mut circle_vel: Mut<Velocity>,
    contact: Contact,
    restitution: f32,
) {
    // Split velocity into normal & tangential parts
    let vel2d = Vec2::new(circle_vel.x, circle_vel.y);
    let v_n = vel2d.dot(contact.normal);

    // Only flip if we were moving into the wall
    if v_n < 0.0 {
        // v' = v - (1 + e) * (v·n) * n
        let reflected = vel2d - (1.0 + restitution) * v_n * contact.normal;
        circle_vel.x = reflected.x;
        circle_vel.y = reflected.y;
    }

    circle_pos.x += contact.normal.x * contact.penetration;
    circle_pos.y += contact.normal.y * contact.penetration;
}

#[allow(clippy::too_many_arguments)]
fn resolve_circle_circle(
    mut pos1: Mut<PhysicalTranslation>,
    mut vel1: Mut<Velocity>,
    m1: f32,
    mut pos2: Mut<PhysicalTranslation>,
    mut vel2: Mut<Velocity>,
    m2: f32,
    contact: Contact,
    restitution: f32,
) {
    let n = contact.normal; // Vec2, unit, points 1 → 2
    let n3 = n.extend(0.0); // Vec3

    // ── 1. impulse (only if approaching) ────────────────────────────────
    let rel_speed = (Vec2::new(vel1.x, vel1.y) - Vec2::new(vel2.x, vel2.y)).dot(n);

    if rel_speed > 0.0 {
        // approaching
        let j = -(1.0 + restitution) * rel_speed / (1.0 / m1 + 1.0 / m2);
        let impulse = n3 * j;
        vel1.0 += impulse / m1;
        vel2.0 -= impulse / m2;
    }
    // If rel_speed ≤ 0 we skip the bounce, but we **continue** to separation

    // ── 2. depenetration (always) ───────────────────────────────────────
    if contact.penetration > 0.0 {
        let total_m = m1 + m2;
        let corr1 = contact.penetration * (m2 / total_m);
        let corr2 = contact.penetration * (m1 / total_m);

        pos1.0 -= n3 * corr1;
        pos2.0 += n3 * corr2;
    }
}

fn circle_wall_contact_data(vel: Vec2, m: f32, contact: &Contact) -> ContactData {
    let v_a_n = vel.dot(contact.normal);
    let v_b_n = 0.0; // Because it's a wall

    let impulse = if v_a_n < 0.0 {
        -(1.0 + RESTITUTION) * v_a_n * m
    } else {
        0.0
    };

    ContactData {
        impulse,
        v_a_n: -v_a_n,  // Flips the speed because the normal vector is point towards the circle.
        v_b_n,
    }
}

fn circle_circle_contact_data(
    v1: Vec2,
    v2: Vec2,
    m1: f32,
    m2: f32,
    contact: &Contact,
) -> ContactData {
    let rel_vel = v2 - v1;
    let closing = rel_vel.dot(contact.normal);

    let impulse = if closing < 0.0 {
        -(1.0 + RESTITUTION) * closing / (1.0 / m1 + 1.0 / m2)
    } else {
        0.0
    };

    let v_a_n = v1.dot(contact.normal);
    let v_b_n = v2.dot(contact.normal);

    ContactData {
        impulse,
        v_a_n,
        v_b_n,
    }
}
