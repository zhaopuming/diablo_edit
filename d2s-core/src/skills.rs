use binrw::binrw;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[binrw]
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[brw(little)]
pub struct CharSkills {
    #[br(assert(magic == 0x6669, "Invalid Skills Magic: expected 0x6669 (if)"))]
    #[bw(assert(*magic == 0x6669))]
    pub magic: u16,
    
    #[ts(type = "number[]")]
    pub skills: [u8; 30],
}
