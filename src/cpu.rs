use crate::Rom;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Cpu {
    pc: u16,    //  Program Counter
    sp: u8,     //  Stack Pointer
    ac: u8,     //  Accumulator
    rx: u8,     //  Register x
    ry: u8,     //  Register y
    status: u8, //  Processor Status

    memory: [u8; 0xFFFF], //  https://www.nesdev.org/wiki/CPU_memory_map
}

impl Cpu {
    pub fn default() -> Self {
        let memory: [u8; 0xFFFF] = [0; 0xFFFF];
        Cpu {
            pc: 0,
            sp: 0xFF,
            ac: 0,
            rx: 0,
            ry: 0,
            status: 0,
            memory,
        }
    }

    #[allow(dead_code)]
    pub fn start(self, rom: Rom) {
        //	Start the cpu
        println!("Start Cpu");
        rom.print_memory(0);
        //loop {
        //    //	Main loop
        //    println!("loop {}", rom.name);
        //}
    }
}
