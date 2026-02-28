use binrw::binrw;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[binrw]
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[brw(little)]
pub struct Mercenary {
    #[br(assert(magic == 0x666A, "Invalid Mercenary Magic: expected 'jf'"))]
    #[bw(assert(*magic == 0x666A))]
    pub magic: u16,
}
