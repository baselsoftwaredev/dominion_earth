use bevy::prelude::*;

/// Component that links a game entity to its visual sprite entity
///
/// This component represents the "View" in MVC architecture (following moonshine-save philosophy).
/// It should be despawned before loading a save game, as visual entities
/// should be recreated from the saved model data.
///
/// MVC Separation:
/// - Model (saved): City, Civilization, MilitaryUnit, Position, etc.
/// - View (not saved): SpriteEntityReference, UI components, Camera, etc.
#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct SpriteEntityReference {
    pub sprite_entity: Entity,
}

impl SpriteEntityReference {
    pub fn create_new_reference(sprite_entity: Entity) -> Self {
        Self { sprite_entity }
    }
}
