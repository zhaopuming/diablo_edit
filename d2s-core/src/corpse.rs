use binrw::binrw;
use serde::{Deserialize, Serialize};
use crate::items::ItemList;
use ts_rs::TS;

#[binrw]
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[brw(little)]
pub struct Corpse {
    #[br(assert(magic == 0x4D4A, "Invalid Corpse Magic: expected 'JM'"))]
    #[bw(assert(*magic == 0x4D4A))]
    pub magic: u16,

    #[bw(calc = corpses.len() as u16)]
    pub count: u16,

    #[br(count = count)]
    pub corpses: Vec<CorpseData>,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[brw(little)]
pub struct CorpseData {
    #[ts(type = "number[]")]
    pub unknown: [u8; 12],
    pub items: ItemList,
}
