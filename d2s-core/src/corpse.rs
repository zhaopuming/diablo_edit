use binrw::binread;
use serde::{Deserialize, Serialize};
use crate::items::ItemList;

#[binread]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[br(little)]
pub struct Corpse {
    #[br(assert(magic == 0x4D4A, "Invalid Corpse Magic: expected 'JM'"))]
    pub magic: u16,
    pub count: u16,
    
    #[br(count = count)]
    pub corpses: Vec<CorpseData>,
}

#[binread]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[br(little)]
pub struct CorpseData {
    pub unknown: [u8; 12],
    pub items: ItemList,
}
