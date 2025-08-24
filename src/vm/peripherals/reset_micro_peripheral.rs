use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "RESET_MICRO";

#[derive(Default)]
pub struct ResetMicroPeripheral {}


impl Peripheral for ResetMicroPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        println!("Resetting micro op-counter");

        state.micro_op_counter = 0;
    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}