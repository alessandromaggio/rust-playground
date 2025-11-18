use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.81))
    }
}

#[derive(Resource, Debug, Default)]
pub struct CollisionPairs(pub Vec<(Entity, Entity)>);

#[derive(Resource, Debug)]
pub struct Contacts(pub Vec<(Entity, Entity, Vec2)>);

impl Default for Contacts {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Resource, Debug, Default)]
pub struct StaticContacts(pub Vec<(Entity, Entity, Vec2)>);


