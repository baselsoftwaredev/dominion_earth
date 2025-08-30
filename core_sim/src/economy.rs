use crate::{CivId, Position, Economy, GameResource, GlobalEconomy, TradeRoute, Building, BuildingType, City};
use crate::constants::economy;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Economic calculations and updates
pub struct EconomicSystem;

impl EconomicSystem {
    /// Update the economy for all civilizations
    pub fn update_economy(
        civilizations: &mut bevy_ecs::system::Query<(bevy_ecs::entity::Entity, &mut Economy, &CivId)>,
        cities: &bevy_ecs::system::Query<(&City, &Position)>,
        buildings: &bevy_ecs::system::Query<(&Building, &Position)>,
        trade_routes: &bevy_ecs::system::Query<&TradeRoute>,
        global_economy: &mut bevy_ecs::system::ResMut<GlobalEconomy>,
    ) {
        // Update resource production for each civilization
        for (entity, mut economy, civ_id) in civilizations.iter_mut() {
            Self::calculate_production(&mut economy, civ_id, cities, buildings);
            Self::calculate_maintenance_costs(&mut economy, buildings, civ_id);
            Self::update_treasury(&mut economy);
        }

        // Update global trade and resource prices
        Self::update_global_trade(&mut global_economy, trade_routes);
        Self::update_resource_prices(&mut global_economy);
    }

    /// Calculate resource production for a civilization
    fn calculate_production(
        economy: &mut Economy,
        civ_id: &CivId,
        cities: &bevy_ecs::system::Query<(&City, &Position)>,
        buildings: &bevy_ecs::system::Query<(&Building, &Position)>,
    ) {
        let mut base_production = HashMap::new();
        base_production.insert(GameResource::Gold, economy::BASE_GOLD_PRODUCTION);
        base_production.insert(GameResource::Wheat, economy::BASE_WHEAT_PRODUCTION);

        // Add production from cities
        for (city, _position) in cities.iter() {
            if city.owner == *civ_id {
                for (resource, amount) in &city.resource_yields {
                    *base_production.entry(resource.clone()).or_insert(economy::DEFAULT_RESOURCE_AMOUNT) += amount;
                }
            }
        }

        // Add production from buildings
        for (building, _position) in buildings.iter() {
            if building.owner == *civ_id {
                let building_production = Self::get_building_production(&building.building_type);
                for (resource, amount) in building_production {
                    *base_production.entry(resource).or_insert(economy::DEFAULT_RESOURCE_AMOUNT) += amount;
                }
            }
        }

        economy.resource_production = base_production;
        
        // Calculate total production value
        economy.total_production = economy.resource_production.values().sum();
    }

    /// Calculate maintenance costs for buildings and units
    fn calculate_maintenance_costs(
        economy: &mut Economy,
        buildings: &bevy_ecs::system::Query<(&Building, &Position)>,
        civ_id: &CivId,
    ) {
        let mut total_maintenance = 0.0;

        // Building maintenance
        for (building, _position) in buildings.iter() {
            if building.owner == *civ_id {
                total_maintenance += Self::get_building_maintenance(&building.building_type);
            }
        }

        economy.maintenance_costs = total_maintenance;
    }

    /// Update civilization treasury
    fn update_treasury(economy: &mut Economy) {
        let income = economy.resource_production.get(&GameResource::Gold).unwrap_or(&0.0);
        let net_income = income - economy.maintenance_costs;
        economy.treasury = (economy.treasury + net_income).max(0.0);
    }

    /// Update global trade and resource circulation
    fn update_global_trade(
        global_economy: &mut GlobalEconomy,
        trade_routes: &bevy_ecs::system::Query<&TradeRoute>,
    ) {
        let mut total_trade_volume = 0.0;

        for trade_route in trade_routes.iter() {
            if trade_route.active {
                total_trade_volume += trade_route.gold_per_turn;
            }
        }

        global_economy.total_trade_volume = total_trade_volume;
    }

    /// Update resource prices based on supply and demand
    fn update_resource_prices(global_economy: &mut GlobalEconomy) {
        for (resource, price) in global_economy.resource_prices.iter_mut() {
            let base_price = Self::get_base_resource_price(resource);
            let demand_factor = global_economy.resource_demand.get(resource).unwrap_or(&1.0);
            let supply_factor = global_economy.resource_supply.get(resource).unwrap_or(&1.0);
            
            let new_price = base_price * (demand_factor / supply_factor.max(0.1));
            *price = (*price * 0.8 + new_price * 0.2).max(base_price * 0.5).min(base_price * 3.0);
        }
    }

    fn get_base_resource_price(resource: &GameResource) -> f32 {
        match resource {
            GameResource::Iron => 12.0,
            GameResource::Gold => 50.0,
            GameResource::Horses => 20.0,
            GameResource::Wheat => 5.0,
            GameResource::Fish => 3.0,
            GameResource::Stone => 8.0,
            GameResource::Wood => 6.0,
            GameResource::Spices => 25.0,
        }
    }

    /// Get building production yields
    fn get_building_production(building_type: &BuildingType) -> HashMap<GameResource, f32> {
        let mut production = HashMap::new();
        
        match building_type {
            BuildingType::Granary => {
                production.insert(GameResource::Wheat, 2.0);
            }
            BuildingType::Market => {
                production.insert(GameResource::Gold, 3.0);
            }
            BuildingType::Workshop => {
                production.insert(GameResource::Wood, 1.5);
                production.insert(GameResource::Stone, 1.0);
            }
            BuildingType::Library => {
                production.insert(GameResource::Gold, 1.0);
            }
            BuildingType::Barracks => {}
            BuildingType::Temple => {
                production.insert(GameResource::Gold, 1.5);
            }
            BuildingType::Walls => {}
        }
        
        production
    }

    /// Get building maintenance costs
    fn get_building_maintenance(building_type: &BuildingType) -> f32 {
        match building_type {
            BuildingType::Granary => 1.0,
            BuildingType::Market => 2.0,
            BuildingType::Workshop => 2.5,
            BuildingType::Library => 2.0,
            BuildingType::Barracks => 1.5,
            BuildingType::Temple => 1.0,
            BuildingType::Walls => 0.5,
        }
    }

    /// Check if civilization can afford something
    pub fn can_afford(economy: &Economy, cost: f32) -> bool {
        economy.treasury >= cost
    }

    /// Spend gold from treasury
    pub fn spend_gold(economy: &mut Economy, amount: f32) -> bool {
        if Self::can_afford(economy, amount) {
            economy.treasury -= amount;
            true
        } else {
            false
        }
    }

    /// Calculate strategic resource availability
    pub fn get_strategic_resources(economy: &Economy) -> Vec<GameResource> {
        let strategic_resources = [
            GameResource::Iron,
            GameResource::Gold,
            GameResource::Horses,
            GameResource::Wheat,
            GameResource::Fish,
            GameResource::Stone,
            GameResource::Wood,
            GameResource::Spices,
        ];

        strategic_resources
            .iter()
            .filter(|resource| {
                economy.resource_production.get(resource).unwrap_or(&0.0) > &0.0
            })
            .cloned()
            .collect()
    }

    /// Calculate total civilization wealth
    pub fn calculate_total_wealth(
        economy: &Economy,
        global_economy: &GlobalEconomy,
    ) -> f32 {
        let mut total_wealth = economy.treasury;

        // Add value of resource stockpiles
        for (resource, amount) in &economy.resource_production {
            if let Some(price) = global_economy.resource_prices.get(resource) {
                total_wealth += amount * price * 10.0; // Assume 10 turns of stockpile
            }
        }

        total_wealth
    }

    /// Calculate economic power rating
    pub fn calculate_economic_power(
        economy: &Economy,
        global_economy: &GlobalEconomy,
    ) -> f32 {
        let wealth = Self::calculate_total_wealth(economy, global_economy);
        let production = economy.total_production;
        let efficiency = if economy.maintenance_costs > 0.0 {
            production / economy.maintenance_costs
        } else {
            production
        };

        (wealth * 0.3 + production * 0.5 + efficiency * 0.2) / 100.0
    }

    /// Get trade route profitability
    pub fn calculate_trade_route_value(
        from_civ: &CivId,
        to_civ: &CivId,
        distance: f32,
        global_economy: &GlobalEconomy,
    ) -> f32 {
        let base_value = 5.0;
        let distance_penalty = (distance / 10.0).min(2.0);
        let trade_multiplier = global_economy.total_trade_volume / 1000.0 + 1.0;
        
        (base_value * trade_multiplier - distance_penalty).max(1.0)
    }

    /// Create economic report for a civilization
    pub fn generate_economic_report(economy: &Economy) -> EconomicReport {
        EconomicReport {
            treasury: economy.treasury,
            total_production: economy.total_production,
            maintenance_costs: economy.maintenance_costs,
            net_income: economy.resource_production.get(&GameResource::Gold).unwrap_or(&0.0) - economy.maintenance_costs,
            resource_production: economy.resource_production.clone(),
        }
    }
}

/// Economic report for UI display
#[derive(Debug, Clone)]
pub struct EconomicReport {
    pub treasury: f32,
    pub total_production: f32,
    pub maintenance_costs: f32,
    pub net_income: f32,
    pub resource_production: HashMap<GameResource, f32>,
}

/// Market prices for UI
#[derive(Debug, Clone)]
pub struct MarketPrices {
    pub resource_prices: HashMap<GameResource, f32>,
    pub trade_goods: Vec<TradeGood>,
}

/// Tradeable goods
#[derive(Debug, Clone)]
pub struct TradeGood {
    pub name: String,
    pub resource_type: GameResource,
    pub base_price: f32,
    pub current_price: f32,
    pub availability: f32,
}
