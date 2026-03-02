use crate::prelude::*;

#[derive(Component)]
pub struct Node;

#[derive(Component)]
pub struct Constellation(pub Vec<Entity>);
