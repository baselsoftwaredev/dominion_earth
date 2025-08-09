use crate::{Position, WorldMap, MapTile};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// A* pathfinding implementation optimized for the game world
pub struct Pathfinder {
    cache: HashMap<(Position, Position), Option<Vec<Position>>>,
}

impl Default for Pathfinder {
    fn default() -> Self {
        Self::new()
    }
}

impl Pathfinder {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn find_path(&mut self, world_map: &WorldMap, start: Position, goal: Position, max_movement: f32) -> Option<Vec<Position>> {
        // Check cache first
        let cache_key = (start, goal);
        if let Some(cached_result) = self.cache.get(&cache_key) {
            return cached_result.clone();
        }

        let result = self.a_star(world_map, start, goal, max_movement);
        
        // Cache the result
        self.cache.insert(cache_key, result.clone());
        
        result
    }

    fn a_star(&self, world_map: &WorldMap, start: Position, goal: Position, max_movement: f32) -> Option<Vec<Position>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<Position, Position> = HashMap::new();
        let mut g_score: HashMap<Position, f32> = HashMap::new();
        let mut f_score: HashMap<Position, f32> = HashMap::new();
        let mut closed_set: HashSet<Position> = HashSet::new();

        g_score.insert(start, 0.0);
        f_score.insert(start, self.heuristic(start, goal));
        open_set.push(AStarNode {
            position: start,
            f_score: self.heuristic(start, goal),
        });

        while let Some(current_node) = open_set.pop() {
            let current = current_node.position;

            if current == goal {
                return Some(self.reconstruct_path(&came_from, current));
            }

            closed_set.insert(current);

            for neighbor in world_map.neighbors(current) {
                if closed_set.contains(&neighbor) {
                    continue;
                }

                let Some(neighbor_tile) = world_map.get_tile(neighbor) else {
                    continue;
                };

                // Skip impassable terrain
                if neighbor_tile.movement_cost >= 10.0 {
                    continue;
                }

                let tentative_g_score = g_score.get(&current).unwrap_or(&f32::INFINITY) + neighbor_tile.movement_cost;

                // Check if this path exceeds movement limit
                if tentative_g_score > max_movement {
                    continue;
                }

                let neighbor_g_score = *g_score.get(&neighbor).unwrap_or(&f32::INFINITY);

                if tentative_g_score < neighbor_g_score {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    let neighbor_f_score = tentative_g_score + self.heuristic(neighbor, goal);
                    f_score.insert(neighbor, neighbor_f_score);

                    // Add to open set if not already there
                    if !open_set.iter().any(|node| node.position == neighbor) {
                        open_set.push(AStarNode {
                            position: neighbor,
                            f_score: neighbor_f_score,
                        });
                    }
                }
            }
        }

        None
    }

    fn heuristic(&self, a: Position, b: Position) -> f32 {
        // Manhattan distance as heuristic
        ((a.x - b.x).abs() + (a.y - b.y).abs()) as f32
    }

    fn reconstruct_path(&self, came_from: &HashMap<Position, Position>, mut current: Position) -> Vec<Position> {
        let mut path = vec![current];
        while let Some(&previous) = came_from.get(&current) {
            current = previous;
            path.push(current);
        }
        path.reverse();
        path
    }

    /// Find all positions reachable within movement range
    pub fn find_reachable_positions(&self, world_map: &WorldMap, start: Position, max_movement: f32) -> HashSet<Position> {
        let mut reachable = HashSet::new();
        let mut distances: HashMap<Position, f32> = HashMap::new();
        let mut open_set = BinaryHeap::new();

        distances.insert(start, 0.0);
        open_set.push(DijkstraNode {
            position: start,
            distance: 0.0,
        });

        while let Some(current_node) = open_set.pop() {
            let current = current_node.position;
            let current_distance = current_node.distance;

            if current_distance > max_movement {
                continue;
            }

            reachable.insert(current);

            for neighbor in world_map.neighbors(current) {
                let Some(neighbor_tile) = world_map.get_tile(neighbor) else {
                    continue;
                };

                // Skip impassable terrain
                if neighbor_tile.movement_cost >= 10.0 {
                    continue;
                }

                let new_distance = current_distance + neighbor_tile.movement_cost;
                
                if new_distance <= max_movement {
                    let current_neighbor_distance = *distances.get(&neighbor).unwrap_or(&f32::INFINITY);
                    
                    if new_distance < current_neighbor_distance {
                        distances.insert(neighbor, new_distance);
                        open_set.push(DijkstraNode {
                            position: neighbor,
                            distance: new_distance,
                        });
                    }
                }
            }
        }

        reachable
    }

    /// Find nearest position of a specific type
    pub fn find_nearest<F>(&self, world_map: &WorldMap, start: Position, predicate: F, max_search_distance: i32) -> Option<Position>
    where
        F: Fn(&MapTile) -> bool,
    {
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((current, distance)) = queue.pop_front() {
            if distance > max_search_distance {
                continue;
            }

            if let Some(tile) = world_map.get_tile(current) {
                if predicate(tile) && current != start {
                    return Some(current);
                }
            }

            for neighbor in world_map.neighbors(current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct AStarNode {
    position: Position,
    f_score: f32,
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl Eq for AStarNode {}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug, Clone)]
struct DijkstraNode {
    position: Position,
    distance: f32,
}

impl PartialEq for DijkstraNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for DijkstraNode {}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.distance.partial_cmp(&self.distance).unwrap_or(Ordering::Equal)
    }
}
