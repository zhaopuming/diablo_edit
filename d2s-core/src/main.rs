use d2s_core::parse_d2s;
use std::fs;

fn main() {
    println!("D2S Full Parser Test");
    let path = "tests/fixtures/test_char.d2s";
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };
    
    println!("File size: {} bytes", bytes.len());
    
    if bytes.len() > 835 {
        let start = 835;
        let end = (835 + 128).min(bytes.len());
        println!("Hex dump at {}: {:02X?}", start, &bytes[start..end]);
        for i in 0..((end - start + 7) / 8) {
            let row_start = start + i * 8;
            let row_end = (row_start + 8).min(end);
            print!("{:04X} | ", row_start);
            for b in &bytes[row_start..row_end] {
                print!("{:08b} ", b);
            }
            println!();
        }
    }
    
    match parse_d2s(&bytes) {
        Ok(save) => {
            println!("SUCCESS!");
            let name = String::from_utf8_lossy(&save.header.name)
                .trim_matches(char::from(0))
                .to_string();
            println!("Name: '{}'", name);
            println!("Level: {}", save.header.char_level);
            
            println!("Items (Character): {}", save.items.items.len());
            for (i, item) in save.items.items.iter().enumerate() {
                let type_name = String::from_utf8_lossy(&item.data.type_id);
                println!("  Item {}: ID '{}', Pos({}, {}), Simple: {}", 
                    i, type_name, item.data.column, item.data.row, item.data.simple);
            }
            
            println!("Corpse Count: {}", save.corpse.count);
        },
        Err(e) => {
            eprintln!("PARSING FAILED: {}", e);
        }
    }
}
