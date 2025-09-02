use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "PC_IN";

#[derive(Default)]
pub struct ProgramCounterInputEnablePeripheral {}

impl Peripheral for ProgramCounterInputEnablePeripheral {
    fn call(&self, _: String, _: &mut MachineState) {}
    fn late_call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        // The address that should be loaded into the program counter (ignoring the program counter not having the last 4 bits)
        let address = state.main_bus as u32;

        // The address dropping the last two bits
        let new_contents = address & 0xFF_FF_FF_FC;

        state.program_counter = new_contents;
    }
}