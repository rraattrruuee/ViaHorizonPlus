use crate::emulation::EmulationProfile;
use crate::protocol::ProtocolVersion;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViaHorizonConfig {
    /// Manually selected target protocol, or default
    pub target_protocol: ProtocolVersion,
    /// Automatically negotiate version via server ping
    pub auto_detect: bool,
    /// Show version badge in HUD
    pub hud_indicator: bool,
    /// Active emulation settings
    pub emulation: EmulationProfile,
}

impl Default for ViaHorizonConfig {
    fn default() -> Self {
        Self {
            target_protocol: ProtocolVersion::HorizonNative,
            auto_detect: true,
            hud_indicator: true,
            emulation: EmulationProfile::default(),
        }
    }
}

pub struct ViaHorizonState {
    pub config: RwLock<ViaHorizonConfig>,
    pub current_connected_protocol: RwLock<Option<ProtocolVersion>>,
}

impl ViaHorizonState {
    pub fn new() -> Self {
        Self {
            config: RwLock::new(ViaHorizonConfig::default()),
            current_connected_protocol: RwLock::new(None),
        }
    }

    /// Sets the active target protocol version.
    pub fn set_target_protocol(&self, version: ProtocolVersion) {
        let mut cfg = self.config.write();
        cfg.target_protocol = version;
    }

    /// Returns the target protocol version currently requested.
    pub fn get_target_protocol(&self) -> ProtocolVersion {
        self.config.read().target_protocol
    }

    /// Sets the connected server protocol once connected.
    pub fn set_connected_protocol(&self, version: Option<ProtocolVersion>) {
        *self.current_connected_protocol.write() = version;
    }

    /// Returns the active protocol (either current connected server protocol or target).
    pub fn effective_protocol(&self) -> ProtocolVersion {
        if let Some(connected) = *self.current_connected_protocol.read() {
            connected
        } else {
            self.config.read().target_protocol
        }
    }
}

// Global thread-safe singleton
static STATE: parking_lot::Mutex<Option<Arc<ViaHorizonState>>> =
    parking_lot::Mutex::new(None);

pub fn global_state() -> Arc<ViaHorizonState> {
    let mut lock = STATE.lock();
    if let Some(state) = lock.as_ref() {
        return Arc::clone(state);
    }
    let state = Arc::new(ViaHorizonState::new());
    *lock = Some(Arc::clone(&state));
    state
}
