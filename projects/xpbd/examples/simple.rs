use bevy::prelude::*;
use xpbd::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins(DefaultPlugins)
        .add_plugins(XPBDPlugin)
        .add_systems(Startup, startup)
        .run();
}

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