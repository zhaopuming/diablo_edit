use binrw::binread;
use serde::{Deserialize, Serialize};

#[binread]
#[derive(Debug, Serialize, Deserialize)]
#[br(little)]
pub struct Corpse {
    #[br(assert(magic == 0x4D4A, "Invalid Corpse Magic: expected 'JM'"))]
    pub magic: u16,
    pub count: u16, // always 0 or 1
    // If count == 1, CorpseData follows (12 unknown bytes + ItemList)
}
