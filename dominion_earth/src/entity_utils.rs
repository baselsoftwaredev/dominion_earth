use bevy::prelude::*;
use std::collections::HashSet;

pub fn recursively_despawn_entity_with_children(
    commands: &mut Commands,
    entity: Entity,
    children_query: &Query<&Children>,
    despawned: &mut HashSet<Entity>,
) {
    if despawned.contains(&entity) {
        return;
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children {
            recursively_despawn_entity_with_children(commands, *child, children_query, despawned);
        }
    }

    commands.entity(entity).despawn();
    despawned.insert(entity);
}
