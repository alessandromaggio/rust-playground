use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Debug)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 25. }
    }
}

#[derive(Component, Debug, Default)]
pub struct Vel(pub(crate) Vec2);

#[derive(Component, Debug, Default)]
pub struct PreSolveVel(pub(crate) Vec2);

#[derive(Component, Debug)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}

#[derive(Component, Debug)]
pub struct BoxCollider {
    pub size: Vec2,
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self {
            size: Vec2::new(50., 50.),
        }
    }
}