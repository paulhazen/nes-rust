mod common; 

#[test]
fn test_rom_loading() {
    let cartridge = common::load_test_rom("dk.nes");
    
    assert_eq!(cartridge.header.mapper_id, 0, "Mapper ID for Donkey Kong should be 0.");
    println!("TEST");
}

