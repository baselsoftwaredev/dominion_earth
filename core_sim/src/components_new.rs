//! # Component System for Dominion Earth
//! 
//! This module contains all the ECS components used in the core simulation.
//! Components are organized into logical sections for better maintainability.
//! 
//! ## Architecture Notes
//! - All components use manual `Component` implementations to avoid proc macro issues
//! - Components are designed to be data-only (no business logic)
//! - Related functionality is implemented through systems in the `systems` module

pub mod components;
pub use components::*;
