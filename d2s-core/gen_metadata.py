import os

file_path = r"d:\github\zhaopuming\diablo_edit\Generate Data\itemdata.txt"
with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
    lines = f.readlines()

header = lines[1].split()

def get_col(parts, name):
    try:
        idx = header.index(name)
        val = parts[idx].strip()
        if not val: return 0
        return int(val)
    except:
        return 0

rust_code = """
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct ItemMeta {
    pub simple: bool,
    pub def: bool,
    pub dur: bool,
    pub stacked: bool,
    pub charm: bool,
    pub monster: bool,
    pub spell: u8,
    pub ipad_bits: u16,
}

pub static ITEM_METADATA: OnceLock<HashMap<[u8; 4], ItemMeta>> = OnceLock::new();

pub fn get_item_meta(id: &[u8; 4]) -> Option<&'static ItemMeta> {
    ITEM_METADATA.get_or_init(init_metadata).get(id)
}

fn init_metadata() -> HashMap<[u8; 4], ItemMeta> {
    let mut m = HashMap::new();
"""

def parse_val(s):
    try:
        return int(s.strip())
    except ValueError:
        return 0

ipad_idx = header.index("iPadBits")
simple_idx = header.index("bSimple")
def_idx = header.index("bDef")
dur_idx = header.index("bDur")
stacked_idx = header.index("bStack")
charm_idx = header.index("bCharm")
monster_idx = header.index("bMonst")
spell_idx = header.index("iSpell")

for line in lines[2:]:
    parts = line.split("\t")
    if len(parts) < 2: continue
    
    item_id = parts[0].strip()
    if not item_id or item_id.startswith("*"): continue
    
    # ID must be ASCII and <= 4 chars
    if not all(ord(c) < 128 for c in item_id) or len(item_id) > 4:
        continue

    ipad = parse_val(parts[ipad_idx]) if ipad_idx < len(parts) else 0
    simple = parse_val(parts[simple_idx]) == 1 if simple_idx < len(parts) else False
    def_ = parse_val(parts[def_idx]) == 1 if def_idx < len(parts) else False
    dur = parse_val(parts[dur_idx]) == 1 if dur_idx < len(parts) else False
    stacked = parse_val(parts[stacked_idx]) == 1 if stacked_idx < len(parts) else False
    charm = parse_val(parts[charm_idx]) == 1 if charm_idx < len(parts) else False
    monster = parse_val(parts[monster_idx]) == 1 if monster_idx < len(parts) else False
    spell = parse_val(parts[spell_idx]) if spell_idx < len(parts) else 0

    rust_code += f"    m.insert(*b\"{item_id.ljust(4)}\", ItemMeta {{ simple: {str(simple).lower()}, def: {str(def_).lower()}, dur: {str(dur).lower()}, stacked: {str(stacked).lower()}, charm: {str(charm).lower()}, monster: {str(monster).lower()}, spell: {spell}, ipad_bits: {ipad} }});\n"

rust_code += """
    m
}
"""

with open(r"d:\github\zhaopuming\diablo_edit\d2s-core\src\metadata.rs", "w") as f:
    f.write(rust_code)
print("Generated metadata.rs")
