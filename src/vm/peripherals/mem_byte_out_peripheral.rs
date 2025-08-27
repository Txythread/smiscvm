use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "MEM_BYTE_OUT";

#[derive(Default)]
pub struct MemByteOutPeripheral {}

impl Peripheral for MemByteOutPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        let address = state.memory_address_ptr;

        let data = state.memory[address as usize];

        state.push_to_main_bus(data as u32);
    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}