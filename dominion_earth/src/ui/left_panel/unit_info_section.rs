use bevy::prelude::*;
use core_sim::MilitaryUnit;

use super::constants::*;

// Component markers for unit info UI elements
#[derive(Component)]
pub struct UnitInfoPanel;

#[derive(Component)]
pub struct UnitNameText;

#[derive(Component)]
pub struct UnitAttackText;

#[derive(Component)]
pub struct UnitDefenseText;

#[derive(Component)]
pub struct UnitHealthText;

#[derive(Component)]
pub struct UnitRangeText;

#[derive(Component)]
pub struct UnitMovementText;

#[derive(Component)]
pub struct UnitFatigueText;

#[derive(Component)]
pub struct UnitTypeText;

#[derive(Component)]
pub struct UnitStrengthText;

#[derive(Component)]
pub struct UnitSupplyText;

#[derive(Component)]
pub struct UnitDecayText;

#[derive(Component)]
pub struct UnitExperienceText;

#[derive(Component)]
pub struct UnitEffectiveAttackText;

#[derive(Component)]
pub struct UnitEffectiveDefenseText;

/// System to update unit info panel with selected unit data
pub fn update_unit_info(
    selected_unit: Res<core_sim::SelectedUnit>,
    units_query: Query<&MilitaryUnit>,
    mut panel_query: Query<&mut Node, With<UnitInfoPanel>>,
    mut unit_name_text: Query<
        &mut Text,
        (
            With<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut attack_text: Query<
        &mut Text,
        (
            With<UnitAttackText>,
            Without<UnitNameText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut defense_text: Query<
        &mut Text,
        (
            With<UnitDefenseText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut health_text: Query<
        &mut Text,
        (
            With<UnitHealthText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut range_text: Query<
        &mut Text,
        (
            With<UnitRangeText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut movement_text: Query<
        &mut Text,
        (
            With<UnitMovementText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut fatigue_text: Query<
        &mut Text,
        (
            With<UnitFatigueText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut supply_text: Query<
        &mut Text,
        (
            With<UnitSupplyText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitDecayText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut decay_text: Query<
        &mut Text,
        (
            With<UnitDecayText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitExperienceText>,
        ),
    >,
    mut experience_text: Query<
        &mut Text,
        (
            With<UnitExperienceText>,
            Without<UnitNameText>,
            Without<UnitAttackText>,
            Without<UnitDefenseText>,
            Without<UnitHealthText>,
            Without<UnitRangeText>,
            Without<UnitMovementText>,
            Without<UnitFatigueText>,
            Without<UnitSupplyText>,
            Without<UnitDecayText>,
        ),
    >,
) {
    if selected_unit.is_changed() {
        if let Some(mut node) = panel_query.iter_mut().next() {
            node.display = if selected_unit.unit_entity.is_some() {
                Display::Flex
            } else {
                Display::None
            };
        }

        if let Some(unit_entity) = selected_unit.unit_entity {
            if let Ok(unit) = units_query.get(unit_entity) {
                if let Some(mut text) = unit_name_text.iter_mut().next() {
                    **text = format!("Unit #{} - {}", unit.id, unit.unit_type.name());
                }

                if let Some(mut text) = attack_text.iter_mut().next() {
                    **text = format!(
                        "Attack: {:.1} (Effective: {:.1})",
                        unit.attack,
                        unit.effective_attack()
                    );
                }

                if let Some(mut text) = defense_text.iter_mut().next() {
                    **text = format!(
                        "Defense: {:.1} (Effective: {:.1})",
                        unit.defense,
                        unit.effective_defense()
                    );
                }

                if let Some(mut text) = health_text.iter_mut().next() {
                    **text = format!("Health: {:.0} / {:.0}", unit.health, unit.max_health);
                }

                if let Some(mut text) = range_text.iter_mut().next() {
                    **text = format!("Range: {}", unit.range);
                }

                if let Some(mut text) = movement_text.iter_mut().next() {
                    **text = format!(
                        "Remaining: {} / {}",
                        unit.movement_remaining, unit.movement_range
                    );
                }

                if let Some(mut text) = fatigue_text.iter_mut().next() {
                    **text = format!("Fatigue: {:.0}%", unit.fatigue * 100.0);
                }

                if let Some(mut text) = supply_text.iter_mut().next() {
                    **text = format!("Supply: {:.0}%", unit.supply * 100.0);
                }

                if let Some(mut text) = decay_text.iter_mut().next() {
                    **text = format!("Decay: {:.0}%", unit.decay * 100.0);
                }

                if let Some(mut text) = experience_text.iter_mut().next() {
                    **text = format!("Experience: {:.0}%", unit.experience * 100.0);
                }
            }
        }
    }
}
