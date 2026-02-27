use binrw::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};
use bitstream_io::{BitRead, BitReader, LittleEndian};
use crate::huffman::get_huffman;
use crate::metadata::get_item_meta;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct D2Item {
    pub data: ItemData,
    pub socketed_items: Vec<D2Item>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct ItemData {
    pub identified: bool,
    pub socketed: bool,
    pub ethereal: bool,
    pub personalized: bool,
    pub runeword: bool,
    pub simple: bool,
    pub location: u8,
    pub position: u8,
    pub column: u8,
    pub row: u8,
    pub container: u8,
    #[ts(type = "number[]")]
    pub type_id: [u8; 4],
    pub is_ear: bool,
}

#[binread]
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[br(little)]
pub struct ItemList {
    #[br(assert(magic == 0x4D4A, "Invalid Items Magic: expected 'JM'"))]
    pub magic: u16,
    pub count: u16,
    
    #[br(parse_with = parse_items, args(count))]
    pub items: Vec<D2Item>,
}

pub fn parse_items<R: Read + Seek>(
    reader: &mut R,
    _endian: binrw::Endian,
    args: (u16,),
) -> BinResult<Vec<D2Item>> {
    let count = args.0;
    let mut items = Vec::with_capacity(count as usize);
    let mut bit_reader = BitReader::endian(reader, LittleEndian);
    
    for i in 0..count {
        items.push(parse_single_item(&mut bit_reader, i, 0, 0x63)?);
    }
    
    Ok(items)
}

fn parse_single_item<R: Read>(
    bit_reader: &mut BitReader<R, LittleEndian>,
    index: u16,
    depth: u8,
    version: u32,
) -> BinResult<D2Item> {
    if depth > 10 {
        return Err(binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "Recursion limit")));
    }

    let mut cur_ver = version;
    let magic = bit_reader.read::<16, u16>().map_err(io_err)?;
    
    let flags_27: u32;
    if magic == 0x4D4A {
        let ver = bit_reader.read::<32, u32>().map_err(io_err)?;
        if is_valid_version(ver) {
             cur_ver = ver;
             flags_27 = bit_reader.read::<27, u32>().map_err(io_err)?;
        } else {
             // Rollback: read flags using magic as start
             let extra = bit_reader.read::<11, u16>().map_err(io_err)? as u32;
             flags_27 = (magic as u32) | (extra << 16);
             return Err(binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "Magic rollback fail")));
        }
    } else {
        let low_16 = magic as u32;
        let high_11 = bit_reader.read::<11, u16>().map_err(io_err)? as u32;
        flags_27 = low_16 | (high_11 << 16);
    }

    let identified   = (flags_27 & (1 << 4)) != 0;
    let socketed     = (flags_27 & (1 << 11)) != 0;
    let is_ear       = (flags_27 & (1 << 16)) != 0;
    let simple       = (flags_27 & (1 << 21)) != 0;
    let ethereal     = (flags_27 & (1 << 22)) != 0;
    let personalized = (flags_27 & (1 << 24)) != 0;
    let runeword     = (flags_27 & (1 << 26)) != 0;

    if cur_ver >= 0x61 {
        let _ = bit_reader.read::<8, u8>().map_err(io_err)?; 
    } else {
        let _ = bit_reader.read::<15, u16>().map_err(io_err)?; 
    }
    
    let location     = bit_reader.read::<3, u8>().map_err(io_err)?;
    let position     = bit_reader.read::<4, u8>().map_err(io_err)?;
    let column       = bit_reader.read::<4, u8>().map_err(io_err)?;
    let row          = bit_reader.read::<4, u8>().map_err(io_err)?;
    let container    = bit_reader.read::<3, u8>().map_err(io_err)?;

    let mut type_id = [0u8; 4];
    if !is_ear {
        for i in 0..4 {
            type_id[i] = get_huffman().read_char(bit_reader).map_err(|e| binrw::Error::Io(e))?;
        }
    } else {
        let _class = bit_reader.read::<3, u8>().map_err(io_err)?;
        let _level = bit_reader.read::<7, u8>().map_err(io_err)?;
        loop {
            let c = bit_reader.read::<7, u8>().map_err(io_err)?;
            if c == 0 { break; }
        }
    }

    let meta = get_item_meta(&type_id);
    // println!("Item {}: ID='{}', flags={:07X}, simple={}, ear={}, meta={:?}", 
    //     index, String::from_utf8_lossy(&type_id), flags_27, simple, is_ear,
    //     meta.map(|m| (m.simple, m.def, m.dur, m.ipad_bits)));
    
    let mut n_gems = 0;
    
    if !simple {
        // CExtItemInfo::ReadData
        n_gems      = bit_reader.read::<3, u8>().map_err(io_err)?;
        let _guid       = bit_reader.read::<32, u32>().map_err(io_err)?;
        let _drop_level  = bit_reader.read::<7, u8>().map_err(io_err)?;
        let quality     = bit_reader.read::<4, u8>().map_err(io_err)?;
        
        let var_gfx     = bit_reader.read_bit().map_err(io_err)?;
        if var_gfx { let _ = bit_reader.read::<3, u8>().map_err(io_err)?; }
        
        let has_class   = bit_reader.read_bit().map_err(io_err)?;
        if has_class { let _ = bit_reader.read::<11, u16>().map_err(io_err)?; }

        match quality {
            1 => { let _ = bit_reader.read::<3, u8>().map_err(io_err)?; } // low quality
            2 => { // normal
                if let Some(m) = meta {
                    if m.charm { let _ = bit_reader.read::<12, u16>().map_err(io_err)?; }
                }
            }
            3 => { let _ = bit_reader.read::<3, u8>().map_err(io_err)?; } // high quality
            4 => { // magic
                let _ = bit_reader.read::<11, u16>().map_err(io_err)?; 
                let _ = bit_reader.read::<11, u16>().map_err(io_err)?; 
            }
            5 => { let _ = bit_reader.read::<12, u16>().map_err(io_err)?; } // set
            6 | 8 => { // rare / crafted
                let _name1 = bit_reader.read::<8, u8>().map_err(io_err)?;
                let _name2 = bit_reader.read::<8, u8>().map_err(io_err)?;
                for _ in 0..3 {
                    if bit_reader.read_bit().map_err(io_err)? { let _ = bit_reader.read::<11, u16>().map_err(io_err)?; }
                    if bit_reader.read_bit().map_err(io_err)? { let _ = bit_reader.read::<11, u16>().map_err(io_err)?; }
                }
            }
            7 => { let _ = bit_reader.read::<12, u16>().map_err(io_err)?; } // unique
            _ => {}
        }
        
        if runeword {
            let _ = bit_reader.read::<16, u16>().map_err(io_err)?; 
        }
        
        if personalized {
            let b = if cur_ver >= 0x62 { 8 } else { 7 };
            loop {
                let c = if b == 8 { bit_reader.read::<8, u8>().map_err(io_err)? } 
                        else { bit_reader.read::<7, u8>().map_err(io_err)? };
                if c == 0 { break; }
            }
        }
        
        if let Some(m) = meta {
            if m.monster { let _ = bit_reader.read::<10, u16>().map_err(io_err)?; }
            else if m.spell > 0 { let _ = bit_reader.read::<5, u8>().map_err(io_err)?; }
        }
    }

    if type_id == *b"gld " {
         let _ = bit_reader.read::<12, u16>().map_err(io_err)?;
    }

    let b_has_rand = bit_reader.read_bit().map_err(io_err)?;
    if !simple {
        if b_has_rand {
            for _ in 0..4 { let _ = bit_reader.read::<32, u32>().map_err(io_err)?; }
        }
        
        // --- TpSpInfo ---
        if let Some(m) = meta {
            if m.def { let _ = bit_reader.read::<11, u16>().map_err(io_err)?; }
            if m.dur {
                let max_dur = bit_reader.read::<8, u8>().map_err(io_err)?;
                if max_dur > 0 {
                    let _cur_dur = bit_reader.read::<9, u16>().map_err(io_err)?;
                }
            }
            if socketed {
                let _ = bit_reader.read::<4, u8>().map_err(io_err)?;
            }
            if m.stacked {
                let _ = bit_reader.read::<9, u16>().map_err(io_err)?;
            }
        }

        // Property List
        loop {
            let prop_id = bit_reader.read::<9, u16>().map_err(io_err)?;
            if prop_id == 0x1FF { break; }
            let bits = crate::properties::get_property_bits(prop_id, cur_ver);
            if bits > 0 {
                for _ in 0..bits { let _ = bit_reader.read_bit().map_err(io_err)?; }
            } else {
                if prop_id != 0 {
                    // println!("      WARNING: Unknown bits for property {} at item {}", prop_id, index);
                }
            }
        }
    } else {
        // Simple D2R padding
        if let Some(m) = meta {
            if cur_ver >= 0x61 && m.ipad_bits > 0 {
                for _ in 0..m.ipad_bits { let _ = bit_reader.read_bit().map_err(io_err)?; }
            }
        }
    }

    bit_reader.byte_align();
    
    let mut socketed_items = Vec::new();
    if !simple && n_gems > 0 {
        for _ in 0..n_gems {
            socketed_items.push(parse_single_item(bit_reader, index, depth + 1, cur_ver)?);
        }
    }

    Ok(D2Item {
        data: ItemData { identified, socketed, ethereal, personalized, runeword, simple, location, position, column, row, container, type_id, is_ear },
        socketed_items,
    })
}

fn is_valid_version(v: u32) -> bool {
    matches!(v, 0x63 | 0x62 | 0x61 | 0x60 | 0x5C | 0x59 | 0x57 | 0x47)
}

fn io_err(e: std::io::Error) -> binrw::Error { binrw::Error::Io(e) }
