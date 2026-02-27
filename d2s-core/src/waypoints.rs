use binrw::binrw;
use serde::{Deserialize, Serialize};

#[binrw]
#[derive(Debug, Serialize, Deserialize)]
#[brw(little)]
pub struct WaypointData {
    pub unk1: u16,        // 0x102
    pub waypoints: [u8; 5],
    pub unk2: [u8; 17],   // Zeroes
}

#[binrw]
#[derive(Debug, Serialize, Deserialize)]
#[brw(little)]
pub struct Waypoints {
    #[br(assert(magic == 0x5357, "Invalid Waypoints Magic: expected 0x5357 (WS)"))]
    pub magic: u16,
    pub unk1: u32,
    pub size: u16,        // 0x50 (80)
    pub modes: [WaypointData; 3], // Normal, Nightmare, Hell
}
