use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "MEM_B_IN";

#[derive(Default)]
pub struct MemByteInPeripheral {}

impl Peripheral for MemByteInPeripheral {
    fn call(&self, _: String, _: &mut MachineState ) { /* pass */ }
    fn late_call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        let address = state.memory_address_ptr;

        state.memory[address as usize + 0] = (state.main_bus as u32 & 0x00_00_00_FF) as u8;
    }

}