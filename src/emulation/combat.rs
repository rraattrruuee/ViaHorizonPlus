use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEmulationSettings {
    /// Emulate 1.8 sword blocking instead of shield usage
    pub sword_blocking: bool,
    /// Disable attack cooldown indicator and logic on legacy servers
    pub disable_attack_cooldown: bool,
    /// Restore 1.8 hit sound and critical particle mechanics
    pub legacy_hit_feedback: bool,
}

impl Default for CombatEmulationSettings {
    fn default() -> Self {
        Self {
            sword_blocking: true,
            disable_attack_cooldown: true,
            legacy_hit_feedback: true,
        }
    }
}
