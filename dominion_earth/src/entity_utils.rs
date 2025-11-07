use bevy::prelude::*;
use std::collections::HashSet;

/// Recursively despawn an entity and all its children.
///
/// # Deprecated
///
/// Use [`Commands::entity().despawn_recursive()`] instead. Bevy's built-in hierarchy system
/// (added in 0.16 with ChildOf/Children components) provides automatic recursive despawn
/// functionality, making this manual implementation unnecessary.
///
/// This function is kept for backwards compatibility but should not be used in new code.
///
/// # Example (Preferred)
/// ```ignore
/// commands.entity(parent).despawn_recursive();
/// ```
#[deprecated(
    since = "phase_1",
    note = "Use `Commands::entity().despawn_recursive()` instead"
)]
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
