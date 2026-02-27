use binrw::binread;
use serde::{Deserialize, Serialize};

#[binread]
#[derive(Debug, Serialize, Deserialize)]
#[br(little)]
pub struct Mercenary {
    #[br(assert(magic == 0x666A, "Invalid Mercenary Magic: expected 'jf'"))]
    pub magic: u16,
    
    // In D2S, the items for mercenary only exist if the character has a mercenary.
    // This is often controlled by a flag in the header (dwMercControl)
    // For now, we'll try to read it as a MayExist pattern or handle it in lib.rs logic.
}
