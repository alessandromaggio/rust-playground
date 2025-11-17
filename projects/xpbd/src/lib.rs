use bevy::prelude::*;

mod components;
mod entity;

pub use components::*;
pub use entity::*;


#[derive(Debug, Default)]
pub struct XPBDPlugin;

pub const FIXED_TIMESTEP_INTERVAL: f32 = 64.; // FPS
pub const DELTA_TIME: f32 = 1. / FIXED_TIMESTEP_INTERVAL;

impl Plugin for XPBDPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_hz(FIXED_TIMESTEP_INTERVAL.into()))
            .insert_resource(Contacts::default())
            .insert_resource(StaticContacts::default())
            .add_systems(FixedUpdate, (
                collect_collision_pairs,
                integrate,
                solve_pos,
                solve_pos_statics,
                update_vel,
                solve_vel,
                solve_vel_statics,
                sync_transforms
            ).chain())
            .add_systems(Update, sync_transforms);
    }
}

#[derive(Resource, Debug)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.81))
    }
}

#[derive(Resource, Debug)]
pub struct Contacts(pub Vec<(Entity, Entity, Vec2)>);

impl Default for Contacts {
    fn default() -> Self {
        Self(Vec::new())
    }
}

fn collect_collision_pairs() {
    
}

#[derive(Resource, Debug, Default)]
pub struct StaticContacts(pub Vec<(Entity, Entity, Vec2)>);

fn integrate(mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel, &mut PreSolveVel, &Mass)>, gravity: Res<Gravity>) {
    for (mut pos, mut prev_pos, mut vel, mut pre_solve_vel, mass) in query.iter_mut() {
        prev_pos.0 = pos.0;

        let gravitation_force = mass.0 * gravity.0;
        let external_forces = gravitation_force;
        vel.0 += DELTA_TIME * external_forces / mass.0;
        pos.0 += DELTA_TIME * vel.0;
        pre_solve_vel.0 = vel.0;
    }
}

fn solve_pos(
    mut query: Query<(Entity, &mut Pos, &Mass, &CircleCollider)>,
    mut contacts: ResMut<Contacts>
) {
    contacts.0.clear();
    let mut iter = query.iter_combinations_mut();
    while let Some(
        [(entity_a, mut pos_a, mass_a, collider_a), (entity_b, mut pos_b, mass_b, collider_b)]
    ) = iter.fetch_next() {
        let ab = pos_b.0 - pos_a.0;
        let combined_radius = collider_a.radius + collider_b.radius;
        let ab_sqr_len = ab.length_squared();
        if ab_sqr_len < combined_radius * combined_radius {
            let ab_length = ab_sqr_len.sqrt();
            let n = ab / ab_length; // Normalization
            let penetration_depth = combined_radius - ab_length;

            let w_a = 1. / mass_a.0; // Inverse of mass
            let w_b = 1. / mass_b.0; // Inverse of mass
            let w_sum = w_a + w_b;

            // How much an object is to be affected by a collision is proportional to its inverse of mass
            pos_a.0 += n * penetration_depth * w_a / w_sum;
            pos_b.0 += n * penetration_depth * w_b / w_sum;

            contacts.0.push((entity_a, entity_b, n));
        }
    }
}

fn solve_pos_statics(
    mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>,
    statics: Query<(Entity, &Pos, &CircleCollider), Without<Mass>>,
    mut contacts: ResMut<StaticContacts>
) {
    contacts.0.clear();
    for (entity_a, mut pos_a, collider_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, collider_b) in statics.iter() {
            let ab = pos_b.0 - pos_a.0;
            let combined_radius = collider_a.radius + collider_b.radius;
            let ab_sqr_len = ab.length_squared();
            if ab_sqr_len < combined_radius * combined_radius {
                let ab_length = ab_sqr_len.sqrt();
                let penetration_depth = combined_radius - ab_length;
                let n = ab / ab_length;
                pos_a.0 -= n * penetration_depth;
                contacts.0.push((entity_a, entity_b, n));
            }
        }
    }
}

fn update_vel(mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel)>) {
    for (pos, prev_pos, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - prev_pos.0) / DELTA_TIME;
    }
}

fn solve_vel(
    query: Query<(&mut Vel, &PreSolveVel, &Mass, &Restitution)>,
    contacts: Res<Contacts>
) {
    for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
        let (
            (mut vel_a, pre_solve_vel_a, mass_a, restitution_a),
            (mut vel_b, pre_solve_vel_b, mass_b, restituion_b)
        ) = unsafe {
            assert!(entity_a != entity_b); // Ensure safety
            (
                query.get_unchecked(entity_a).unwrap(),
                query.get_unchecked(entity_b).unwrap()
            )
        };

        let pre_solve_relative_vel = pre_solve_vel_a.0 - pre_solve_vel_b.0;
        let pre_solve_normal_vel = Vec2::dot(pre_solve_relative_vel, n);

        let relative_vel = vel_a.0 - vel_b.0;
        let normal_vel = Vec2::dot(relative_vel, n);
        let restitution = (restitution_a.0 + restituion_b.0) / 2.;

        let w_a = 1. / mass_a.0;
        let w_b = 1. / mass_b.0;
        let w_sum = w_a + w_b;

        vel_a.0 = n * (-normal_vel - restitution * pre_solve_normal_vel) * w_a / w_sum;
        vel_b.0 = n * (-normal_vel - restitution * pre_solve_normal_vel) * w_b / w_sum;
    }
}

fn solve_vel_statics(
    mut dynamics: Query<(&mut Vel, &PreSolveVel, &Restitution), With<Mass>>,
    statics: Query<&Restitution, Without<Mass>>,
    contacts: Res<StaticContacts>
) {
    for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
        let (mut vel_a, pre_solve_vel_a, restitution_a) =
            dynamics.get_mut(entity_a).expect(&format!("Could not unwrap dynamic entity {:?}", entity_a));
        let restitution_b = statics.get(entity_b).expect(&format!("Could not unwrap static entity {:?}", entity_b));
        let pre_solve_normal_vel = Vec2::dot(pre_solve_vel_a.0, n);
        let normal_vel = Vec2::dot(vel_a.0, n);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.;
        vel_a.0 += n * (-normal_vel - restitution * pre_solve_normal_vel);
    }
}

// This applies the position component to the Bevy Transform component for rendering
fn sync_transforms(
    mut query: Query<(&mut Transform, &Pos)>,
) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
}