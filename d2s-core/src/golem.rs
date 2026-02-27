use binrw::binread;
use serde::{Deserialize, Serialize};

#[binread]
#[derive(Debug, Serialize, Deserialize)]
#[br(little)]
pub struct Golem {
    #[br(assert(magic == 0x666B, "Invalid Golem Magic: expected 'kf'"))]
    pub magic: u16,
    pub has_golem: u8,
    // If has_golem != 0, a CD2Item follows.
}
