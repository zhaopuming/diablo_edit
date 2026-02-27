import re

with open(r"d:\github\zhaopuming\diablo_edit\Diablo Edit2\D2Item.cpp", "r") as f:
    content = f.read()

match = re.search(r"const BYTE HUFFMAN\[\] = \{(.*?)\};", content, re.DOTALL)
if match:
    data_str = match.group(1)
    # The array seems to have 0x01, 0x00, and characters in quotes.
    # Let's parse it very carefully.
    parts = data_str.split(',')
    bytes_list = []
    for p in parts:
        p = p.strip()
        if not p: continue
        if p.startswith("'"):
            # It's a character
            c = p.strip("'")
            if c == r"\t": bytes_list.append(9)
            elif c == r"\n": bytes_list.append(10)
            elif c == r" ": bytes_list.append(32)
            else: bytes_list.append(ord(c[0]))
        else:
            # It's a number
            bytes_list.append(int(p, 0))
    print(bytes_list)
else:
    print("Not found")
