use binrw::binrw;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[binrw]
#[derive(Debug, Serialize, Deserialize)]
#[brw(little)]
pub struct D2sHeader {
    pub magic: u32,       // 0xAA55AA55
    pub version: u32,     // 0x5C = 1.09, 0x60 = 1.10, 0x61 = D2R
    pub size: u32,
    pub crc: u32,
    pub active_weapon: u32,
    
    pub name: [u8; 16],
    
    pub char_status: u8,
    pub char_progression: u8,
    pub unk1: u16,
    pub char_class: u8,
    pub unk2: u16,
    pub char_level: u8,
    pub unk3: u32,
    pub timestamp: u32,
    pub unk4: u32,
    
    pub hotkeys: [u32; 16],
    pub left_skill_alt: u32,
    pub right_skill_alt: u32,
    pub left_skill: u32,
    pub right_skill: u32,
    
    pub outfit: [u8; 16],
    pub colors: [u8; 16],
    pub town: [u8; 3],
    pub map_seed: u32,
    pub unk5: u16,
    pub merc_dead: u8,
    pub unk6: u8,
    pub merc_control_seed: u32,
    pub merc_name_id: u16,
    pub merc_type: u16,
    pub merc_exp: u32,
    
    #[serde(with = "BigArray")]
    pub unk7: [u8; 0x4C],
    
    #[serde(with = "BigArray")]
    pub name_ptr: [u8; 64], // PTR 2.4+ UTF8 Name
    
    pub unk8: u32,
}
