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

for line in lines[2:]:
    parts = line.split("\t")
    if len(parts) < 2: continue
    item_id = parts[0].strip()
    ipad = get_col(parts, "iPadBits")
    if ipad > 0:
        print(f"ID: {item_id:4} iPadBits: {ipad}")
