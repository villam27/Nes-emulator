use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
mod cpu;

#[allow(dead_code)]
fn print_data(my_buf: BufReader<File>) {
    let mut size = 0;
    for byte in my_buf.bytes() {
        if size % 256 == 0 {
            println!("\n{:#06x}", size)
        }
        let b: u8 = byte.unwrap();
        print!("{:b} ", b);
        size += 1;
    }
    println!("\n{}", size);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please enter a rom path");
    }
    let my_buf: BufReader<File> = BufReader::new(File::open(&args[1]).unwrap());
    print_data(my_buf);
    println!("Nes emulator");
}
