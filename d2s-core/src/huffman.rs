use bitstream_io::{BitRead, BitReader, BitWrite, LittleEndian};
use std::collections::HashMap;
use std::io;
use std::sync::OnceLock;

const HUFFMAN_DATA: &[u8] = &[
    1, 1, 1, 1, 1, 119, 0, 0, 117, 0, 0, 1, 1, 56, 0, 0, 1, 121, 0, 0, 1, 53, 0, 0, 1, 106, 0, 0, 1, 0, 0, 
    104, 0, 0, 1, 115, 0, 0, 1, 1, 50, 0, 0, 110, 0, 0, 120, 0, 0, 1, 1, 1, 99, 0, 0, 1, 107, 0, 0, 102, 0, 0, 
    98, 0, 0, 1, 1, 116, 0, 0, 109, 0, 0, 1, 57, 0, 0, 55, 0, 0, 1, 32, 0, 0, 1, 1, 1, 1, 101, 0, 0, 100, 0, 0, 
    112, 0, 0, 1, 103, 0, 0, 1, 1, 1, 122, 0, 0, 113, 0, 0, 51, 0, 0, 1, 118, 0, 0, 54, 0, 0, 1, 1, 114, 0, 0, 
    108, 0, 0, 1, 97, 0, 0, 1, 1, 49, 0, 0, 1, 52, 0, 0, 48, 0, 0, 1, 105, 0, 0, 111, 0
];

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: u8,
}

pub struct HuffmanTree {
    root: Option<Box<Node>>,
    encode_map: HashMap<u8, Vec<bool>>,
}

impl HuffmanTree {
    pub fn new() -> Self {
        let mut index = 0;
        let root = Self::construct(&mut index);
        let mut encode_map = HashMap::new();
        if let Some(ref r) = root {
            Self::build_encode_map(r, &mut Vec::new(), &mut encode_map);
        }
        HuffmanTree { root, encode_map }
    }

    fn build_encode_map(node: &Node, path: &mut Vec<bool>, map: &mut HashMap<u8, Vec<bool>>) {
        if node.data != 0 {
            map.insert(node.data, path.clone());
            return;
        }
        if let Some(ref left) = node.left {
            path.push(false);
            Self::build_encode_map(left, path, map);
            path.pop();
        }
        if let Some(ref right) = node.right {
            path.push(true);
            Self::build_encode_map(right, path, map);
            path.pop();
        }
    }

    fn construct(index: &mut usize) -> Option<Box<Node>> {
        if *index >= HUFFMAN_DATA.len() {
            return None;
        }
        let c = HUFFMAN_DATA[*index];
        *index += 1;
        
        if c == 0 {
            return None;
        }
        
        let mut node = Box::new(Node {
            left: None,
            right: None,
            data: if c > 1 { c } else { 0 },
        });
        
        // C++: node->left = construct(index); node->right = construct(index);
        node.left = Self::construct(index);
        node.right = Self::construct(index);
        
        Some(node)
    }

    pub fn read_char<R: io::Read>(&self, bit_reader: &mut BitReader<R, LittleEndian>) -> io::Result<u8> {
        let mut node = self.root.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Empty Huffman tree"))?;
        let mut bits = String::new();
        loop {
            if node.data != 0 {
                // println!("    Huffman Decoded '{}' from bits: {}", node.data as char, bits);
                return Ok(node.data);
            }
            let b = bit_reader.read_bit()?;
            bits.push(if b { '1' } else { '0' });
            if b {
                node = node.right.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid path"))?;
            } else {
                node = node.left.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid path"))?;
            }
        }
    }

    pub fn write_char<W: io::Write>(&self, bit_writer: &mut bitstream_io::BitWriter<W, LittleEndian>, c: u8) -> io::Result<()> {
        let bits = self.encode_map.get(&c)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, format!("Character '{}' not in Huffman tree", c as char)))?;
        for &bit in bits {
            bit_writer.write_bit(bit)?;
        }
        Ok(())
    }
}

pub static D2R_HUFFMAN: OnceLock<HuffmanTree> = OnceLock::new();

pub fn get_huffman() -> &'static HuffmanTree {
    D2R_HUFFMAN.get_or_init(|| HuffmanTree::new())
}
