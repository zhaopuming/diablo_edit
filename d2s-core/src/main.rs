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
            println!("Class: {} (Amazon)", save.header.char_class);
            println!("Level: {}", save.header.char_level);
            
            println!("\nSTATS:");
            let stat_labels = [
                "Strength", "Energy", "Dexterity", "Vitality", "Stat Points", "Skill Points",
                "Life (cur)", "Life (max)", "Mana (cur)", "Mana (max)", "Stamina (cur)", "Stamina (max)",
                "Level", "Experience", "Gold (inv)", "Gold (stash)"
            ];
            for (id, val) in &save.stats.values {
                let label = if (*id as usize) < stat_labels.len() { stat_labels[*id as usize] } else { "Unknown" };
                println!("  {:<12}: {}", label, val);
            }

            println!("\nSKILLS (levels):");
            println!("  {:?}", save.skills.skills);

            println!("\nITEMS ({} total):", save.items.items.len());
            for (i, item) in save.items.items.iter().enumerate() {
                let type_id = String::from_utf8_lossy(&item.data.type_id);
                let loc = match item.data.container {
                    0 => "Inventory/Belt",
                    2 => "Equipped",
                    4 => "Cube",
                    5 => "Stash",
                    _ => "Other"
                };
                println!("  #{:<2} {:<5} | Loc: {:<14} | Pos: ({:>2}, {:>2}) | Simple: {:<5} | ID'd: {:<5}", 
                    i, type_id, loc, item.data.column, item.data.row, item.data.simple, item.data.identified);
            }
            
            println!("\nOTHER:");
            println!("  Corpse Count   : {}", save.corpse.corpses.len());
            println!("  Mercenary Magic: 0x{:04X}", save.mercenary.magic);
            println!("  Golem Status   : {}", save.golem.exists);
        },
        Err(e) => {
            eprintln!("PARSING FAILED: {}", e);
        }
    }
}
