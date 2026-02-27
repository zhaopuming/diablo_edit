use binrw::binrw;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[binrw]
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[brw(little)]
pub struct DifficultyWaypoints {
    pub unknown: [u8; 2],
    pub waypoints: [u8; 9],
    pub unknown2: [u8; 17],
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[brw(little)]
pub struct Waypoints {
    #[br(assert(magic == 0x5357, "Invalid Waypoints Magic: expected 'WS' (0x5357)"))]
    pub magic: u16,
    pub unknown: u16,
    pub size: u16,
    pub difficulties: [DifficultyWaypoints; 3], // Normal, Nightmare, Hell
}
