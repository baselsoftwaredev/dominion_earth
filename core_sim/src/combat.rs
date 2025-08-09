use crate::{CivId, Position, MilitaryUnit, UnitType};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rand::Rng;

/// Combat resolution system
pub struct CombatSystem;

impl CombatSystem {
    /// Resolve combat between units at the same position
    pub fn resolve_combat(
        attacking_units: &mut Vec<MilitaryUnit>,
        defending_units: &mut Vec<MilitaryUnit>,
        terrain_defense_bonus: f32,
        rng: &mut impl Rng,
    ) -> CombatResult {
        let initial_attacker_strength = attacking_units.iter().map(|u| u.strength).sum::<f32>();
        let initial_defender_strength = defending_units.iter().map(|u| u.strength).sum::<f32>();

        let mut attacker_strength = initial_attacker_strength;
        let mut defender_strength = initial_defender_strength * (1.0 + terrain_defense_bonus);

        let mut rounds = 0;
        let max_rounds = 10;
        let mut casualties = CombatCasualties::default();

        while attacker_strength > 0.0 && defender_strength > 0.0 && rounds < max_rounds {
            rounds += 1;

            // Calculate damage dealt by each side
            let attacker_damage = Self::calculate_damage(attacker_strength, rng);
            let defender_damage = Self::calculate_damage(defender_strength, rng);

            // Apply damage
            defender_strength = (defender_strength - attacker_damage).max(0.0);
            attacker_strength = (attacker_strength - defender_damage).max(0.0);
        }

        // Determine winner and apply casualties
        let (winner, attacker_survival_rate, defender_survival_rate) = if attacker_strength > defender_strength {
            (CombatWinner::Attacker, attacker_strength / initial_attacker_strength, 0.0)
        } else if defender_strength > attacker_strength {
            (CombatWinner::Defender, 0.0, defender_strength / (initial_defender_strength * (1.0 + terrain_defense_bonus)))
        } else {
            (CombatWinner::Draw, 0.1, 0.1) // Both sides heavily damaged
        };

        // Apply casualties to units
        Self::apply_casualties(attacking_units, 1.0 - attacker_survival_rate, &mut casualties.attacker_losses);
        Self::apply_casualties(defending_units, 1.0 - defender_survival_rate, &mut casualties.defender_losses);

        // Grant experience to survivors
        Self::grant_combat_experience(attacking_units);
        Self::grant_combat_experience(defending_units);

        CombatResult {
            winner,
            rounds,
            casualties,
            attacker_final_strength: attacker_strength,
            defender_final_strength: defender_strength,
        }
    }

    fn calculate_damage(strength: f32, rng: &mut impl Rng) -> f32 {
        let base_damage = strength * 0.3; // 30% of strength as base damage
        let random_factor = rng.gen_range(0.7..1.3); // ±30% random variation
        base_damage * random_factor
    }

    fn apply_casualties(units: &mut Vec<MilitaryUnit>, casualty_rate: f32, losses: &mut HashMap<UnitType, u32>) {
        units.retain_mut(|unit| {
            if casualty_rate >= 1.0 {
                // Unit destroyed
                *losses.entry(unit.unit_type.clone()).or_insert(0) += 1;
                false
            } else {
                // Unit damaged
                unit.strength *= 1.0 - casualty_rate;
                if unit.strength <= 0.1 {
                    // Unit effectively destroyed if strength too low
                    *losses.entry(unit.unit_type.clone()).or_insert(0) += 1;
                    false
                } else {
                    true
                }
            }
        });
    }

    fn grant_combat_experience(units: &mut [MilitaryUnit]) {
        for unit in units {
            unit.experience += 0.1; // Gain experience from combat
            if unit.experience >= 1.0 {
                // Promote unit
                unit.strength *= 1.2;
                unit.experience = 0.0;
            }
        }
    }

    /// Calculate combat strength for a group of units
    pub fn calculate_total_strength(units: &[MilitaryUnit]) -> f32 {
        units.iter().map(|unit| {
            let base_strength = unit.strength;
            let experience_bonus = 1.0 + unit.experience * 0.5; // Up to 50% bonus from experience
            base_strength * experience_bonus
        }).sum()
    }

    /// Calculate defensive bonuses from terrain and fortifications
    pub fn calculate_defense_bonus(terrain_bonus: f32, fortification_level: u32) -> f32 {
        terrain_bonus + (fortification_level as f32 * 0.1)
    }

    /// Determine if a unit can attack another unit
    pub fn can_attack(attacker: &MilitaryUnit, defender_pos: Position, max_range: u32) -> bool {
        let distance = attacker.position.distance_to(&defender_pos) as u32;
        
        match attacker.unit_type {
            UnitType::Archer => distance <= max_range.max(2),
            UnitType::Siege => distance <= max_range.max(1),
            _ => distance <= max_range.max(1),
        }
    }

    /// Calculate unit effectiveness against different unit types
    pub fn calculate_effectiveness(attacker_type: &UnitType, defender_type: &UnitType) -> f32 {
        match (attacker_type, defender_type) {
            // Cavalry strong against archers and siege
            (UnitType::Cavalry, UnitType::Archer) => 1.5,
            (UnitType::Cavalry, UnitType::Siege) => 1.3,
            
            // Infantry strong against cavalry
            (UnitType::Infantry, UnitType::Cavalry) => 1.3,
            
            // Archers strong against infantry
            (UnitType::Archer, UnitType::Infantry) => 1.2,
            
            // Siege strong against fortified positions
            (UnitType::Siege, _) => 1.4, // Bonus against all when attacking cities
            
            // Naval units
            (UnitType::Naval, UnitType::Naval) => 1.0,
            
            // Default effectiveness
            _ => 1.0,
        }
    }

    /// Simulate siege warfare
    pub fn resolve_siege(
        attacking_units: &[MilitaryUnit],
        city_defense: f32,
        fortification_level: u32,
        turns_besieged: u32,
        rng: &mut impl Rng,
    ) -> SiegeResult {
        let attacker_strength = Self::calculate_total_strength(attacking_units);
        let siege_units = attacking_units.iter().filter(|u| matches!(u.unit_type, UnitType::Siege)).count();
        
        let siege_bonus = siege_units as f32 * 0.5; // Siege units are effective against cities
        let effective_attack = attacker_strength + siege_bonus;
        
        let fortification_bonus = fortification_level as f32 * 0.3;
        let starvation_penalty = if turns_besieged > 10 { (turns_besieged - 10) as f32 * 0.1 } else { 0.0 };
        let effective_defense = (city_defense + fortification_bonus - starvation_penalty).max(1.0);
        
        let attack_success_chance = effective_attack / (effective_attack + effective_defense);
        let random_factor = rng.gen::<f32>();
        
        if random_factor < attack_success_chance {
            SiegeResult::Captured {
                turns_to_capture: 1,
                attacker_casualties: 0.2, // 20% casualties for successful assault
            }
        } else if turns_besieged > 20 && random_factor < 0.3 {
            SiegeResult::Surrendered {
                turns_besieged,
            }
        } else {
            SiegeResult::Ongoing {
                attacker_casualties: 0.1, // 10% casualties per failed assault
                defender_casualties: 0.05, // 5% casualties from bombardment
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatResult {
    pub winner: CombatWinner,
    pub rounds: u32,
    pub casualties: CombatCasualties,
    pub attacker_final_strength: f32,
    pub defender_final_strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatWinner {
    Attacker,
    Defender,
    Draw,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CombatCasualties {
    pub attacker_losses: HashMap<UnitType, u32>,
    pub defender_losses: HashMap<UnitType, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SiegeResult {
    Captured {
        turns_to_capture: u32,
        attacker_casualties: f32,
    },
    Surrendered {
        turns_besieged: u32,
    },
    Ongoing {
        attacker_casualties: f32,
        defender_casualties: f32,
    },
}
