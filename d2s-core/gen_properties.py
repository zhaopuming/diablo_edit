import os

def parse_val(s):
    s = s.strip()
    if not s: return 0
    if s.startswith("0x"):
        return int(s, 16)
    return int(s)

def run():
    file_path = r"d:\github\zhaopuming\diablo_edit\Generate Data\property.txt"
    if not os.path.exists(file_path):
        print("File not found")
        return

    with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
        lines = f.readlines()

    # Header is line 1 (index 0 is *PROP, index 1 is header)
    header = lines[1].split()
    
    # Indices for bits
    arg1_idx = header.index("Arg1Bits")
    arg2_idx = header.index("Arg2Bits")
    arg3_idx = header.index("Arg3Bits")
    arg4_idx = header.index("Arg4Bits")
    ver_idx = header.index("VersionMin")

    props = {} # (id, version) -> bits

    for line in lines[2:]:
        parts = line.split("\t")
        if len(parts) < 2: continue
        try:
            prop_id = parse_val(parts[0])
            version = parse_val(parts[ver_idx]) if ver_idx < len(parts) else 0
            
            bits = 0
            bits += parse_val(parts[arg1_idx]) if arg1_idx < len(parts) else 0
            bits += parse_val(parts[arg2_idx]) if arg2_idx < len(parts) else 0
            bits += parse_val(parts[arg3_idx]) if arg3_idx < len(parts) else 0
            bits += parse_val(parts[arg4_idx]) if arg4_idx < len(parts) else 0
            
            if prop_id not in props:
                props[prop_id] = []
            props[prop_id].append((version, bits))
        except:
            continue

    with open(r"d:\github\zhaopuming\diablo_edit\d2s-core\src\properties.rs", "w", encoding="utf-8") as f:
        f.write("pub fn get_property_bits(id: u16, version: u32) -> u32 {\n")
        f.write("    match id {\n")
        
        # Sort by id
        for pid in sorted(props.keys()):
            versions = props[pid]
            if len(versions) == 1 and versions[0][0] == 0:
                f.write(f"        {pid} => {versions[0][1]},\n")
            else:
                # Handle versions. Sort descending by version to match highest first.
                versions.sort(key=lambda x: x[0], reverse=True)
                f.write(f"        {pid} => {{\n")
                found_default = False
                for ver, bits in versions:
                    if ver > 0:
                        f.write(f"            if version >= {hex(ver)} {{ return {bits}; }}\n")
                    else:
                        f.write(f"            {bits}\n")
                        found_default = True
                if not found_default:
                    f.write("            0\n")
                f.write("        }\n")
        
        f.write("        _ => 0,\n")
        f.write("    }\n")
        f.write("}\n")

if __name__ == "__main__":
    run()
