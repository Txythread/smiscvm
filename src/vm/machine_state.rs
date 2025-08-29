use std::process::exit;
use std::io;
use std::io::{stdout, Write};
use colorize::AnsiColor;
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::cursor::{ position };
use crossterm::{cursor, execute};

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
    
    // Standard Output
    pub std_transmitter_contents: u8,
    pub stdout: String,

    // Information stored for pure interest
    pub completed_clock_cycles: usize,
    pub completed_instructions: usize,
}

#[derive(Clone, Copy)]
pub struct ScreenPrintingInfo {
    pub size: (u16, u16),
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState { registers: [0; 32], flag1: false, flag2: false, memory_address_ptr: 0, current_instruction: 0, micro_op_counter: 0, alu_arg_1: 0, alu_arg_2: 0, program_counter: 0, main_bus: 0, memory: [0; MEMORY_SIZE], std_transmitter_contents: 0, stdout: "\n\n\n\n".to_string(), completed_clock_cycles: 0, completed_instructions: 0 }
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



    /// Prints information about the machine to stdout. Redraws entire screen if needed. Always pass the same screen printing info.
    pub fn print(&self, clk_high: bool, main_bus_contents: u32, screen_info: &mut Option<ScreenPrintingInfo>) {
        // Redraw screen if needed (when no previous print by this function occurred (no screen printing info) or terminal size changed)
        let current_screen_size = size().unwrap();
        if screen_info.is_none() || screen_info.clone().unwrap().size != current_screen_size {
            _ = execute!(stdout(), Clear(ClearType::All));
        }


        execute!(
            stdout(),
            cursor::MoveTo(0, 0)
        ).unwrap();
        println!("Registers:");

        // Print the regular registers
        for i in 0..self.registers.len() {
            Self::print_register(i.to_string(), self.registers[i] as u32)
        }

        // Print the special registers
        Self::print_register("ma".to_string(), self.memory_address_ptr as u32);
        Self::print_register("in".to_string(), self.current_instruction);
        Self::print_register("pc".to_string(), self.program_counter);

        println!("\n");
        println!("ZF: {}, PM: {}, ALU: ({:#010X}, {:#010X})", if self.flag1 { "1" } else { "0" }, if self.flag2 { "1" } else { "0" }, self.alu_arg_1, self.alu_arg_2);


        let mut ipc = "N/A".to_string(); // Initialize with default value
        if self.completed_instructions != 0 {
            ipc = (self.completed_instructions as f32 / self.completed_clock_cycles as f32).to_string();
        }
        println!("\n");
        println!("Current:");
        println!("CLK: ({}), Step: {:#04X}, øIPC: {:.5}, mBus: {:#010X}", /* Clock: */ if clk_high { "*" } else { " "}, /* Step: */ self.micro_op_counter, /* øIPC: */ ipc, /* mBus: */main_bus_contents);

        println!("Standard Output (3 lines):\n");
        let standard_output_lines: Vec<_> = self.stdout.lines().collect();
        let lines_count = standard_output_lines.clone().len();
        let last_3_lines = &standard_output_lines[lines_count - 3 .. lines_count];
        println!("{}", last_3_lines.join("\n"));
        println!("--------- smiscvm - stdout ---------");


        *screen_info = Some(ScreenPrintingInfo { size: size().unwrap() }).clone();
    }

    fn print_register(name: String, contents: u32){
        const REGISTER_PRINT_LENGTH: u16 = 14;
        let output = format!("{}{}: {:#010X}  ", if name.len() > 1 {""} else {" "}, name, contents);

        print!("{}", output);

        // Flush the buffer
        io::stdout().flush().unwrap();

        // Look if a newline is required before the next register print
        let size = size();

        if size.is_err() {
            println!("{}", "Can't get size of terminal".to_string().red());
        }

        let columns = size.unwrap().0;

        let cursor_pos = position();

        if cursor_pos.is_err(){
            println!("{}", "Can't get cursor position".to_string().red());
        }

        let cursor_column = cursor_pos.unwrap().0;
        let cursor_column_after_next_reg_print = cursor_column + REGISTER_PRINT_LENGTH;


        // Print if it would be too long after the next register print
        if cursor_column_after_next_reg_print > columns{
            println!();
        }
    }
}