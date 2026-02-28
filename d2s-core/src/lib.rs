pub mod header;
pub mod quests;
pub mod waypoints;
pub mod skills;
pub mod stats;
pub mod items;
pub mod corpse;
pub mod mercenary;
pub mod golem;
pub mod huffman;
pub mod properties;
pub mod metadata;

use binrw::binrw;
use serde::{Deserialize, Serialize};

use header::D2sHeader;
use quests::QuestInfo;
use waypoints::Waypoints;
use skills::CharSkills;
use stats::PlayerStats;
use items::ItemList;
use corpse::Corpse;
use mercenary::Mercenary;
use golem::Golem;
use serde_big_array::BigArray;
use ts_rs::TS;

#[binrw]
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[brw(little)]
pub struct D2sSave {
    pub header: D2sHeader,
    pub quests: QuestInfo,
    pub waypoints: Waypoints,

    // NPC Dialog Intro Flags
    #[serde(with = "BigArray")]
    #[ts(type = "number[]")]
    pub npc_data: [u8; 0x34],

    pub stats: PlayerStats,
    pub skills: CharSkills,
    pub items: ItemList,
    pub corpse: Corpse,
    pub mercenary: Mercenary,
    pub golem: Golem,
}

use binrw::BinRead;
use binrw::BinWrite;
use std::io::Cursor;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO/Parsing Error: {0}")]
    BinrwError(#[from] binrw::Error),
}

#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("IO/Serialization Error: {0}")]
    BinrwError(#[from] binrw::Error),
}

/// Parses a D2S byte slice into a completely structured save representation.
pub fn parse_d2s(bytes: &[u8]) -> Result<D2sSave, ParseError> {
    let mut reader = Cursor::new(bytes);
    let save = D2sSave::read(&mut reader)?;
    Ok(save)
}

/// Serializes a D2sSave back to bytes.
/// Note: This requires all structs to implement BinWrite.
pub fn serialize_d2s(save: &D2sSave) -> Result<Vec<u8>, SerializeError> {
    let mut writer = Cursor::new(Vec::new());
    save.write(&mut writer)?;
    Ok(writer.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal() {
        println!("INTERNAL TEST RUNNING");
        assert_eq!(1+1, 2);
    }

    #[test]
    fn export_types() {
        use ts_rs::TS;
        // This triggers the export to bindings/
        let _ = D2sSave::decl();
    }
}
