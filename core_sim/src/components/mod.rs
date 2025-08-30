pub mod position;
pub mod civilization;
pub mod military;
pub mod city;
pub mod terrain;
pub mod diplomacy;
pub mod ai;
pub mod orders;
pub mod rendering;

// Re-export all components for compatibility
pub use position::*;
pub use civilization::*;
pub use military::*;
pub use city::*;
pub use terrain::*;
pub use diplomacy::*;
pub use ai::*;
pub use orders::*;
pub use rendering::*;
