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
            .insert_resource(Gravity(Vec2::ZERO))
            .add_systems(FixedUpdate, (
                collect_collision_pairs,
                integrate,
                update_vel
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

fn collect_collision_pairs(mut query: Query<(&mut Pos, &Mass, &CircleCollider)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some(
        [(mut pos_a, mass_a, collider_a), (mut pos_b, mass_b, collider_b)]
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
            pos_a.0 -= n * penetration_depth * w_a / w_sum;
            pos_b.0 += n * penetration_depth * w_b / w_sum;
        }
    }
}

fn integrate(mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel, &Mass)>, gravity: Res<Gravity>) {
    for (mut pos, mut prev_pos, mut vel, mass) in query.iter_mut() {
        prev_pos.0 = pos.0;

        let gravitation_force = mass.0 * gravity.0;
        let external_forces = gravitation_force;
        vel.0 += DELTA_TIME * external_forces / mass.0;
        pos.0 += DELTA_TIME * vel.0;
    }
}

fn solve_pos() {}

fn update_vel(mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel)>) {
    for (pos, prev_pos, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - prev_pos.0) / DELTA_TIME;
    }
}

fn solve_vel() {}

// This applies the position component to the Bevy Transform component for rendering
fn sync_transforms(
    mut query: Query<(&mut Transform, &Pos)>,
) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
}