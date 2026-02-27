use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::BTreeMap;

fn main() {
    let input = File::open("d:\\github\\zhaopuming\\diablo_edit\\Generate Data\\property.txt").unwrap();
    let reader = BufReader::new(input);
    let mut props = BTreeMap::new();

    for (i, line) in reader.lines().enumerate() {
        if i < 2 { continue; }
        let line = line.unwrap();
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.is_empty() { continue; }
        
        let id_str = fields[0].trim();
        if id_str.is_empty() { continue; }
        let id: u16 = match id_str.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let mut bits = 0;
        let indices = [3, 7, 11, 15];
        for &idx in &indices {
            if fields.len() > idx {
                if let Ok(b) = fields[idx].trim().parse::<u16>() {
                    bits += b;
                }
            }
        }
        // Always take the latest/largest (handles version overrides)
        let entry = props.entry(id).or_insert(0);
        if bits > *entry { *entry = bits; }
    }

    let mut output = File::create("d:\\github\\zhaopuming\\diablo_edit\\d2s-core\\src\\properties.rs").unwrap();
    writeln!(output, "pub fn get_property_bits(id: u16) -> u16 {{").unwrap();
    writeln!(output, "    match id {{").unwrap();
    for (id, bits) in props {
        writeln!(output, "        {} => {},", id, bits).unwrap();
    }
    writeln!(output, "        _ => 0,").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}").unwrap();
}
