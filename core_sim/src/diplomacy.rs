use crate::{CivId, Treaty, DiplomaticRelation, DiplomaticState, CivPersonality};
use crate::resources::{Negotiation, DiplomaticProposal, DiplomaticEventType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

/// Diplomatic system for AI decision making
pub struct DiplomaticSystem;

impl DiplomaticSystem {
    /// Update diplomatic relations for all civilizations
    pub fn update_diplomacy(
        diplomatic_state: &mut DiplomaticState,
        civs: &HashMap<CivId, CivPersonality>,
        military_strengths: &HashMap<CivId, f32>,
        economic_powers: &HashMap<CivId, f32>,
        turn: u32,
        rng: &mut impl Rng,
    ) {
        // Process ongoing negotiations
        Self::process_negotiations(diplomatic_state, civs, rng);

        // Update relation values based on recent events
        Self::update_relation_values(diplomatic_state, civs, turn);

        // Decay treaty durations
        Self::update_treaty_durations(diplomatic_state);

        // Generate new diplomatic events
        Self::generate_diplomatic_events(diplomatic_state, civs, military_strengths, economic_powers, rng);
    }

    fn process_negotiations(
        diplomatic_state: &mut DiplomaticState,
        civs: &HashMap<CivId, CivPersonality>,
        rng: &mut impl Rng,
    ) {
        diplomatic_state.ongoing_negotiations.retain_mut(|negotiation| {
            negotiation.turns_remaining = negotiation.turns_remaining.saturating_sub(1);
            
            if negotiation.turns_remaining == 0 {
                // Evaluate if the negotiation succeeds
                let success_chance = Self::calculate_negotiation_success_chance(
                    negotiation,
                    civs,
                    &diplomatic_state.relations,
                );
                
                if rng.gen::<f32>() < success_chance {
                    Self::accept_proposal(negotiation, diplomatic_state);
                }
                false // Remove completed negotiation
            } else {
                true // Keep ongoing negotiation
            }
        });
    }

    fn calculate_negotiation_success_chance(
        negotiation: &Negotiation,
        civs: &HashMap<CivId, CivPersonality>,
        relations: &HashMap<(CivId, CivId), DiplomaticRelation>,
    ) -> f32 {
        let initiator_personality = civs.get(&negotiation.initiator);
        let target_personality = civs.get(&negotiation.target);
        
        if let (Some(initiator), Some(target)) = (initiator_personality, target_personality) {
            let relation_key = (negotiation.initiator, negotiation.target);
            let current_relation = relations.get(&relation_key)
                .map(|r| r.relation_value)
                .unwrap_or(0.0);
            
            let base_chance = match negotiation.proposal {
                DiplomaticProposal::TradePact => 0.6,
                DiplomaticProposal::NonAggressionPact => 0.4,
                DiplomaticProposal::Alliance => 0.2,
                DiplomaticProposal::PeaceTreaty => 0.8,
                DiplomaticProposal::TechnologyExchange(_) => 0.3,
                DiplomaticProposal::ResourceTrade(_, _) => 0.7,
            };
            
            let relation_modifier = current_relation / 100.0; // -1.0 to 1.0
            let personality_modifier = match negotiation.proposal {
                DiplomaticProposal::TradePact => target.industry_focus * 0.5,
                DiplomaticProposal::NonAggressionPact => target.honor_treaties * 0.3,
                DiplomaticProposal::Alliance => (target.honor_treaties + target.interventionism) * 0.25,
                DiplomaticProposal::PeaceTreaty => target.honor_treaties * 0.4,
                DiplomaticProposal::TechnologyExchange(_) => target.tech_focus * 0.4,
                DiplomaticProposal::ResourceTrade(_, _) => target.industry_focus * 0.3,
            };
            
            (base_chance + relation_modifier * 0.5 + personality_modifier * 0.3).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    fn accept_proposal(negotiation: &Negotiation, diplomatic_state: &mut DiplomaticState) {
        let relation_key = (negotiation.initiator, negotiation.target);
        let relation = diplomatic_state.relations.get_mut(&relation_key);
        
        if let Some(relation) = relation {
            match &negotiation.proposal {
                DiplomaticProposal::TradePact => {
                    relation.trade_agreement = true;
                    relation.relation_value += 10.0;
                }
                DiplomaticProposal::NonAggressionPact => {
                    relation.treaties.push(Treaty::NonAggression { turns_remaining: 50 });
                    relation.relation_value += 15.0;
                }
                DiplomaticProposal::Alliance => {
                    relation.treaties.push(Treaty::Alliance { turns_remaining: 100 });
                    relation.relation_value += 25.0;
                }
                DiplomaticProposal::PeaceTreaty => {
                    // Remove war status
                    relation.treaties.retain(|treaty| !matches!(treaty, Treaty::War { .. }));
                    relation.relation_value += 20.0;
                }
                _ => {}
            }
        }
        
        // Add diplomatic event
        diplomatic_state.diplomatic_events.push(crate::DiplomaticEvent {
            event_type: match negotiation.proposal {
                DiplomaticProposal::Alliance => DiplomaticEventType::AllianceFormed,
                DiplomaticProposal::PeaceTreaty => DiplomaticEventType::PeaceSigned,
                DiplomaticProposal::TradePact => DiplomaticEventType::TradeAgreementSigned,
                _ => DiplomaticEventType::TradeAgreementSigned, // Generic for other agreements
            },
            involved_civs: vec![negotiation.initiator, negotiation.target],
            turn: 0, // Will be set by caller
        });
    }

    fn update_relation_values(
        diplomatic_state: &mut DiplomaticState,
        civs: &HashMap<CivId, CivPersonality>,
        turn: u32,
    ) {
        for relation in diplomatic_state.relations.values_mut() {
            // Natural decay of extreme relations towards neutral
            if relation.relation_value > 0.0 {
                relation.relation_value = (relation.relation_value - 0.5).max(0.0);
            } else if relation.relation_value < 0.0 {
                relation.relation_value = (relation.relation_value + 0.5).min(0.0);
            }
            
            // Personality-based relationship drift
            if let (Some(civ_a_personality), Some(civ_b_personality)) = 
                (civs.get(&relation.civ_a), civs.get(&relation.civ_b)) {
                
                let compatibility = Self::calculate_personality_compatibility(civ_a_personality, civ_b_personality);
                relation.relation_value += compatibility * 0.1;
            }
            
            // Clamp relation values
            relation.relation_value = relation.relation_value.clamp(-100.0, 100.0);
        }
    }

    fn calculate_personality_compatibility(personality_a: &CivPersonality, personality_b: &CivPersonality) -> f32 {
        let mut compatibility = 0.0;
        
        // Similar tech focus improves relations
        compatibility += 1.0 - (personality_a.tech_focus - personality_b.tech_focus).abs();
        
        // Different interventionism levels can cause friction
        compatibility -= (personality_a.interventionism - personality_b.interventionism).abs() * 0.5;
        
        // High honor_treaties on both sides improves relations
        compatibility += (personality_a.honor_treaties + personality_b.honor_treaties) * 0.25;
        
        // Opposing militarism levels cause tension
        compatibility -= (personality_a.militarism - personality_b.militarism).abs() * 0.3;
        
        compatibility.clamp(-1.0, 1.0)
    }

    fn update_treaty_durations(diplomatic_state: &mut DiplomaticState) {
        for relation in diplomatic_state.relations.values_mut() {
            relation.treaties.retain_mut(|treaty| {
                match treaty {
                    Treaty::NonAggression { turns_remaining } => {
                        *turns_remaining = turns_remaining.saturating_sub(1);
                        *turns_remaining > 0
                    }
                    Treaty::Alliance { turns_remaining } => {
                        *turns_remaining = turns_remaining.saturating_sub(1);
                        *turns_remaining > 0
                    }
                    Treaty::TradePact { turns_remaining } => {
                        *turns_remaining = turns_remaining.saturating_sub(1);
                        *turns_remaining > 0
                    }
                    Treaty::War { .. } => true, // Wars don't expire automatically
                }
            });
        }
    }

    fn generate_diplomatic_events(
        diplomatic_state: &mut DiplomaticState,
        civs: &HashMap<CivId, CivPersonality>,
        military_strengths: &HashMap<CivId, f32>,
        economic_powers: &HashMap<CivId, f32>,
        rng: &mut impl Rng,
    ) {
        // Randomly generate diplomatic incidents
        if rng.gen_bool(0.02) { // 2% chance per turn
            let civ_ids: Vec<_> = civs.keys().cloned().collect();
            if civ_ids.len() >= 2 {
                let civ_a = civ_ids[rng.gen_range(0..civ_ids.len())];
                let civ_b = civ_ids[rng.gen_range(0..civ_ids.len())];
                
                if civ_a != civ_b {
                    let event = Self::generate_random_diplomatic_event(
                        civ_a, civ_b, civs, military_strengths, economic_powers, rng
                    );
                    
                    if let Some(event) = event {
                        diplomatic_state.diplomatic_events.push(event.clone());
                        
                        // Apply event effects to relations
                        let relation_key = (civ_a, civ_b);
                        if let Some(relation) = diplomatic_state.relations.get_mut(&relation_key) {
                            Self::apply_event_to_relation(&event, relation);
                        }
                    }
                }
            }
        }
    }

    fn generate_random_diplomatic_event(
        civ_a: CivId,
        civ_b: CivId,
        civs: &HashMap<CivId, CivPersonality>,
        _military_strengths: &HashMap<CivId, f32>,
        _economic_powers: &HashMap<CivId, f32>,
        rng: &mut impl Rng,
    ) -> Option<crate::DiplomaticEvent> {
        let personality_a = civs.get(&civ_a)?;
        let personality_b = civs.get(&civ_b)?;
        
        let event_types = [
            DiplomaticEventType::DiplomaticInsult,
            DiplomaticEventType::TradeAgreementSigned,
        ];
        
        let event_type = event_types[rng.gen_range(0..event_types.len())].clone();
        
        // Check if event makes sense given personalities
        let event_probability = match event_type {
            DiplomaticEventType::DiplomaticInsult => {
                personality_a.interventionism * 0.5 + (1.0 - personality_a.honor_treaties) * 0.3
            }
            DiplomaticEventType::TradeAgreementSigned => {
                (personality_a.industry_focus + personality_b.industry_focus) * 0.25
            }
            _ => 0.1,
        };
        
        if rng.gen::<f32>() < event_probability {
            Some(crate::DiplomaticEvent {
                event_type,
                involved_civs: vec![civ_a, civ_b],
                turn: 0, // Will be set by caller
            })
        } else {
            None
        }
    }

    fn apply_event_to_relation(event: &crate::DiplomaticEvent, relation: &mut DiplomaticRelation) {
        match event.event_type {
            DiplomaticEventType::DiplomaticInsult => {
                relation.relation_value -= 10.0;
            }
            DiplomaticEventType::TradeAgreementSigned => {
                relation.trade_agreement = true;
                relation.relation_value += 5.0;
            }
            DiplomaticEventType::AllianceFormed => {
                relation.relation_value += 20.0;
            }
            DiplomaticEventType::WarDeclared => {
                relation.relation_value -= 50.0;
            }
            DiplomaticEventType::PeaceSigned => {
                relation.relation_value += 15.0;
            }
            _ => {}
        }
    }

    /// Calculate the likelihood of war between two civilizations
    pub fn calculate_war_likelihood(
        civ_a: CivId,
        civ_b: CivId,
        diplomatic_state: &DiplomaticState,
        civs: &HashMap<CivId, CivPersonality>,
        military_strengths: &HashMap<CivId, f32>,
    ) -> f32 {
        let relation_key = (civ_a, civ_b);
        let relation = diplomatic_state.relations.get(&relation_key);
        
        if let Some(relation) = relation {
            // Check for existing treaties
            for treaty in &relation.treaties {
                match treaty {
                    Treaty::NonAggression { .. } => return 0.0, // Can't declare war
                    Treaty::Alliance { .. } => return 0.0, // Allies don't fight
                    Treaty::War { .. } => return 0.0, // Already at war
                    _ => {}
                }
            }
            
            let relation_value = relation.relation_value;
            let personality_a = civs.get(&civ_a);
            let personality_b = civs.get(&civ_b);
            
            if let (Some(personality_a), Some(personality_b)) = (personality_a, personality_b) {
                let mut war_likelihood = 0.0;
                
                // Bad relations increase war likelihood
                if relation_value < -20.0 {
                    war_likelihood += (-relation_value - 20.0) / 80.0; // 0.0 to 1.0
                }
                
                // Militaristic civilizations more likely to declare war
                war_likelihood += personality_a.militarism * 0.3;
                
                // High land hunger increases aggression
                war_likelihood += personality_a.land_hunger * 0.2;
                
                // Low honor treaties increases likelihood
                war_likelihood += (1.0 - personality_a.honor_treaties) * 0.2;
                
                // Military strength comparison
                let strength_a = military_strengths.get(&civ_a).unwrap_or(&0.0);
                let strength_b = military_strengths.get(&civ_b).unwrap_or(&0.0);
                
                if *strength_a > *strength_b * 1.5 {
                    war_likelihood += 0.3; // Strong vs weak
                }
                
                war_likelihood.clamp(0.0, 1.0)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Get diplomatic recommendations for AI
    pub fn get_diplomatic_recommendations(
        civ_id: CivId,
        diplomatic_state: &DiplomaticState,
        civs: &HashMap<CivId, CivPersonality>,
        military_strengths: &HashMap<CivId, f32>,
        economic_powers: &HashMap<CivId, f32>,
    ) -> Vec<DiplomaticRecommendation> {
        let mut recommendations = Vec::new();
        let personality = civs.get(&civ_id);
        
        if let Some(personality) = personality {
            for &other_civ in civs.keys() {
                if other_civ == civ_id {
                    continue;
                }
                
                let relation_key = (civ_id, other_civ);
                let relation = diplomatic_state.relations.get(&relation_key);
                
                if let Some(relation) = relation {
                    // Recommend trade agreements for economic civs
                    if personality.industry_focus > 0.6 && !relation.trade_agreement {
                        recommendations.push(DiplomaticRecommendation {
                            target: other_civ,
                            action: DiplomaticAction::ProposeTradePact,
                            priority: personality.industry_focus,
                        });
                    }
                    
                    // Recommend alliances for friendly relations
                    if relation.relation_value > 30.0 && personality.honor_treaties > 0.5 {
                        let has_alliance = relation.treaties.iter().any(|t| matches!(t, Treaty::Alliance { .. }));
                        if !has_alliance {
                            recommendations.push(DiplomaticRecommendation {
                                target: other_civ,
                                action: DiplomaticAction::ProposeAlliance,
                                priority: relation.relation_value / 100.0,
                            });
                        }
                    }
                    
                    // Recommend war for aggressive civs with bad relations
                    if relation.relation_value < -30.0 && personality.militarism > 0.6 {
                        let war_likelihood = Self::calculate_war_likelihood(
                            civ_id, other_civ, diplomatic_state, civs, military_strengths
                        );
                        if war_likelihood > 0.3 {
                            recommendations.push(DiplomaticRecommendation {
                                target: other_civ,
                                action: DiplomaticAction::DeclareWar,
                                priority: war_likelihood,
                            });
                        }
                    }
                }
            }
        }
        
        recommendations.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        recommendations
    }
}

#[derive(Debug, Clone)]
pub struct DiplomaticRecommendation {
    pub target: CivId,
    pub action: DiplomaticAction,
    pub priority: f32,
}

#[derive(Debug, Clone)]
pub enum DiplomaticAction {
    ProposeTradePact,
    ProposeAlliance,
    ProposeNonAggression,
    DeclareWar,
    OfferPeace,
    SendGift,
    DemandTribute,
}
