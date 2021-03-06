use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub struct Rom {
    bytes: Box<[u8]>
}

impl Rom {

    pub fn new_from_bin<P: AsRef<Path>>(path: P) -> io::Result<Rom> {
        //read provided file
        let mut file = fs::File::open(path)?;
        let mut file_buff = Vec::new();

        file.read_to_end(&mut file_buff)?;
        // TODO: Make sure the rom is valid
        Ok(Rom { bytes: file_buff.into_boxed_slice() })

    }
    pub fn size (&self) -> u32 {
        self.bytes.len() as _
    }
    
    pub fn game_title (&self) -> String {
        String::from_utf8_lossy(&self.bytes[0x0134..0x0142]).into_owned()
    }

    pub fn is_super_gameboy (&self) -> bool {
        self.bytes[0x0146] == 0x03
    }

    pub fn cardrige_type (&self) -> u8 {
        self.bytes[0x0147]
    }

    pub fn start_bytes_ok (&self) -> bool {
        let checked_bytes: &[u8] = &self.bytes[0x0134..0x014d];
        let sum: u32 = checked_bytes.iter()
            .map(|&v| v as u32)
            .sum();
        sum % 10 != 0
    }
}
