import re

with open(r"d:\github\zhaopuming\diablo_edit\Diablo Edit2\D2Item.cpp", "r") as f:
    content = f.read()

match = re.search(r"const BYTE HUFFMAN\[\] = \{(.*?)\};", content, re.DOTALL)
if match:
    data_str = match.group(1)
    # Replace ' ' with ord(' ')
    data_str = data_str.replace("' '", "32")
    # Replace other chars
    data_str = re.sub(r"'(\w)'", lambda m: str(ord(m.group(1))), data_str)
    # Parse numbers
    bytes_list = []
    for x in data_str.split(','):
        x = x.strip()
        if x:
            bytes_list.append(int(x, 0))
    print(bytes_list)
else:
    print("Not found")
