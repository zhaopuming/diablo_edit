use binrw::prelude::*;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use ts_rs::TS;

#[binread]
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[br(little)]
pub struct QuestInfo {
    #[br(assert(magic == 0x216F6F57, "Invalid Quest Magic: expected 0x216F6F57 ('Wo o!')"))]
    pub magic: u32,
    pub acts: u32,
    pub size: u16,
    
    #[serde(with = "BigArray")]
    #[ts(type = "number[]")]
    pub data: [u8; 288],
}
