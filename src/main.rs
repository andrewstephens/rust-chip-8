use std::env;

#[derive(Debug)]
struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [u16; 16],
    sp: u8,
    index: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    video: [u32; 64 * 32],
    opcode: u16,
    keypad: [u8; 16]
}

impl Chip8 {
    fn load_rom(&self, file_path: &String) {
        println!("{:?}", file_path);
    }
}

fn build_chip8() -> Chip8 {
    Chip8 {
        memory: [0; 4096],
        registers: [0; 16],
        stack: [0; 16],
        sp: 0,
        index: 0,
        pc: 0,
        delay_timer: 0,
        sound_timer: 0,
        video: [0; 64 * 32],
        opcode: 0,
        keypad: [0; 16]
    }
}

fn main() {
    println!("Welcome to Chip-8?");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let chip_8 = build_chip8();
    chip_8.load_rom(file_path);
}
