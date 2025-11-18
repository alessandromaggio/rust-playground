use bevy::prelude::*;
use xpbd::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.8, 0.8, 0.9)))
        .insert_resource(Gravity(Vec2::new(0., 0.)))
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
    let blue = materials.add(Color::srgb(0.4, 0.4, 0.6));
    let size = Vec2::new(350., 100.);

    commands.spawn((
        Name::new("Floor"),
        Mesh2d(meshes.add(Mesh::from(Rectangle::new(size.x, size.y)))),
        MeshMaterial2d(blue.clone()),
        StaticBoxBundle {
            pos: Pos(Vec2::new(0., -62.)),
            collider: BoxCollider { size: size  },
            ..default()
        }
    ));

    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Transform {
            translation: Vec3::new(0., 3., 100.),
            scale: Vec3::splat(1.),
            ..default()
        },
        GlobalTransform::default(),
    ));

    let radius = 10.;
    let stacks = 5;

    for i in 0..15 {
        for j in 0..stacks {
            let pos = Vec2::new(
                (j as f32 - stacks as f32 / 2.) * 2.5 * radius,
                2. * radius * i as f32 - 2.
            );
            let vel = Vec2::ZERO;

            commands.spawn((
                Name::new("Circle"),
                Mesh2d(meshes.add(Mesh::from(Circle::new(radius).clone()))),
                MeshMaterial2d(blue.clone()),
                ParticleBundle::new_with_pos_vel_mass_radius(pos, vel, 10., radius),
                Transform::from_translation(pos.extend(0.))
            ));
        }
    }

    
}