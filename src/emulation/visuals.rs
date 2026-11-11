use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualEmulationSettings {
    /// Restore 1.7/1.8 block-hitting animation in third & first person
    pub block_hit_animation: bool,
    /// Restore old red damage tint on entities
    pub legacy_damage_tint: bool,
    /// Render old sneak transitions (instant vs smooth interpolate)
    pub instant_sneak_animation: bool,
}

impl Default for VisualEmulationSettings {
    fn default() -> Self {
        Self {
            block_hit_animation: true,
            legacy_damage_tint: true,
            instant_sneak_animation: false,
        }
    }
}
