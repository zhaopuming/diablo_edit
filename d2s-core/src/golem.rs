use binrw::binrw;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[binrw]
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[brw(little)]
pub struct Golem {
    #[br(assert(magic == 0x666B, "Invalid Golem Magic: expected 'kf'"))]
    #[bw(assert(*magic == 0x666B))]
    pub magic: u16,
    pub exists: u8,
    // If exists == 1, more data might follow. For now we assume 0 for simple saves.
}
