pub mod position;
pub mod civilization;
pub mod military;
pub mod city;
pub mod terrain;
pub mod diplomacy;
pub mod ai;
pub mod action_queue;
pub mod orders;
pub mod rendering;
pub mod player;
pub mod production;
pub mod turn_phases;

// Re-export all components for compatibility
pub use position::*;
pub use civilization::*;
pub use military::*;
pub use city::*;
pub use terrain::*;
pub use diplomacy::*;
pub use ai::*;
pub use action_queue::*;
pub use orders::*;
pub use rendering::*;
pub use player::*;
pub use production::*;
pub use turn_phases::*;
