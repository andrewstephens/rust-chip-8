use std::env;
use std::fs;


#[derive(Debug)]
struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [u16; 16],
    sp: usize,
    index: usize,
    pc: usize,
    delay_timer: u8,
    sound_timer: u8,
    video: [u32; 64 * 32],
    opcode: u16,
    keypad: [u8; 16]
}

impl Chip8 {
    fn load_rom(&mut self, file_path: &String) {
        println!("{:?}", file_path);

        // Read file into chip-8
        let file_as_bytes = fs::read(file_path).expect("File couldn't be read.");

        let chip8_program_starting_address = 0x200;

        for (idx, i) in file_as_bytes.iter().enumerate() {
            self.memory[chip8_program_starting_address + idx] = *i;
        }
    }

    fn cycle(&mut self) -> u16 {
        let opcode_first_piece = (self.memory[self.pc] as u16) << 8;
        let opcode_second_piece = self.memory[self.pc + 1] as u16;
        let opcode = opcode_first_piece | opcode_second_piece;
        return opcode;
    }

    // Clear the display -- OP Code 00E0 -- CLS
    fn op_00e0(&mut self) {
        self.video = [0; 64 * 32];
    }
}

fn get_fontset() -> [u8; 80] {
    let fontset: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    return fontset;
}

fn load_fontset(chip8: &mut Chip8) -> &Chip8 {
    let fontset = get_fontset();
    let font_memory_start_point = 0x50;

    for (idx, i) in fontset.iter().enumerate() {
        chip8.memory[font_memory_start_point + idx] = *i;
    }

    return chip8;
}

fn build_chip8() -> Chip8 {
    Chip8 {
        memory: [0; 4096],
        registers: [0; 16],
        stack: [0; 16],
        sp: 0,
        index: 0,
        pc: 0x200,
        delay_timer: 0,
        sound_timer: 0,
        video: [0; 64 * 32],
        opcode: 0,
        keypad: [0; 16]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut chip_8 = build_chip8();

    load_fontset(&mut chip_8);

    chip_8.load_rom(file_path);

    let opcode = chip_8.cycle();

    println!("{:#06x}", opcode);
}
