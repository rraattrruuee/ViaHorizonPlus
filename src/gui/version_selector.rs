use crate::protocol::ProtocolVersion;
use crate::state::global_state;

/// UI state for the ViaHorizonPlus version selection menu.
pub struct VersionSelectorUi {
    pub is_open: bool,
    pub search_query: String,
    pub selected_index: usize,
}

impl Default for VersionSelectorUi {
    fn default() -> Self {
        Self {
            is_open: false,
            search_query: String::new(),
            selected_index: 0,
        }
    }
}

impl VersionSelectorUi {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the list of selectable versions filtered by the search query.
    pub fn filtered_versions(&self) -> Vec<ProtocolVersion> {
        let all = ProtocolVersion::all_selectable();
        if self.search_query.trim().is_empty() {
            return all.to_vec();
        }
        let query = self.search_query.to_lowercase();
        all.iter()
            .copied()
            .filter(|v| v.name().to_lowercase().contains(&query))
            .collect()
    }

    /// Selects the currently highlighted version and updates the global mod state.
    pub fn apply_selection(&self) {
        let versions = self.filtered_versions();
        if let Some(&chosen) = versions.get(self.selected_index) {
            global_state().set_target_protocol(chosen);
        }
    }
}
