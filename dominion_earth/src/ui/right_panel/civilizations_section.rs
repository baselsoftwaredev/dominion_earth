use bevy::prelude::*;
use core_sim::{Civilization, PlayerControlled};

#[derive(Component)]
pub struct CivilizationsListPanel;

#[derive(Component)]
pub struct CivilizationsListText;

pub fn update_civilizations_list(
    civs: Query<&Civilization>,
    player_civs: Query<&Civilization, With<PlayerControlled>>,
    mut civs_text: Query<&mut Text, With<CivilizationsListText>>,
) {
    let all_civs: Vec<&Civilization> = civs.iter().collect();

    if all_civs.is_empty() {
        if let Some(mut text) = civs_text.iter_mut().next() {
            **text = "No civilizations yet".to_string();
        }
        return;
    }

    let civ_details: Vec<String> = all_civs
        .iter()
        .map(|civ| {
            let civ_type = if player_civs.iter().any(|pc| pc.id == civ.id) {
                "Player"
            } else {
                "AI"
            };
            format!(
                "{} - {} (Gold: {})",
                civ.name, civ_type, civ.economy.gold as i32
            )
        })
        .collect();

    if let Some(mut text) = civs_text.iter_mut().next() {
        **text = civ_details.join(", ");
    }
}
