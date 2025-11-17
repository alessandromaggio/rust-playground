use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
use xpbd::*;
use rand::random;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.8, 0.8, 0.9)))
        .insert_resource(Gravity::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(XPBDPlugin)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, spawn_marbles.run_if(on_timer(Duration::from_millis(250))))
        .add_systems(FixedUpdate, visible_area.run_if(on_timer(Duration::from_millis(500))))
        .add_systems(FixedUpdate, despawn_marbles)
        .run();
}

#[derive(Resource)]
struct Materials {
    blue: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct Meshes {
    circle: Handle<Mesh>,
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let blue = materials.add(Color::srgb(0.4, 0.4, 0.6));

    commands.insert_resource(Meshes {
        circle: meshes.add(Mesh::from(Circle::new(2.5).clone()))
    });
    commands.insert_resource(Materials {
        blue: blue.clone()
    });

    let size = Vec2::new(350., 100.);

    commands.spawn((
        Name::new("Floor"),
        Mesh2d(meshes.add(Mesh::from(Rectangle::new(size.x, size.y)))),
        MeshMaterial2d(blue.clone()),
        StaticBoxBundle {
            pos: Pos(Vec2::new(0., -355.)),
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
}

fn visible_area(
    cam_q: Query<(&GlobalTransform, &Camera), With<Camera2d>>,
) {
    let Ok((transform, camera)) = cam_q.single() else {
        info!("No Camera2d found");
        return;
    };

    if let Some(viewport_size) = camera.logical_viewport_size() {
        let pos = transform.translation().truncate();
        let scale = transform.scale().truncate();

        let half_w = viewport_size.x / 2.0 / scale.x;
        let half_h = viewport_size.y / 2.0 / scale.y;

        info!(
            "Visible world: left={:.2}, right={:.2}, bottom={:.2}, top={:.2}",
            pos.x - half_w,
            pos.x + half_w,
            pos.y - half_h,
            pos.y + half_h
        );
    } else {
        info!("Camera viewport size not available yet");
    }
}

fn spawn_marbles(
    mut commands: Commands,
    materials: Res<Materials>,
    meshes: Res<Meshes>
) {
    let radius = 2.5;
    let pos = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5) * 25. + Vec2::Y * 3.;
    let vel = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5);

    info!("Spawn marble: pos={:?}, vel={:?}", pos, vel);

    commands.spawn((
        Name::new("Marble"),
        Mesh2d(meshes.circle.clone()),
        MeshMaterial2d(materials.blue.clone()),
        ParticleBundle::new_with_pos_vel_mass_radius(pos, vel, 1., radius),
        Transform::from_translation(pos.extend(0.))
    ));
}

fn despawn_marbles(mut commands: Commands, query: Query<(Entity, &Pos)>) {
    for (entity, pos) in query.iter() {
        if pos.0.y < -360. {
            info!("Despawn marble: pos={:?}", pos.0);
            commands.entity(entity).despawn();
        }
    }
}
