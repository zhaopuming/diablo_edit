use binrw::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};
use bitstream_io::{BitRead, BitReader, LittleEndian};
use ts_rs::TS;

#[binread]
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[br(little)]
pub struct PlayerStats {
    #[br(assert(magic == 0x6667, "Invalid Stats Magic: expected 0x6667 (gf)"))]
    pub magic: u16,
    
    #[br(parse_with = parse_stats_values)]
    pub values: std::collections::HashMap<u16, u32>,
}


const STAT_BITS: [u32; 16] = [
    10, 10, 10, 10, 10, 8,
    21, 21, 21, 21, 21, 21,
    7, 32, 25, 25
];

pub fn parse_stats_values<R: Read + Seek>(
    reader: &mut R,
    _endian: binrw::Endian,
    _args: (),
) -> BinResult<std::collections::HashMap<u16, u32>> {
    let mut values = std::collections::HashMap::new();
    let mut bit_reader = BitReader::endian(reader, LittleEndian);

    loop {
        // Read 9-bit ID using const generic (bitstream-io 4.x)
        let id = bit_reader.read::<9, u16>().map_err(|e| binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        
        if id == 0x1FF {
            break;
        }

        if (id as usize) < STAT_BITS.len() {
            let bits = STAT_BITS[id as usize];
            // Since bitstream-io 4.x read() is const generic, 
            // for dynamic bits we might need read_container or a manual loop/match.
            // However, most D2 bitstream readers use a runtime bit count.
            // Let's try bit_reader.read_u32(bits) which is often available in these crates.
            // Or better: bit_reader.read_type::<u32>(bits)
            let val = match bits {
                7 => bit_reader.read::<7, u32>().map_err(|e| binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?,
                8 => bit_reader.read::<8, u32>().map_err(|e| binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?,
                10 => bit_reader.read::<10, u32>().map_err(|e| binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?,
                21 => bit_reader.read::<21, u32>().map_err(|e| binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?,
                25 => bit_reader.read::<25, u32>().map_err(|e| binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?,
                32 => bit_reader.read::<32, u32>().map_err(|e| binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?,
                _ => break, // Fallback
            };
            values.insert(id, val);
        } else {
            // Unknown stat ID
            break; 
        }
    }

    bit_reader.byte_align();
    Ok(values)
}
