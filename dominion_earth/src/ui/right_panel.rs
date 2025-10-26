//! Native Bevy UI implementation for the right side panel.
//!
//! Displays world statistics, hovered tile information, civilizations list, and minimap.
//!
//! This module is organized into separate sections for better maintainability:
//! - `statistics_section`: World statistics (turn count, terrain counts)
//! - `hovered_tile_section`: Information about the currently hovered tile
//! - `civilizations_section`: List of all civilizations in the game
//! - `minimap_section`: Minimap panel (placeholder for future implementation)

// Re-export the modular implementation
pub use right_panel::*;

mod right_panel;
