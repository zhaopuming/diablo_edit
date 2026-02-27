use binrw::binrw;
use serde::{Deserialize, Serialize};

#[binrw]
#[derive(Debug, Serialize, Deserialize)]
#[brw(little)]
pub struct CharSkills {
    #[br(assert(magic == 0x6669, "Invalid Skills Magic: expected 0x6669 (if)"))]
    pub magic: u16,
    pub skills: [u8; 30],
}
