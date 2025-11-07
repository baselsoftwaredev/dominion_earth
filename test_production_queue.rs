/// Quick test to verify production queue entity-based system works
/// Run with: cargo test test_production_queue -- --nocapture

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;
    use core_sim::{CivId, ProductionItem, ProductionQueue, QueueItem, UnitType};

    #[test]
    fn test_queue_item_creation_and_retrieval() {
        let mut world = World::new();
        let mut query_state = world.query::<&QueueItem>();

        // Create a queue item
        let item = ProductionItem::Unit(UnitType::Infantry);
        let entity = world.spawn(QueueItem(item.clone())).id();

        // Retrieve it
        let retrieved = query_state
            .get(&world, entity)
            .expect("Failed to get queue item");
        assert_eq!(retrieved.0, item);
        println!("✅ Queue item created and retrieved successfully");
    }

    #[test]
    fn test_production_queue_add_items() {
        let mut world = World::new();

        // Create a production queue
        let civ_id = CivId(0);
        let mut queue = ProductionQueue::new(civ_id);

        // Spawn queue item entities
        let item1 = ProductionItem::Unit(UnitType::Infantry);
        let entity1 = world.spawn(QueueItem(item1.clone())).id();

        let item2 = ProductionItem::Unit(UnitType::Archer);
        let entity2 = world.spawn(QueueItem(item2.clone())).id();

        // Add to queue
        queue.add_to_queue(entity1);
        queue.add_to_queue(entity2);

        assert_eq!(queue.queue.len(), 2);
        assert_eq!(queue.queue[0], entity1);
        assert_eq!(queue.queue[1], entity2);
        println!("✅ Items added to queue successfully");
    }

    #[test]
    fn test_production_queue_start_next_production() {
        let mut world = World::new();
        let civ_id = CivId(0);
        let mut queue = ProductionQueue::new(civ_id);

        let item = ProductionItem::Unit(UnitType::Infantry);
        let entity = world.spawn(QueueItem(item.clone())).id();

        queue.add_to_queue(entity);
        assert_eq!(queue.current_production, None);

        let started = queue.start_next_production();
        assert_eq!(started, Some(entity));
        assert_eq!(queue.current_production, Some(entity));
        assert_eq!(queue.queue.len(), 0);
        println!("✅ Production started from queue successfully");
    }

    #[test]
    fn test_production_queue_clear() {
        let mut world = World::new();
        let civ_id = CivId(0);
        let mut queue = ProductionQueue::new(civ_id);

        let item1 = ProductionItem::Unit(UnitType::Infantry);
        let entity1 = world.spawn(QueueItem(item1)).id();

        let item2 = ProductionItem::Unit(UnitType::Archer);
        let entity2 = world.spawn(QueueItem(item2)).id();

        queue.add_to_queue(entity1);
        queue.add_to_queue(entity2);
        queue.start_next_production();

        let cleared = queue.clear();
        assert_eq!(cleared.len(), 3); // current + 2 queued
        assert_eq!(queue.queue.len(), 0);
        assert_eq!(queue.current_production, None);
        println!("✅ Queue cleared successfully");
    }
}
