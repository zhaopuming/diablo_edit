# Fixed Huffman tree from item.txt
# [[[[["w","u"],[["8",["y",["5",["j",[]]]]],"h"]],["s",[["2","n"],"x"]]],[[["c",["k","f"]],"b"],[["t","m"],["9","7"]]]],[" ",[[[["e","d"],"p"],["g",[[["z","q"],"3"],["v","6"]]]],[["r","l"],["a",[["1",["4","0"]],["i","o"]]]]]]]

class Node:
    def __init__(self, left=None, right=None, data=None):
        self.left = left
        self.right = right
        self.data = data

def construct(arr):
    if not arr: return None
    if isinstance(arr, list):
        if not arr: return None
        return Node(left=construct(arr[0]), right=construct(arr[1]))
    else:
        return Node(data=arr)

tree_data = [[[[["w","u"],[["8",["y",["5",["j",[]]]]],"h"]],["s",[["2","n"],"x"]]],[[["c",["k","f"]],"b"],[["t","m"],["9","7"]]]],[" ",[[[["e","d"],"p"],["g",[[["z","q"],"3"],["v","6"]]]],[["r","l"],["a",[["1",["4","0"]],["i","o"]]]]]]]
root = construct(tree_data)

def decode(bits):
    res = []
    node = root
    i = 0
    while len(res) < 4 and i < len(bits):
        bit = bits[i]
        i += 1
        if bit == '0':
            node = node.left
        else:
            node = node.right
        
        if node.data:
            res.append(node.data)
            node = root
    return "".join(res), i

# Raw data from Item 0 start (Byte 839)
# 10 00 A2 00 15 00 00 CF 4F 00 10 00 A2 00 15 00 00 CF 4F 00 ...
# I need more bytes!
raw_bytes = [
    0x10, 0x00, 0xA2, 0x00, 0x15, 0x00, 0x00, 0xCF, 
    0x4F, 0x00, 0x10, 0x00, 0xA2, 0x00, 0x15, 0x00, 
    0x00, 0xCF, 0x4F, 0x00, 0x10, 0x00, 0xA2, 0x00,
    0x15, 0x00, 0x00, 0xCF
]
all_bits = ""
for b in raw_bytes:
    all_bits += format(b, "08b")[::-1] # LSB first

# Try Item 0
code0, len0 = decode(all_bits[53:])
print(f"Item 0 (shift 53): {code0} (ends at {53+len0})")

# Assume Item 0 is 10 bytes (80 bits)
# Then Item 1 starts at 80
code1, len1 = decode(all_bits[80+53:])
print(f"Item 1 (shift 133): {code1} (ends at {133+len1})")

# Let's try every possible Item 0 length (multiple of 8 bits)
for length in [64, 80, 96, 112, 128]:
    code, _ = decode(all_bits[length+53:])
    print(f"If Item 0 is {length//8} bytes, Item 1 ID is: {code}")
