use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "RESET_MICRO";

#[derive(Default)]
pub struct ResetMicroPeripheral {}


impl Peripheral for ResetMicroPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        state.micro_op_counter = 0;

        // Update the amount of executed instructions
        state.completed_instructions += 1;
    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}