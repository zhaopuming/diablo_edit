
import os

def parse_language(file_path):
    # Try different encodings
    encodings = ['gb18030', 'utf-16', 'utf-8', 'cp1252']
    content = None
    for enc in encodings:
        try:
            with open(file_path, 'r', encoding=enc) as f:
                content = f.read()
            print(f"Language file opened with {enc}")
            break
        except Exception:
            continue
    
    if content is None:
        raise Exception("Could not open language file")
    
    lines = content.splitlines()
    names = []
    in_section = False
    for line in lines:
        if line.startswith('*=======================以下是物品名称'):
            in_section = True
            continue
        if in_section:
            if line.startswith('*'):
                break
            parts = line.split('\t')
            # English is Usually at index 1 or 2 depending on the file
            # Based on preview, it seems many items have English at index 1
            name = ""
            for p in parts:
                p = p.strip()
                if p:
                    name = p
                    break
            names.append(name)
    return names

def parse_itemdata(file_path, lang_names):
    # Try different encodings
    encodings = ['gb18030', 'utf-16', 'utf-8', 'cp1252']
    content = None
    for enc in encodings:
        try:
            with open(file_path, 'r', encoding=enc) as f:
                content = f.read()
            print(f"Itemdata file opened with {enc}")
            break
        except Exception:
            continue
    
    if content is None:
        raise Exception("Could not open itemdata file")
    
    lines = content.splitlines()
    
    items = []
    item_counter = 0
    for line in lines:
        line = line.strip()
        if not line or line.startswith('*'):
            continue
        
        parts = line.split('\t')
        def get_int(idx, default=0):
            if idx >= len(parts): return default
            val = parts[idx].strip()
            return int(val) if val else default

        def get_bool(idx):
            if idx >= len(parts): return "false"
            val = parts[idx].strip()
            return 'true' if val == '1' else 'false'

        item_id = parts[0].strip()
        if len(item_id) < 3:
            continue
        if len(item_id) == 3:
            item_id += ' '
        
        name_index = get_int(2, -1)
        if name_index == -1:
            name_index = item_counter
        
        name = "Unknown"
        if name_index < len(lang_names):
            name = lang_names[name_index]
        
        # Advance counter for items that don't have explicit index
        # Actually the C++ code uses idMap.size() which is equivalent to item_counter
        item_counter += 1

        range_val = get_int(3)
        if range_val == 0:
            width, height = 1, 1
        else:
            width = range_val // 10
            height = range_val % 10
            
        items.append({
            'id': item_id,
            'name': name,
            'simple': get_bool(5),
            'def': get_bool(9),
            'dur': get_bool(10),
            'stacked': get_bool(11),
            'charm': get_bool(13),
            'monster': get_bool(12),
            'spell': get_int(14),
            'ipad_bits': get_int(22),
            'width': width,
            'height': height
        })
    return items

def generate_rust(items):
    rust = """
use std::collections::HashMap;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMeta {
    pub name: &'static str,
    pub simple: bool,
    pub def: bool,
    pub dur: bool,
    pub stacked: bool,
    pub charm: bool,
    pub monster: bool,
    pub spell: u8,
    pub ipad_bits: u16,
    pub width: u8,
    pub height: u8,
}

pub static ITEM_METADATA: OnceLock<HashMap<[u8; 4], ItemMeta>> = OnceLock::new();

pub fn get_item_meta(id: &[u8; 4]) -> Option<&'static ItemMeta> {
    ITEM_METADATA.get_or_init(init_metadata).get(id)
}

fn init_metadata() -> HashMap<[u8; 4], ItemMeta> {
    let mut m = HashMap::new();
"""
    for item in items:
        id_bytes = f"b\"{item['id']}\""
        name = item['name'].replace('"', '\\"')
        rust += f"    m.insert(*{id_bytes}, ItemMeta {{ name: \"{name}\", simple: {item['simple']}, def: {item['def']}, dur: {item['dur']}, stacked: {item['stacked']}, charm: {item['charm']}, monster: {item['monster']}, spell: {item['spell']}, ipad_bits: {item['ipad_bits']}, width: {item['width']}, height: {item['height']} }});\n"
    
    rust += "    m\n}\n"
    return rust

if __name__ == "__main__":
    lang_names = parse_language("Generate Data/language.txt")
    print(f"Parsed {len(lang_names)} names.")
    items = parse_itemdata("Generate Data/itemdata.txt", lang_names)
    rust_code = generate_rust(items)
    with open("d2s-core/src/metadata.rs", "w", encoding="utf-8") as f:
        f.write(rust_code)
    print(f"Generated {len(items)} items.")
