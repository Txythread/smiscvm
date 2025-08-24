use std::collections::HashMap;
use rand::random;
use crate::instruction::instruction::{ OUTPUT_MAP_STRING, get_generated_instructions };
use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;
use crate::vm::peripherals::immediate_out_peripheral::ImmediateOutPeripheral;
use crate::vm::peripherals::mem_out_peripheral::MemOutPeripheral;
use crate::vm::peripherals::pc_inc_peripheral::PCIncPeripheral;
use crate::vm::peripherals::plus_peripheral::PlusPeripheral;
use crate::vm::peripherals::reset_micro_peripheral::ResetMicroPeripheral;

pub struct Machine {
    pub peripherals: Vec<Box<dyn Peripheral>>,
    pub state: MachineState,
    pub instructions: HashMap<u16, Vec<u8>>, /* TODO: is pub necessary */
}

impl Machine {
    pub fn new() -> Self {
        Machine{ peripherals: vec![], state: MachineState::new(), instructions: get_generated_instructions() }
    }

    pub fn set_up_peripherals(&mut self) {
        // Create the peripherals first
        let immediate_out_peripheral = ImmediateOutPeripheral {};
        let mem_out_peripheral = MemOutPeripheral {};
        let pc_inc_peripheral = PCIncPeripheral {};
        let plus_peripheral = PlusPeripheral {};
        let reset_micro_peripheral = ResetMicroPeripheral {};

        // Then add them to the list of peripherals
        self.peripherals.push(Box::new(immediate_out_peripheral));
        self.peripherals.push(Box::new(mem_out_peripheral));
        self.peripherals.push(Box::new(pc_inc_peripheral));
        self.peripherals.push(Box::new(plus_peripheral));
        self.peripherals.push(Box::new(reset_micro_peripheral));
    }

    pub fn simulate_clock_pulse(&mut self) {
        // Generate the OP-Code
        // First, get the instruction
        // Result: 0x3F_C0
        let mut op_code = ((self.state.current_instruction & 0xFF_80_00_00) >> 18) as u16;

        // Might as well generate more necessary information for later while we're at it
        // get the cal(led)_reg(isters)
        let cal_reg_a = ((self.state.current_instruction & 0x00_7C_00_00) >> 18) as u8;
        let cal_reg_b = ((self.state.current_instruction & 0x00_03_E0_00) >> 13) as u8;
        let immediate_value = (self.state.current_instruction & 0x00_00_1F_FF) as i32;

        // Then, add the flags
        op_code |= (self.state.flag1 as u16) << 14;
        op_code |= (self.state.flag2 as u16) << 15;

        // Last, add the current step
        op_code |= (self.state.micro_op_counter as u16) & 0x00_1Fu16;

        let control_indexes = self.instructions.get(&op_code);

        if control_indexes.is_none() { return }

        self.execute_control_indexes(control_indexes.unwrap().clone(), cal_reg_a, cal_reg_b, immediate_value);

        // End of clock pulse - end of outputs
        self.state.main_bus = 0;
        self.state.micro_op_counter += 1;
    }

    pub fn execute_control_indexes(&mut self, indexes: Vec<u8>, first_called_reg: u8, second_called_reg: u8, immediate_value: i32) {
        let mut indexes = indexes;


        // Reverse the list sometimes to ensure the instructions actually work in practice
        if random() {
            indexes.reverse();
        }

        // Early calls first
        for idx in indexes.clone(){
            let name = OUTPUT_MAP_STRING[idx as usize].to_string().clone();

            self.register_calls(name.clone(), first_called_reg, second_called_reg, immediate_value);
            for i in 0..self.peripherals.len() {
                self.peripherals[i].call(name.clone(), &mut self.state);
            }
        }

        // Simulate late clock pulse now
        for idx in indexes{
            let name = OUTPUT_MAP_STRING[idx as usize].to_string().clone();
            self.late_register_calls(name.clone(), first_called_reg, second_called_reg);
            for i in 0..self.peripherals.len(){
                self.peripherals[i].late_call(name.clone(), &mut self.state)
            }
        }

    }

    /// Handles the direct calls to registers that take effect once CLK pulses. Complex behaviour is for peripherals to handle.
    fn register_calls(&mut self, name: String, cal_reg_a: u8, cal_reg_b: u8, immediate_value: i32) {
        match name.as_str(){
            "CAL_REG_A_OUT" => self.cal_reg_out(cal_reg_a),
            "CAL_REG_B_OUT" => self.cal_reg_out(cal_reg_b),
            "IMMEDIATE_OUT" => self.immediate_value_out(immediate_value),
            "PC_OUT" => self.state.push_to_main_bus(self.state.program_counter),
            _ => {}
        }
    }

    /// Handles the direct calls to registers that take effect once LCLK pulses
    fn late_register_calls(&mut self, name: String, cal_reg_a: u8, cal_reg_b: u8) {
        match name.as_str(){
            "CAL_REG_A_IN" => self.cal_reg_in(cal_reg_a),
            "CAL_REG_B_IN" => self.cal_reg_in(cal_reg_b),
            "ALU_IN_A" => self.alu_a_in(),
            "ALU_IN_B" => self.alu_b_in(),
            "PC_IN" => self.state.program_counter = self.state.main_bus as u32,
            "INSTR_IN" => self.state.current_instruction = self.state.main_bus as u32,
            "MEM_ADDR_PTR_IN" => self.state.memory_address_ptr = self.state.main_bus,
            _ => {}
        }
    }
    fn cal_reg_out(&mut self, cal_reg: u8){
        self.state.main_bus = self.state.registers[cal_reg as usize];
    }

    fn cal_reg_in(&mut self, cal_reg: u8){
        self.state.registers[cal_reg as usize] = self.state.main_bus;
    }

    fn alu_a_in(&mut self){
        self.state.alu_arg_1 = self.state.main_bus;
    }

    fn alu_b_in(&mut self){
        self.state.alu_arg_2 = self.state.main_bus;
    }

    fn immediate_value_out(&mut self, immediate_value: i32){
        self.state.main_bus = immediate_value;
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::machine::Machine;

    #[test]
    fn test_execute_control_indexes(){
        let mut machine = Machine::new();

        machine.state.registers[0] = 10;

        machine.execute_control_indexes(vec![8, 7], 0, 1, 0);

        assert_eq!(machine.state.registers[1], 10);
    }

    #[test]
    fn test_clock_pulse(){
        let mut machine = Machine::new();

        // Add the peripherals
        machine.set_up_peripherals();

        // Set the instruction to "add x0, 1"
        machine.state.current_instruction = 0x50_00_00_01;

        // Load the third (or fourth, the step with the index 0 will always be zero) step
        machine.state.micro_op_counter = 3;

        // This setup should then execute ALU_IN_A & IMMEDIATE_OUT

        machine.simulate_clock_pulse();

        // After execution, the alu_arg_1 should have the immediate value's value, which is 1.
        assert_eq!(machine.state.alu_arg_1, 1);


        // Also test state #5, where the addition occurs
        machine.state.micro_op_counter = 5;

        machine.simulate_clock_pulse();

        // Theoretically, x0 (in reg) should be 1 (immediate value) + 0 (empty register)
        assert_eq!(machine.state.registers[0], 1);

        // And the op code counter should've been reset
        assert_eq!(machine.state.micro_op_counter, 1);
    }

    #[test]
    fn test_add_instruction(){
        let mut machine = Machine::new();

        // Add the peripherals
        machine.set_up_peripherals();

        // Set the instruction to "add x0, 1" & add the instruction to mem addr 0x0
        // so that after the machine catches the next instruction in the beginning,
        // it'll still execute the correct one
        let instruction = 0x50_00_00_01;
        machine.state.current_instruction = instruction;
        machine.state.memory[0] = ((instruction & 0xFF_00_00_00) >> 24) as u8;
        machine.state.memory[1] = ((instruction & 0x00_FF_00_00) >> 16) as u8;
        machine.state.memory[2] = ((instruction & 0x00_00_FF_00) >> 8) as u8;
        machine.state.memory[3] = (instruction & 0x00_00_00_FF) as u8;

        // Put an initial value of 0xA into reg 0 to make sure it's not just move functionality later
        machine.state.registers[0] = 0xA;

        // Execute 5 clock cycles
        for _ in 0..6{
            machine.simulate_clock_pulse();
        }

        // Test if expectation matches reality
        assert_eq!(machine.state.current_instruction, 0x50_00_00_01);   // SUCCESS: Instruction loaded correctly
        assert_eq!(machine.state.micro_op_counter, 1);                  // SUCCESS: Reset successful
        assert_eq!(machine.state.registers[0], 0xB);                    // SUCCESS: Add successful
    }
}