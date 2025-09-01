use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "MEM_IN";

#[derive(Default)]
pub struct MemInPeripheral {}

impl Peripheral for MemInPeripheral {
    fn call(&self, _: String, _: &mut MachineState ) { /* pass */ }
    fn late_call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        let address = state.memory_address_ptr;

        state.memory[address as usize + 3] = ((state.main_bus & 0x00_00_00_FF) >> 0) as u8;
        state.memory[address as usize + 2] = ((state.main_bus & 0x00_00_FF_00) >> 8) as u8;
        state.memory[address as usize + 1] = ((state.main_bus & 0x00_FF_00F_00) >> 16) as u8;
        state.memory[address as usize + 0] = ((state.main_bus as u32 & 0xFF_00_00_00) >> 24) as u8;
    }

}