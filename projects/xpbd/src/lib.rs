use bevy::prelude::*;

mod entity;
pub use entity::*;


#[derive(Debug, Default)]
pub struct XPBDPlugin;

impl Plugin for XPBDPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AccumulatedTime { time: 0. })
            .add_systems(Startup, startup)
            .add_systems(PreUpdate, accumulate_time)
            .add_systems(Update, (simulate, sync_transforms).chain());
    }
}

#[derive(Resource, Default)]
pub struct AccumulatedTime {
    pub time: f32,
}

pub const DELTA_TIME: f32 = 1. / 60.; // 60 FPS

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle = meshes.add(Circle::new(25.0));

    let white = materials.add(Color::WHITE);

    commands.spawn((
        Name::new("Circle"),
        Mesh2d(circle.clone()),
        MeshMaterial2d(white.clone()),
        ParticleBundle::new_with_pos_and_vel(Vec2::ZERO, Vec2::new(60.0, 0.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((Name::new("Camera"), Camera2d::default()));
}

// Simple position integration system using Verlet integration
fn simulate(
    mut query: Query<(&mut Pos, &mut PrevPos)>,
    mut accumulator: ResMut<AccumulatedTime>,
) {
    while accumulator.time >= DELTA_TIME {
        integrate_positions(&mut query);
        accumulator.time -= DELTA_TIME;
    }
}

fn integrate_positions(query: &mut Query<(&mut Pos, &mut PrevPos)>) {
    for (mut pos, mut prev_pos) in query.iter_mut() {
        // Velocity is computed as the difference between current and previous position over delta time
        let velocity = (pos.0 - prev_pos.0) / DELTA_TIME;
        prev_pos.0 = pos.0;
        pos.0 = pos.0 + velocity * DELTA_TIME;
    }
}

// This applies the position component to the Bevy Transform component for rendering
fn sync_transforms(
    mut query: Query<(&mut Transform, &Pos, &PrevPos)>,
    accumulator: ResMut<AccumulatedTime>,
) {
    let alpha = accumulator.time / DELTA_TIME; // Leftover time ratio for interpolation
    for (mut transform, pos, prev_pos) in query.iter_mut() {
        // We interpolate to be somewhere inbetween the previous and current position based on alpha
        let interpolated = prev_pos.0.lerp(pos.0, alpha);
        transform.translation = interpolated.extend(0.);
    }
}

 fn accumulate_time(mut accumulator: ResMut<AccumulatedTime>, time: Res<Time>) {
    accumulator.time += time.delta_secs();
 }