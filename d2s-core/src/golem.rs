use binrw::binread;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[binread]
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[br(little)]
pub struct Golem {
    #[br(assert(magic == 0x666B, "Invalid Golem Magic: expected 'kf'"))]
    pub magic: u16,
    pub exists: u8,
    // If exists == 1, more data might follow. For now we assume 0 for simple saves.
}
