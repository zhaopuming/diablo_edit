
import os

def parse_itemdata(file_path):
    # Try different encodings
    encodings = ['utf-16', 'gb18030', 'utf-8', 'cp1252']
    content = None
    for enc in encodings:
        try:
            with open(file_path, 'r', encoding=enc) as f:
                content = f.read()
            print(f"Opened with {enc}")
            break
        except Exception:
            continue
    
    if content is None:
        raise Exception("Could not open file with any encoding")
    
    lines = content.splitlines()
    
    items = []
    for line in lines:
        line = line.strip()
        if not line or line.startswith('*'):
            continue
        
        parts = line.split('\t')
        # Parts mapping (based on C++ code):
        # 0: ID
        # 1: Pic
        # 2: NameIndex
        # 3: Range
        # 4: Equip
        # 5: Simple
        # 6: Normal
        # 7: White
        # 8: IsNew
        # 9: HasDef
        # 10: HasDur
        # 11: IsStacked
        # 12: HasMonsterID
        # 13: IsCharm
        # 14: SpellId
        # 15: IsUnique
        # 16: IsCraft
        # 17: IsGem
        # 18: Damage1Min
        # 19: Damage1Max
        # 20: Damage2Min
        # 21: Damage2Max
        # 22: iPadBits
        # 23: iPad
        
        # Ensure we have enough parts
        while len(parts) < 24:
            parts.append('')
            
        def get_bool(idx):
            val = parts[idx].strip()
            return 'true' if val == '1' else 'false'
            
        def get_int(idx, default=0):
            val = parts[idx].strip()
            return int(val) if val else default

        item_id = parts[0].strip()
        if len(item_id) < 3:
            continue
        if len(item_id) == 3:
            item_id += ' '
        
        range_val = get_int(3)
        if range_val == 0:
            width, height = 1, 1
        else:
            width = range_val // 10
            height = range_val % 10
            
        items.append({
            'id': item_id,
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

#[derive(Debug, Clone)]
pub struct ItemMeta {
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
        # Escape quotes if any in ID (unlikely for D2 IDs)
        id_bytes = f"b\"{item['id']}\""
        rust += f"    m.insert(*{id_bytes}, ItemMeta {{ simple: {item['simple']}, def: {item['def']}, dur: {item['dur']}, stacked: {item['stacked']}, charm: {item['charm']}, monster: {item['monster']}, spell: {item['spell']}, ipad_bits: {item['ipad_bits']}, width: {item['width']}, height: {item['height']} }});\n"
    
    rust += "    m\n}\n"
    return rust

if __name__ == "__main__":
    items = parse_itemdata("Generate Data/itemdata.txt")
    rust_code = generate_rust(items)
    with open("d2s-core/src/metadata.rs", "w", encoding="utf-8") as f:
        f.write(rust_code)
    print(f"Generated {len(items)} items.")
