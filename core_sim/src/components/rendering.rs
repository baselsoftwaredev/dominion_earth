use bevy::prelude::*;

/// Component that links a game entity to its visual sprite entity
#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct SpriteEntityReference {
    pub sprite_entity: Entity,
}

impl SpriteEntityReference {
    pub fn create_new_reference(sprite_entity: Entity) -> Self {
        Self { sprite_entity }
    }
}
