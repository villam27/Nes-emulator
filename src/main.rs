use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use cpu::Cpu;
mod cpu;

//  https://www.nesdev.org/wiki/INES
//  https://www.nesdev.org/wiki/NES_2.0
pub enum Format {
    INES,
    NES2,
}

#[allow(dead_code)]
pub struct Rom {
    name: String,
    memory: Vec<u8>,
    format: Format,
    prg_size: u32,
    chr_size: u32,
    //  Store all other header memory ?
}

impl Rom {
    fn new(path: &String) -> Self {
        let my_buf: BufReader<File> = BufReader::new(File::open(path.clone()).unwrap());
        let file: &File = my_buf.get_ref();
        let mut memory: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize);
        for byte in file.bytes() {
            memory.push(byte.unwrap());
        }
        if memory[0] == b'N' && memory[1] == b'E' && memory[2] == b'S' && memory[3] == 0x1A {
            Rom {
                name: path.clone(),
                format: Format::INES,
                prg_size: memory[4] as u32 * 16384,
                chr_size: memory[5] as u32 * 8192,
                memory,
            }
        } else {
            panic!("Not a valid Nes file");
        }
    }

    #[allow(dead_code)]
    fn print_memory(&self, max: i32) {
        let mut size: i32 = 0;
        let mut m: i32 = 0;

        for byte in &self.memory {
            if size % 256 == 0 {
                println!("\n{:#06x}", size);
                m += 1;
            }
            if m >= max {
                break;
            }
            print!("{:b} ", byte);
            size += 1;
        }
        println!("\n{}", self.memory.len());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cpu: Cpu = cpu::Cpu::default();

    if args.len() != 2 {
        panic!("Please enter a rom path");
    }
    let rom: Rom = Rom::new(&args[1]);

    cpu.start(rom);
    //rom.print(10);
}
