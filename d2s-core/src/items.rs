use binrw::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, Write};
use bitstream_io::{BitRead, BitReader, BitWrite, BitWriter, LittleEndian};
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
    pub name: String,
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
    pub width: u8,
    pub height: u8,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[brw(little)]
pub struct ItemList {
    #[br(assert(magic == 0x4D4A, "Invalid Items Magic: expected 'JM'"))]
    #[bw(assert(*magic == 0x4D4A))]
    pub magic: u16,

    #[bw(calc = items.len() as u16)]
    pub count: u16,

    #[br(parse_with = parse_items, args(count))]
    #[bw(write_with = write_items)]
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

pub fn write_items<W: Write + Seek>(
    items: &Vec<D2Item>,
    writer: &mut W,
    _endian: binrw::Endian,
    _args: (),
) -> BinResult<()> {
    let mut bit_writer = BitWriter::endian(writer, LittleEndian);

    for item in items {
        write_single_item(&mut bit_writer, item, 0x63)?;
    }

    bit_writer.byte_align().map_err(io_err)?;
    Ok(())
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

    let mut n_gems = 0;

    if !simple {
        n_gems      = bit_reader.read::<3, u8>().map_err(io_err)?;
        let _guid       = bit_reader.read::<32, u32>().map_err(io_err)?;
        let _drop_level  = bit_reader.read::<7, u8>().map_err(io_err)?;
        let quality     = bit_reader.read::<4, u8>().map_err(io_err)?;

        let var_gfx     = bit_reader.read_bit().map_err(io_err)?;
        if var_gfx { let _ = bit_reader.read::<3, u8>().map_err(io_err)?; }

        let has_class   = bit_reader.read_bit().map_err(io_err)?;
        if has_class { let _ = bit_reader.read::<11, u16>().map_err(io_err)?; }

        match quality {
            1 => { let _ = bit_reader.read::<3, u8>().map_err(io_err)?; }
            2 => {
                if let Some(m) = meta {
                    if m.charm { let _ = bit_reader.read::<12, u16>().map_err(io_err)?; }
                }
            }
            3 => { let _ = bit_reader.read::<3, u8>().map_err(io_err)?; }
            4 => {
                let _ = bit_reader.read::<11, u16>().map_err(io_err)?;
                let _ = bit_reader.read::<11, u16>().map_err(io_err)?;
            }
            5 => { let _ = bit_reader.read::<12, u16>().map_err(io_err)?; }
            6 | 8 => {
                let _name1 = bit_reader.read::<8, u8>().map_err(io_err)?;
                let _name2 = bit_reader.read::<8, u8>().map_err(io_err)?;
                for _ in 0..3 {
                    if bit_reader.read_bit().map_err(io_err)? { let _ = bit_reader.read::<11, u16>().map_err(io_err)?; }
                    if bit_reader.read_bit().map_err(io_err)? { let _ = bit_reader.read::<11, u16>().map_err(io_err)?; }
                }
            }
            7 => { let _ = bit_reader.read::<12, u16>().map_err(io_err)?; }
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

        loop {
            let prop_id = bit_reader.read::<9, u16>().map_err(io_err)?;
            if prop_id == 0x1FF { break; }
            let bits = crate::properties::get_property_bits(prop_id, cur_ver);
            if bits > 0 {
                for _ in 0..bits { let _ = bit_reader.read_bit().map_err(io_err)?; }
            }
        }
    } else {
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

    let meta = get_item_meta(&type_id);
    let mut width = 1;
    let mut height = 1;
    let mut name = String::from_utf8_lossy(&type_id).trim().to_string();
    if let Some(m) = meta {
        width = m.width;
        height = m.height;
        name = m.name.to_string();
    }

    Ok(D2Item {
        data: ItemData { name, identified, socketed, ethereal, personalized, runeword, simple, location, position, column, row, container, type_id, is_ear, width, height },
        socketed_items,
    })
}

fn write_single_item<W: Write>(
    bit_writer: &mut BitWriter<W, LittleEndian>,
    item: &D2Item,
    version: u32,
) -> BinResult<()> {
    // Build flags_27 from item data
    let mut flags_27: u32 = 0;
    if item.data.identified { flags_27 |= 1 << 4; }
    if item.data.socketed { flags_27 |= 1 << 11; }
    if item.data.is_ear { flags_27 |= 1 << 16; }
    if item.data.simple { flags_27 |= 1 << 21; }
    if item.data.ethereal { flags_27 |= 1 << 22; }
    if item.data.personalized { flags_27 |= 1 << 24; }
    if item.data.runeword { flags_27 |= 1 << 26; }

    // Write JM magic + version + flags_27
    bit_writer.write::<16, u16>(0x4D4A).map_err(io_err)?;
    bit_writer.write::<32, u32>(version).map_err(io_err)?;
    bit_writer.write::<27, u32>(flags_27).map_err(io_err)?;

    // Write padding based on version
    if version >= 0x61 {
        bit_writer.write::<8, u8>(0).map_err(io_err)?;
    } else {
        bit_writer.write::<15, u16>(0).map_err(io_err)?;
    }

    // Write location info
    bit_writer.write::<3, u8>(item.data.location).map_err(io_err)?;
    bit_writer.write::<4, u8>(item.data.position).map_err(io_err)?;
    bit_writer.write::<4, u8>(item.data.column).map_err(io_err)?;
    bit_writer.write::<4, u8>(item.data.row).map_err(io_err)?;
    bit_writer.write::<3, u8>(item.data.container).map_err(io_err)?;

    // Write type_id using Huffman encoding
    if !item.data.is_ear {
        for &c in &item.data.type_id {
            get_huffman().write_char(bit_writer, c).map_err(|e| binrw::Error::Io(e))?;
        }
    } else {
        // Ear item - simplified, not commonly edited
        bit_writer.write::<3, u8>(0).map_err(io_err)?; // class
        bit_writer.write::<7, u8>(0).map_err(io_err)?; // level
        bit_writer.write::<7, u8>(0).map_err(io_err)?; // null terminator
    }

    // For simple items, just write padding and end
    if item.data.simple {
        let meta = get_item_meta(&item.data.type_id);
        if let Some(m) = meta {
            if version >= 0x61 && m.ipad_bits > 0 {
                for _ in 0..m.ipad_bits {
                    bit_writer.write_bit(false).map_err(io_err)?;
                }
            }
        }
    } else {
        // Extended item - write socketed count
        let n_gems = item.socketed_items.len() as u8;
        bit_writer.write::<3, u8>(n_gems).map_err(io_err)?;

        // Write placeholder for extended item data
        // Note: This is simplified - full implementation would need all quality-specific data
        bit_writer.write::<32, u32>(0).map_err(io_err)?; // guid
        bit_writer.write::<7, u8>(0).map_err(io_err)?; // drop_level
        bit_writer.write::<4, u8>(2).map_err(io_err)?; // quality = normal

        // TpSpInfo placeholder
        let meta = get_item_meta(&item.data.type_id);
        if let Some(m) = &meta {
            if m.def { bit_writer.write::<11, u16>(0).map_err(io_err)?; }
            if m.dur {
                bit_writer.write::<8, u8>(0).map_err(io_err)?; // max_dur
            }
            if item.data.socketed {
                bit_writer.write::<4, u8>(0).map_err(io_err)?;
            }
            if m.stacked {
                bit_writer.write::<9, u16>(0).map_err(io_err)?;
            }
        }

        // End of property list
        bit_writer.write::<9, u16>(0x1FF).map_err(io_err)?;

        // Random data flag
        bit_writer.write_bit(false).map_err(io_err)?;
    }

    bit_writer.byte_align().map_err(io_err)?;

    // Write socketed items recursively
    for socketed_item in &item.socketed_items {
        write_single_item(bit_writer, socketed_item, version)?;
    }

    Ok(())
}

fn is_valid_version(v: u32) -> bool {
    matches!(v, 0x63 | 0x62 | 0x61 | 0x60 | 0x5C | 0x59 | 0x57 | 0x47)
}

fn io_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> binrw::Error {
    binrw::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
}
