use binrw::binread;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use ts_rs::TS;

#[binread]
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[br(little)]
pub struct D2sHeader {
    pub magic: u32,       // 0xAA55AA55
    pub version: u32,     // 0x5C = 1.09, 0x60 = 1.10, 0x61 = D2R
    pub file_size: u32,
    pub checksum: u32,
    
    pub active_arms: u32,
    
    #[ts(type = "number[]")]
    pub name: [u8; 16],
    
    pub char_status: u8,
    pub char_progres: u8,
    pub unknown_1: [u8; 2],
    pub char_class: u8,
    pub unknown_2: [u8; 2],
    pub char_level: u8,
    pub unknown_3: u32,
    pub last_played: u32,
    pub unknown_4: u32,
    
    #[ts(type = "number[]")]
    pub skill_hotkeys: [u32; 16],
    
    pub left_skill: u32,
    pub right_skill: u32,
    pub left_switch_skill: u32,
    pub right_switch_skill: u32,
    
    #[ts(type = "number[]")]
    pub menu_appearance: [u8; 32],
    
    pub difficulty: [u8; 3],
    pub map_id: u32,
    pub unknown_5: [u8; 2],
    pub dead_corpse: u16,
    pub unknown_6: u32,
    pub unknown_7: u32,
    pub unknown_8: u32,
    pub unknown_9: u32,
    pub unknown_10: u32,
    
    #[serde(with = "BigArray")]
    #[ts(type = "number[]")]
    pub unk7: [u8; 0x4C],
    
    pub unk8: u32,
}
