use binrw::binrw;
use serde::{Deserialize, Serialize};

#[binrw]
#[derive(Debug, Serialize, Deserialize)]
#[brw(little)]
pub struct QuestInfoData {
    pub introduced_act1: u16,
    pub act1: [u16; 6],
    pub travel_act1_to_act2: u16,
    pub introduced_act2: u16,
    pub act2: [u16; 6],
    pub travel_act2_to_act3: u16,
    pub introduced_act3: u16,
    pub act3: [u16; 6],
    pub travel_act3_to_act4: u16,
    pub introduced_act4: u16,
    pub act4: [u16; 3],  // Note: Only 3 quests in Act 4
    pub travel_act4: u16,
    pub unk1: [u16; 3],
    pub introduced_act5: u16,
    pub unk2: [u16; 2],
    pub act5: [u16; 6],
    pub reset_stats: u8,
    pub unk3: u8,
    pub unk4: [u16; 6],
}

#[binrw]
#[derive(Debug, Serialize, Deserialize)]
#[brw(little)]
pub struct QuestInfo {
    #[br(assert(magic == 0x216F6F57, "Invalid Quest Info Magic: expected 0x216F6F57"))]
    pub magic: u32,
    pub acts: u32,       // Should be 6
    pub size: u16,       // 0x12A (298)
    pub modes: [QuestInfoData; 3], // Normal, Nightmare, Hell
}
