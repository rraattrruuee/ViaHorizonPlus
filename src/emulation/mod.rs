pub mod combat;
pub mod movement;
pub mod visuals;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmulationProfile {
    pub combat: combat::CombatEmulationSettings,
    pub movement: movement::MovementEmulationSettings,
    pub visuals: visuals::VisualEmulationSettings,
}
