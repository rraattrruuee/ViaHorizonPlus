use serde::{Deserialize, Serialize};

/// Represents a Minecraft protocol version supported by ViaHorizonPlus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ProtocolVersion {
    // Classic / Alpha / Beta
    Classic0_30 = 7,
    Alpha1_2_6 = 8,
    Beta1_7_3 = 14,
    Beta1_8_1 = 17,

    // Release 1.0 - 1.7
    Release1_0 = 22,
    Release1_2_5 = 29,
    Release1_4_7 = 51,
    Release1_5_2 = 61,
    Release1_6_4 = 78,
    Release1_7_2 = 4,
    Release1_7_10 = 5,

    // Release 1.8 - 1.12
    Release1_8 = 47,
    Release1_9 = 107,
    Release1_9_4 = 110,
    Release1_10 = 210,
    Release1_11_2 = 316,
    Release1_12_2 = 340,

    // Release 1.13 - 1.16
    Release1_13_2 = 404,
    Release1_14_4 = 498,
    Release1_15_2 = 578,
    Release1_16_5 = 754,

    // Release 1.17 - 1.20
    Release1_17_1 = 756,
    Release1_18_2 = 758,
    Release1_19_4 = 762,
    Release1_20_2 = 764,
    Release1_20_4 = 765,
    Release1_20_6 = 766,
    Release1_21_1 = 767,
    Release1_21_3 = 768,
    Release1_21_4 = 769,

    // Horizon Target (Minecraft 26.3)
    HorizonNative = 777,
}

impl ProtocolVersion {
    /// Returns the integer protocol number.
    pub const fn protocol_number(self) -> i32 {
        self as i32
    }

    /// Returns human-readable version name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Classic0_30 => "Classic 0.30",
            Self::Alpha1_2_6 => "Alpha 1.2.6",
            Self::Beta1_7_3 => "Beta 1.7.3",
            Self::Beta1_8_1 => "Beta 1.8.1",
            Self::Release1_0 => "1.0.0",
            Self::Release1_2_5 => "1.2.5",
            Self::Release1_4_7 => "1.4.7",
            Self::Release1_5_2 => "1.5.2",
            Self::Release1_6_4 => "1.6.4",
            Self::Release1_7_2 => "1.7.2",
            Self::Release1_7_10 => "1.7.10",
            Self::Release1_8 => "1.8.x",
            Self::Release1_9 => "1.9",
            Self::Release1_9_4 => "1.9.4",
            Self::Release1_10 => "1.10.2",
            Self::Release1_11_2 => "1.11.2",
            Self::Release1_12_2 => "1.12.2",
            Self::Release1_13_2 => "1.13.2",
            Self::Release1_14_4 => "1.14.4",
            Self::Release1_15_2 => "1.15.2",
            Self::Release1_16_5 => "1.16.5",
            Self::Release1_17_1 => "1.17.1",
            Self::Release1_18_2 => "1.18.2",
            Self::Release1_19_4 => "1.19.4",
            Self::Release1_20_2 => "1.20.2",
            Self::Release1_20_4 => "1.20.4",
            Self::Release1_20_6 => "1.20.6",
            Self::Release1_21_1 => "1.21.1",
            Self::Release1_21_3 => "1.21.3",
            Self::Release1_21_4 => "1.21.4",
            Self::HorizonNative => "Horizon (26.3)",
        }
    }

    /// Returns the release era / group.
    pub const fn era(self) -> VersionEra {
        match self {
            Self::Classic0_30 => VersionEra::Classic,
            Self::Alpha1_2_6 => VersionEra::Alpha,
            Self::Beta1_7_3 | Self::Beta1_8_1 => VersionEra::Beta,
            Self::Release1_0
            | Self::Release1_2_5
            | Self::Release1_4_7
            | Self::Release1_5_2
            | Self::Release1_6_4
            | Self::Release1_7_2
            | Self::Release1_7_10
            | Self::Release1_8 => VersionEra::Legacy,
            Self::Release1_9
            | Self::Release1_9_4
            | Self::Release1_10
            | Self::Release1_11_2
            | Self::Release1_12_2 => VersionEra::CombatUpdate,
            _ => VersionEra::Modern,
        }
    }

    /// Try to find a protocol from an integer protocol version number.
    pub fn from_protocol_number(proto: i32) -> Option<Self> {
        match proto {
            7 => Some(Self::Classic0_30),
            8 => Some(Self::Alpha1_2_6),
            14 => Some(Self::Beta1_7_3),
            17 => Some(Self::Beta1_8_1),
            22 => Some(Self::Release1_0),
            29 => Some(Self::Release1_2_5),
            51 => Some(Self::Release1_4_7),
            61 => Some(Self::Release1_5_2),
            78 => Some(Self::Release1_6_4),
            4 => Some(Self::Release1_7_2),
            5 => Some(Self::Release1_7_10),
            47 => Some(Self::Release1_8),
            107 => Some(Self::Release1_9),
            110 => Some(Self::Release1_9_4),
            210 => Some(Self::Release1_10),
            316 => Some(Self::Release1_11_2),
            340 => Some(Self::Release1_12_2),
            404 => Some(Self::Release1_13_2),
            498 => Some(Self::Release1_14_4),
            578 => Some(Self::Release1_15_2),
            754 => Some(Self::Release1_16_5),
            756 => Some(Self::Release1_17_1),
            758 => Some(Self::Release1_18_2),
            762 => Some(Self::Release1_19_4),
            764 => Some(Self::Release1_20_2),
            765 => Some(Self::Release1_20_4),
            766 => Some(Self::Release1_20_6),
            767 => Some(Self::Release1_21_1),
            768 => Some(Self::Release1_21_3),
            769 => Some(Self::Release1_21_4),
            777 => Some(Self::HorizonNative),
            _ => None,
        }
    }

    /// Returns a list of all commonly targeted versions for the UI selector.
    pub fn all_selectable() -> &'static [ProtocolVersion] {
        &[
            ProtocolVersion::HorizonNative,
            ProtocolVersion::Release1_21_4,
            ProtocolVersion::Release1_20_4,
            ProtocolVersion::Release1_19_4,
            ProtocolVersion::Release1_18_2,
            ProtocolVersion::Release1_16_5,
            ProtocolVersion::Release1_12_2,
            ProtocolVersion::Release1_8,
            ProtocolVersion::Release1_7_10,
            ProtocolVersion::Beta1_7_3,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionEra {
    Classic,
    Alpha,
    Beta,
    Legacy,
    CombatUpdate,
    Modern,
}
