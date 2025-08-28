use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Terrain types for world map tiles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TerrainType {
    Plains,
    Hills,
    Mountains,
    Forest,
    Desert,
    Coast,
    ShallowCoast,
    Ocean,
    River,
}

// Manual Component implementation
impl Component for TerrainType {
    type Mutability = Mutable;
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}

impl TerrainType {
    /// Get movement cost for this terrain type
    pub fn movement_cost(&self) -> f32 {
        match self {
            TerrainType::Plains => 1.0,
            TerrainType::Hills => 2.0,
            TerrainType::Mountains => 3.0,
            TerrainType::Forest => 1.5,
            TerrainType::Desert => 2.0,
            TerrainType::Coast => 1.0,
            TerrainType::ShallowCoast => 2.0,
            TerrainType::Ocean => f32::INFINITY, // Impassable for land units
            TerrainType::River => 1.0,
        }
    }

    /// Get defensive bonus for this terrain type
    pub fn defensive_bonus(&self) -> f32 {
        match self {
            TerrainType::Plains => 0.0,
            TerrainType::Hills => 0.25,
            TerrainType::Mountains => 0.5,
            TerrainType::Forest => 0.2,
            TerrainType::Desert => 0.0,
            TerrainType::Coast => 0.1,
            TerrainType::ShallowCoast => 0.0,
            TerrainType::Ocean => 0.0,
            TerrainType::River => 0.15,
        }
    }

    /// Get resource yield for this terrain type (food, production, gold)
    pub fn resource_yield(&self) -> (f32, f32, f32) {
        match self {
            TerrainType::Plains => (2.0, 1.0, 0.0),
            TerrainType::Hills => (0.0, 2.0, 1.0),
            TerrainType::Mountains => (0.0, 1.0, 2.0),
            TerrainType::Forest => (1.0, 2.0, 0.0),
            TerrainType::Desert => (0.0, 0.0, 1.0),
            TerrainType::Coast => (2.0, 0.0, 2.0),
            TerrainType::ShallowCoast => (1.0, 0.0, 1.0),
            TerrainType::Ocean => (1.0, 0.0, 1.0),
            TerrainType::River => (3.0, 0.0, 1.0),
        }
    }

    /// Check if this terrain type is buildable (can place cities/buildings)
    pub fn is_buildable(&self) -> bool {
        match self {
            TerrainType::Plains
            | TerrainType::Hills
            | TerrainType::Forest
            | TerrainType::Desert
            | TerrainType::Coast
            | TerrainType::River => true,
            TerrainType::Mountains | TerrainType::ShallowCoast | TerrainType::Ocean => false,
        }
    }

    /// Check if this terrain type is water (for naval units)
    pub fn is_water(&self) -> bool {
        matches!(
            self,
            TerrainType::Ocean | TerrainType::Coast | TerrainType::ShallowCoast | TerrainType::River
        )
    }

    /// Check if this terrain type is land (for land units)
    pub fn is_land(&self) -> bool {
        !self.is_water()
    }
}

// Manual Serialize/Deserialize implementation
impl Serialize for TerrainType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TerrainType::Plains => serializer.serialize_str("Plains"),
            TerrainType::Hills => serializer.serialize_str("Hills"),
            TerrainType::Mountains => serializer.serialize_str("Mountains"),
            TerrainType::Forest => serializer.serialize_str("Forest"),
            TerrainType::Desert => serializer.serialize_str("Desert"),
            TerrainType::Coast => serializer.serialize_str("Coast"),
            TerrainType::ShallowCoast => serializer.serialize_str("ShallowCoast"),
            TerrainType::Ocean => serializer.serialize_str("Ocean"),
            TerrainType::River => serializer.serialize_str("River"),
        }
    }
}

impl<'de> Deserialize<'de> for TerrainType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Plains" => Ok(TerrainType::Plains),
            "Hills" => Ok(TerrainType::Hills),
            "Mountains" => Ok(TerrainType::Mountains),
            "Forest" => Ok(TerrainType::Forest),
            "Desert" => Ok(TerrainType::Desert),
            "Coast" => Ok(TerrainType::Coast),
            "ShallowCoast" => Ok(TerrainType::ShallowCoast),
            "Ocean" => Ok(TerrainType::Ocean),
            "River" => Ok(TerrainType::River),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &[
                    "Plains",
                    "Hills",
                    "Mountains",
                    "Forest",
                    "Desert",
                    "Coast",
                    "ShallowCoast",
                    "Ocean",
                    "River",
                ],
            )),
        }
    }
}
