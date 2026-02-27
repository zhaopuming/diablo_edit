use std::fs;
use std::path::Path;
use d2s_core::parse_d2s;

#[test]
fn test_parse_real_save_file() {
    println!("DEBUG: STARTING TEST");
    let save_path = Path::new("tests/fixtures/test_char.d2s");
    
    if !save_path.exists() {
        panic!("Test aborted: File NOT FOUND at {:?}", save_path);
    }

    let bytes = fs::read(save_path).expect("Failed to read save file");
    println!("DEBUG: File size is {} bytes", bytes.len());
    
    if bytes.len() >= 4 {
        let magic = &bytes[0..4];
        println!("DEBUG: First 4 bytes (Magic): {:02X?}", magic);
    }
    
    match parse_d2s(&bytes) {
        Ok(save) => {
            println!("Successfully parsed save file!");
            println!("Character Name: {:?}", save.header.name);
            println!("Level: {}", save.header.char_level);
            
            // Assert magic numbers are correct
            assert_eq!(save.header.magic, 0xAA55AA55);
        },
        Err(e) => {
            eprintln!("PARSING FAILED WITH ERROR: {}", e);
            panic!("Failed to parse save file: {:?}", e);
        }
    }
}
