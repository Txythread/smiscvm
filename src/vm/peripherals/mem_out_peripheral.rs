use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "MEM_OUT";

#[derive(Default)]
pub struct MemOutPeripheral {}

impl Peripheral for MemOutPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        let address = state.memory_address_ptr;

        let data1 = state.memory[address as usize + 0];
        let data2 = state.memory[address as usize + 1];
        let data3 = state.memory[address as usize + 2];
        let data4 = state.memory[address as usize + 3];

        // Add the bytes so they form a dword
        let data: u32 = ((data1 as u32) << 24) | ((data2 as u32) << 16) | ((data3 as u32) << 8) | data4 as u32;

        state.push_to_main_bus(data);
    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}