use std::process::exit;
use colorize::AnsiColor;

pub const MEMORY_PAGE_SIZE: usize = 4096;
pub const MEMORY_SIZE: usize = MEMORY_PAGE_SIZE * 16; // The sixteen pages are chosen randomly, but that'd be 64kiB or 65536 bytes (if I remember correctly)


pub struct MachineState {
    // Registers
    pub registers: [i32; 32],

    // Hidden regs
    pub flag1: bool,                // ZF
    pub flag2: bool,                // PM
    pub memory_address_ptr: i32,
    pub current_instruction: u32,
    pub micro_op_counter: u8,
    pub alu_arg_1: i32,
    pub alu_arg_2: i32,
    pub program_counter: u32,

    // Bus
    pub main_bus: i32,

    // Memory
    pub memory: [u8; MEMORY_SIZE],
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState { registers: [0; 32], flag1: false, flag2: false, memory_address_ptr: 0, current_instruction: 0, micro_op_counter: 0, alu_arg_1: 0, alu_arg_2: 0, program_counter: 0, main_bus: 0, memory: [0; MEMORY_SIZE] }
    }


    /// Push data to the main bus while simulating overflow
    pub fn push_to_main_bus(&mut self, value: u32) {
        // In reality, the 0 would win,
        // in this simulated case, the one will.
        // However, the approximation is close enough for my purposes.
        // If you can even call that an "approximation".
        self.main_bus |= value as i32;
    }

    /// Put an array of bytes into memory at an offset
    pub fn push_to_memory(&mut self, data: Vec<u8>, start_address: usize) {
        if start_address + data.len() > MEMORY_SIZE {
            let msg = format!("Can't append vector of length {} into memory at {}. The position of some bytes exceeds the memory size ({}B).", data.len(), start_address, MEMORY_SIZE).red();
            eprintln!("{}", msg);
            exit(199);
        }

        for i in data.iter().enumerate() {
            let mem_address = start_address + i.0;
            let data = *i.1;

            self.memory[mem_address] = data;
        }
    }
}