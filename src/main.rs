mod utils;
mod rom;
use std::env;

use rom::Rom;
use utils::format_size;


fn main() {
    let mut args = env::args();
    let filename = args.nth(1).unwrap();
    println!("\nFilename: {}", filename);
    // Initialize Rom with the stream of bytes from arg
    let rom = Rom::new_from_bin(&filename).unwrap();

    println!("ROM size: {}", format_size(rom.size()));
    println!("ROM name: {}", rom.game_title());
    println!("Super GameBoy: {}", rom.is_super_gameboy());
    println!("Cardrige type: {:02X}", rom.cardrige_type());
    println!("Start bytes ok: {}", rom.start_bytes_ok());
}

struct Cpu {
    PC: u16,

}

impl Cpu {
    pub fn new () -> Cpu {
        Cpu {
            PC: 0
        }
    }
}
