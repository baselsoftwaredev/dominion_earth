use super::civilization::CivId;
use super::terrain::TerrainType;
use bevy::prelude::Reflect;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// City component representing a settlement
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct City {
    pub name: String,
    pub owner: CivId,
    pub population: u32,
    pub production: f32,
    pub defense: f32,
    pub buildings: Vec<Building>,
}

// Manual Component implementation
impl Component for City {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl City {
    pub fn new(name: String, owner: CivId) -> Self {
        Self {
            name,
            owner,
            population: 1000,
            production: 5.0, // Restored to proper turn-based production value
            defense: 10.0,
            buildings: vec![Building {
                building_type: BuildingType::Granary,
                level: 1,
            }],
        }
    }

    pub fn add_building(&mut self, building_type: BuildingType) {
        self.buildings.push(Building {
            building_type,
            level: 1,
        });
        self.update_stats();
    }

    pub fn upgrade_building(&mut self, building_type: &BuildingType) -> bool {
        if let Some(building) = self
            .buildings
            .iter_mut()
            .find(|b| &b.building_type == building_type)
        {
            building.level += 1;
            self.update_stats();
            true
        } else {
            false
        }
    }

    fn update_stats(&mut self) {
        // Recalculate city stats based on buildings
        let mut total_production = 5.0; // Base production
        let mut total_defense = 10.0; // Base defense

        for building in &self.buildings {
            let (prod_bonus, def_bonus) = building.building_type.bonuses();
            total_production += prod_bonus * building.level as f32;
            total_defense += def_bonus * building.level as f32;
        }

        self.production = total_production;
        self.defense = total_defense;
    }

    pub fn grow_population(&mut self, amount: u32) {
        self.population += amount;
    }
}

/// Capital component for tracking civilization capitals through different ages
#[derive(Debug, Clone)]
pub struct Capital {
    pub owner: CivId,
    pub age: CapitalAge,
    pub sprite_index: u32,
    pub established_turn: u32,
}

// Manual Component implementation
impl Component for Capital {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Ages that a capital can evolve through
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapitalAge {
    Neolithic,
    Bronze,
    Iron,
    Classical,
    Medieval,
    Renaissance,
    Industrial,
    Modern,
    Information,
    Future,
}

impl CapitalAge {
    /// Get the sprite index for this capital age
    pub fn sprite_index(&self) -> u32 {
        match self {
            CapitalAge::Neolithic => 3, // Use index 3 as requested
            CapitalAge::Bronze => 5,    // You can assign these as you add more sprites
            CapitalAge::Iron => 6,
            CapitalAge::Classical => 7,
            CapitalAge::Medieval => 8,
            CapitalAge::Renaissance => 9,
            CapitalAge::Industrial => 10,
            CapitalAge::Modern => 11,
            CapitalAge::Information => 12,
            CapitalAge::Future => 13,
        }
    }

    /// Get the next age for capital evolution
    pub fn next_age(&self) -> Option<CapitalAge> {
        match self {
            CapitalAge::Neolithic => Some(CapitalAge::Bronze),
            CapitalAge::Bronze => Some(CapitalAge::Iron),
            CapitalAge::Iron => Some(CapitalAge::Classical),
            CapitalAge::Classical => Some(CapitalAge::Medieval),
            CapitalAge::Medieval => Some(CapitalAge::Renaissance),
            CapitalAge::Renaissance => Some(CapitalAge::Industrial),
            CapitalAge::Industrial => Some(CapitalAge::Modern),
            CapitalAge::Modern => Some(CapitalAge::Information),
            CapitalAge::Information => Some(CapitalAge::Future),
            CapitalAge::Future => None, // Max evolution
        }
    }

    /// Get the requirements for evolving to the next age
    pub fn evolution_requirements(&self) -> CapitalEvolutionRequirements {
        match self {
            CapitalAge::Neolithic => CapitalEvolutionRequirements {
                min_population: 2000,
                required_technologies: vec!["Bronze Working".to_string()],
                min_buildings: 2,
                min_turn: 10,
            },
            CapitalAge::Bronze => CapitalEvolutionRequirements {
                min_population: 4000,
                required_technologies: vec!["Iron Working".to_string()],
                min_buildings: 3,
                min_turn: 25,
            },
            CapitalAge::Iron => CapitalEvolutionRequirements {
                min_population: 8000,
                required_technologies: vec!["Currency".to_string(), "Writing".to_string()],
                min_buildings: 4,
                min_turn: 50,
            },
            // Add more requirements as needed for other ages
            _ => CapitalEvolutionRequirements {
                min_population: 10000,
                required_technologies: vec![],
                min_buildings: 5,
                min_turn: 100,
            },
        }
    }
}

/// Requirements for capital evolution
#[derive(Debug, Clone)]
pub struct CapitalEvolutionRequirements {
    pub min_population: u32,
    pub required_technologies: Vec<String>,
    pub min_buildings: usize,
    pub min_turn: u32,
}

/// Building in a city
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: u32,
}

// Manual Component implementation
impl Component for Building {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

/// Types of buildings that can be constructed
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum BuildingType {
    Granary,
    Barracks,
    Workshop,
    Library,
    Walls,
    Market,
    Temple,
}

impl BuildingType {
    /// Returns (production_bonus, defense_bonus) per level
    pub fn bonuses(&self) -> (f32, f32) {
        match self {
            BuildingType::Granary => (2.0, 0.0),
            BuildingType::Barracks => (0.0, 5.0),
            BuildingType::Workshop => (3.0, 0.0),
            BuildingType::Library => (1.0, 0.0),
            BuildingType::Walls => (0.0, 10.0),
            BuildingType::Market => (1.5, 0.0),
            BuildingType::Temple => (0.5, 2.0),
        }
    }

    pub fn cost(&self) -> f32 {
        match self {
            BuildingType::Granary => 40.0,
            BuildingType::Barracks => 60.0,
            BuildingType::Workshop => 80.0,
            BuildingType::Library => 100.0,
            BuildingType::Walls => 120.0,
            BuildingType::Market => 90.0,
            BuildingType::Temple => 70.0,
        }
    }

    pub fn maintenance_cost(&self) -> f32 {
        self.cost() * 0.05 // 5% of build cost per turn
    }

    pub fn production_cost(&self) -> f32 {
        match self {
            BuildingType::Granary => 30.0,
            BuildingType::Barracks => 45.0,
            BuildingType::Workshop => 60.0,
            BuildingType::Library => 75.0,
            BuildingType::Walls => 90.0,
            BuildingType::Market => 70.0,
            BuildingType::Temple => 55.0,
        }
    }

    pub fn gold_cost(&self) -> f32 {
        self.cost()
    }

    pub fn name(&self) -> &'static str {
        match self {
            BuildingType::Granary => "Granary",
            BuildingType::Barracks => "Barracks",
            BuildingType::Workshop => "Workshop",
            BuildingType::Library => "Library",
            BuildingType::Walls => "Walls",
            BuildingType::Market => "Market",
            BuildingType::Temple => "Temple",
        }
    }
}

/// Territory control component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Territory {
    pub owner: CivId,
    pub control_strength: f32,
    pub terrain_type: TerrainType,
}

// Manual Component implementation
impl Component for Territory {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}
