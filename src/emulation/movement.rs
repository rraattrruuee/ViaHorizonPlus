use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementEmulationSettings {
    /// Restore 1.8 sneaking eye height (1.54m instead of 1.27m)
    pub legacy_sneak_height: bool,
    /// Restore 1.8 ladder climbing speed and snap
    pub legacy_ladder_climbing: bool,
    /// Emulate old water movement friction
    pub legacy_water_physics: bool,
}

impl Default for MovementEmulationSettings {
    fn default() -> Self {
        Self {
            legacy_sneak_height: true,
            legacy_ladder_climbing: true,
            legacy_water_physics: false,
        }
    }
}
