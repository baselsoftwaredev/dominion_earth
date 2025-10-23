use crate::constants::combat;
use crate::{CivId, MilitaryUnit, Position, UnitType};
use rand::Rng;
use std::collections::HashMap;

pub struct CombatSystem;

impl CombatSystem {
    /// Resolve combat between units at the same position
    pub fn resolve_combat(
        attacking_units: &mut Vec<MilitaryUnit>,
        defending_units: &mut Vec<MilitaryUnit>,
        terrain_defense_bonus: f32,
        rng: &mut impl Rng,
    ) -> CombatResult {
        // Use effective attack and defense stats
        let initial_attacker_strength: f32 =
            attacking_units.iter().map(|u| u.effective_attack()).sum();
        let initial_defender_strength: f32 =
            defending_units.iter().map(|u| u.effective_defense()).sum();

        let mut attacker_strength = initial_attacker_strength;
        let mut defender_strength = initial_defender_strength * (1.0 + terrain_defense_bonus);

        let mut rounds = 0;
        let max_rounds = combat::MAX_COMBAT_ROUNDS;
        let mut casualties = CombatCasualties::default();

        while attacker_strength > 0.0 && defender_strength > 0.0 && rounds < max_rounds {
            rounds += 1;

            let attacker_damage = Self::calculate_damage(attacker_strength, rng);
            let defender_damage = Self::calculate_damage(defender_strength, rng);

            defender_strength = (defender_strength - attacker_damage).max(0.0);
            attacker_strength = (attacker_strength - defender_damage).max(0.0);
        }

        let (winner, attacker_survival_rate, defender_survival_rate) =
            if attacker_strength > defender_strength {
                (
                    CombatWinner::Attacker,
                    attacker_strength / initial_attacker_strength,
                    0.0,
                )
            } else if defender_strength > attacker_strength {
                (
                    CombatWinner::Defender,
                    0.0,
                    defender_survival_rate
                        / (initial_defender_strength * (1.0 + terrain_defense_bonus)),
                )
            } else {
                (
                    CombatWinner::Draw,
                    combat::LOSER_EXPERIENCE_GAIN,
                    combat::LOSER_EXPERIENCE_GAIN,
                )
            };

        Self::apply_casualties(
            attacking_units,
            1.0 - attacker_survival_rate,
            &mut casualties.attacker_losses,
        );
        Self::apply_casualties(
            defending_units,
            1.0 - defender_survival_rate,
            &mut casualties.defender_losses,
        );

        Self::grant_combat_experience_and_effects(
            attacking_units,
            winner == CombatWinner::Attacker,
        );
        Self::grant_combat_experience_and_effects(
            defending_units,
            winner == CombatWinner::Defender,
        );

        CombatResult {
            winner,
            rounds,
            casualties,
            attacker_final_strength: attacker_strength,
            defender_final_strength: defender_strength,
        }
    }

    fn calculate_damage(strength: f32, rng: &mut impl Rng) -> f32 {
        let base_damage = strength * combat::BASE_DAMAGE_MULTIPLIER;
        let random_factor =
            rng.gen_range(combat::RANDOM_DAMAGE_VARIANCE_MIN..combat::RANDOM_DAMAGE_VARIANCE_MAX);
        base_damage * random_factor
    }

    fn apply_casualties(
        units: &mut Vec<MilitaryUnit>,
        casualty_rate: f32,
        losses: &mut HashMap<UnitType, u32>,
    ) {
        units.retain_mut(|unit| {
            if casualty_rate >= combat::COMPLETE_CASUALTY_THRESHOLD {
                *losses.entry(unit.unit_type.clone()).or_insert(0) += 1;
                false
            } else {
                let damage = unit.health * casualty_rate;
                unit.health = (unit.health - damage).max(0.0);

                unit.add_fatigue(casualty_rate * combat::COMBAT_FATIGUE_FROM_CASUALTIES);
                unit.add_decay(casualty_rate * combat::COMBAT_DECAY_INCREASE);

                if casualty_rate > combat::HEAVY_CASUALTY_THRESHOLD {
                    unit.reduce_morale(casualty_rate * combat::HEAVY_CASUALTY_MORALE_PENALTY);
                }

                if unit.health <= combat::MINIMUM_UNIT_HEALTH_THRESHOLD {
                    *losses.entry(unit.unit_type.clone()).or_insert(0) += 1;
                    false
                } else {
                    true
                }
            }
        });
    }

    fn grant_combat_experience_and_effects(units: &mut [MilitaryUnit], won_combat: bool) {
        for unit in units {
            let exp_gain = if won_combat {
                combat::WINNER_EXPERIENCE_GAIN
            } else {
                combat::LOSER_EXPERIENCE_GAIN
            };
            unit.gain_experience(exp_gain);

            unit.add_fatigue(combat::COMBAT_FATIGUE_INCREASE);

            if won_combat {
                unit.morale =
                    (unit.morale + combat::VICTOR_MORALE_INCREASE).min(combat::MAXIMUM_MORALE);
            } else {
                unit.reduce_morale(combat::DEFEATED_MORALE_DECREASE);
            }
        }
    }

    /// Calculate combat strength for a group of units (using effective stats)
    pub fn calculate_total_strength(units: &[MilitaryUnit]) -> f32 {
        units
            .iter()
            .map(|unit| {
                let base_attack = unit.effective_attack();
                let base_defense = unit.effective_defense();
                (base_attack + base_defense) / 2.0
            })
            .sum()
    }

    pub fn calculate_defense_bonus(terrain_bonus: f32, fortification_level: u32) -> f32 {
        terrain_bonus + (fortification_level as f32 * combat::FORTIFICATION_DEFENSE_BONUS)
    }

    pub fn can_attack(attacker: &MilitaryUnit, defender_pos: Position, max_range: u32) -> bool {
        let distance = attacker.position.distance_to(&defender_pos) as u32;
        let effective_range = max_range.max(attacker.range);
        distance <= effective_range
    }

    pub fn calculate_effectiveness(attacker_type: &UnitType, defender_type: &UnitType) -> f32 {
        match (attacker_type, defender_type) {
            (UnitType::Cavalry, UnitType::Archer) => combat::CAVALRY_VS_ARCHER_BONUS,
            (UnitType::Cavalry, UnitType::Siege) => combat::CAVALRY_VS_SIEGE_BONUS,

            (UnitType::Infantry, UnitType::Cavalry) => combat::INFANTRY_VS_CAVALRY_BONUS,

            (UnitType::Archer, UnitType::Infantry) => combat::ARCHER_VS_INFANTRY_BONUS,

            (UnitType::Siege, _) => combat::SIEGE_VS_ALL_BONUS,

            (UnitType::Naval, UnitType::Naval) => combat::NAVAL_VS_NAVAL_MULTIPLIER,

            _ => combat::DEFAULT_TYPE_EFFECTIVENESS,
        }
    }

    pub fn resolve_siege(
        attacking_units: &[MilitaryUnit],
        city_defense: f32,
        fortification_level: u32,
        turns_besieged: u32,
        rng: &mut impl Rng,
    ) -> SiegeResult {
        let attacker_strength = Self::calculate_total_strength(attacking_units);
        let siege_units = attacking_units
            .iter()
            .filter(|u| matches!(u.unit_type, UnitType::Siege))
            .count();

        let siege_bonus = siege_units as f32 * combat::SIEGE_UNIT_BONUS;
        let effective_attack = attacker_strength + siege_bonus;

        let fortification_bonus = fortification_level as f32 * combat::FORTIFICATION_DEFENSE_BONUS;
        let starvation_penalty = if turns_besieged > combat::STARVATION_PENALTY_TURNS_THRESHOLD {
            (turns_besieged - combat::STARVATION_PENALTY_TURNS_THRESHOLD) as f32
                * combat::STARVATION_PENALTY_PER_TURN
        } else {
            0.0
        };
        let effective_defense = (city_defense + fortification_bonus - starvation_penalty)
            .max(combat::MINIMUM_CITY_DEFENSE);

        let attack_success_chance = effective_attack / (effective_attack + effective_defense);
        let random_factor = rng.gen::<f32>();

        if random_factor < attack_success_chance {
            SiegeResult::Captured {
                turns_to_capture: 1,
                attacker_casualties: combat::SIEGE_SUCCESSFUL_ASSAULT_CASUALTIES,
            }
        } else if turns_besieged > combat::PROLONGED_SIEGE_TURNS_THRESHOLD
            && random_factor < combat::PROLONGED_SIEGE_SURRENDER_CHANCE
        {
            SiegeResult::Surrendered { turns_besieged }
        } else {
            SiegeResult::Ongoing {
                attacker_casualties: combat::SIEGE_FAILED_ASSAULT_ATTACKER_CASUALTIES,
                defender_casualties: combat::SIEGE_BOMBARDMENT_DEFENDER_CASUALTIES,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CombatResult {
    pub winner: CombatWinner,
    pub rounds: u32,
    pub casualties: CombatCasualties,
    pub attacker_final_strength: f32,
    pub defender_final_strength: f32,
}

#[derive(Debug, Clone)]
pub enum CombatWinner {
    Attacker,
    Defender,
    Draw,
}

#[derive(Debug, Clone, Default)]
pub struct CombatCasualties {
    pub attacker_losses: HashMap<UnitType, u32>,
    pub defender_losses: HashMap<UnitType, u32>,
}

#[derive(Debug, Clone)]
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
