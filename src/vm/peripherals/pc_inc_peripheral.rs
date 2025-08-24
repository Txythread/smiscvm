use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "PC_INC";

#[derive(Default)]
pub struct PCIncPeripheral {}


impl Peripheral for PCIncPeripheral {
    fn call(&self, _: String, _: &mut MachineState) { /* pass */ }

    fn late_call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        // Increment the program counter by one instruction, which is 4 bytes
        state.program_counter += 4;
    }
}